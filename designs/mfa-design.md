# Design Proposal: TOTP Multi-Factor Authentication

Status: implemented (2026-08-16)
Author: assistant (for review by Feri)
Related env: `JWT_SECRET`, `JWT_REFRESH_SECRET` (existing), `MFA_ENC_KEY` (new)

## 1. Motivation

Admin login (`src/views/src/pages/admin/login.rs` → `AuthServiceImpl::login`) is a
single factor today: email + Argon2-verified password, straight to
access/refresh cookies. There's exactly one privileged account
(`client_category`-gated) and it's reachable from the public internet through
Nginx — a leaked or reused password is the only thing standing between the
outside world and `/admin`. TOTP (Google Authenticator, Authy, 1Password, …)
is the standard cheap second factor: no SMS provider, no new inbound
dependency, works offline on the user's phone.

## 2. Scope

**In scope:** per-user opt-in enrollment, TOTP challenge as a second login
step, one-time recovery codes for device loss.

**Out of scope (fast-follow candidates, not this round):** WebAuthn/passkeys,
org-wide enforcement, "remember this device for N days" skip, MFA secret
rotation tooling.

## 3. Data model changes

New migration, `src/schemas/migrations/<ts>_add_mfa_to_users.up.sql`:

```sql
ALTER TABLE users
    ADD COLUMN mfa_secret          TEXT    NOT NULL DEFAULT '',
    ADD COLUMN mfa_enabled         BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN mfa_recovery_codes  TEXT[]  NOT NULL DEFAULT '{}';
```

`.down.sql` drops the three columns.

- `mfa_secret` — the TOTP secret encrypted with AES-256-GCM under
  `MFA_ENC_KEY`, never stored raw. Decrypted only inside `AuthServiceImpl`,
  in-memory, for the duration of a verify call.
- `mfa_recovery_codes` — Argon2 hashes of ten one-time codes, same hashing
  already used for `users.password`. A code is removed from the array the
  moment it's consumed.

`UserView` (`modules/src/auth/models`) gains the three fields so
`find_user_by_email` / `find_user_by_id` can carry them without an extra
round-trip.

## 4. Architecture

### 4.1 Dependencies

```toml
# src/services/Cargo.toml
totp-rs = { version = "5", features = ["qr"] }
aes-gcm = "0.10"
```

`totp-rs`'s `qr` feature hands back a ready-to-use base64 PNG data URI —
no separate `qrcode`/`image` crate needed for the enrollment screen.
`aes-gcm` covers encrypting `mfa_secret` at rest.

### 4.2 Contracts (`modules/src/auth`)

`login` currently returns `Result<LoginResult>`. That becomes:

```rust
pub enum LoginOutcome {
    Authenticated(LoginResult),
    MfaRequired { challenge_token: String },
}

pub struct MfaEnrollmentView {
    pub secret_base32: String, // manual-entry fallback under the QR
    pub qr_data_uri: String,
}
```

`AuthService` additions/changes:

```rust
async fn login(&self, email: &str, password: &str, ip: &str) -> Result<LoginOutcome>; // was Result<LoginResult>
async fn verify_mfa(&self, challenge_token: &str, code: &str, ip: &str) -> Result<LoginResult>;
async fn enroll_mfa(&self, user_id: i32) -> Result<MfaEnrollmentView>;
async fn confirm_mfa(&self, user_id: i32, code: &str) -> Result<Vec<String>>; // plaintext recovery codes, shown once
async fn disable_mfa(&self, user_id: i32, code: &str) -> Result<()>;          // requires re-auth
```

`AuthRepository` additions:

```rust
async fn save_mfa_secret(&self, user_id: i32, encrypted_secret: &str) -> Result<()>;
async fn enable_mfa(&self, user_id: i32, recovery_code_hashes: Vec<String>) -> Result<()>;
async fn disable_mfa(&self, user_id: i32) -> Result<()>;
async fn consume_recovery_code(&self, user_id: i32, code_hash: &str) -> Result<bool>;
```

### 4.3 Service logic (`services/src/auth/auth_service.rs`)

**`login()`** — unchanged through the Argon2 check. After a valid password:

- `mfa_enabled == false` → same tail as today, `LoginOutcome::Authenticated(...)`.
- `mfa_enabled == true` → mint `challenge_token` as a short-lived, signed
  JWT (`sub: user_id`, `purpose: "mfa_challenge"`, 5-minute `exp`, signed
  with `jwt_secret` — same key/library as the access token, distinct claims
  shape), return `LoginOutcome::MfaRequired { challenge_token }`. **No
  cookies are set** at this point — the caller only holds a short-lived
  challenge, not a session.

    **Deviation from the original plan:** this was going to be a
    `CacheStore`-backed opaque token (`mfa-challenge:{uuid} → user_id`, same
    mechanism as the login-attempt lock), but that would make a Valkey outage
    a full MFA lockout — the one path in this codebase where the cache is
    explicitly documented as never a hard dependency for correctness (see
    `old/valkey-cache-design.md` §3.5). A self-contained signed token needs no
    storage to resolve, so it stays exactly as available as the JWT signing
    key already is. Per-user rate limiting on the _verify_ step (§ below)
    still uses `CacheStore` — that one fails open by design, same as the
    login-attempt lock, so a cache outage only loosens brute-force protection
    rather than blocking a legitimate login.

**`verify_mfa(challenge_token, code, ip)`**:

1. Resolve `challenge_token` → `user_id` from cache; missing/expired → error.
2. Rate-limit per user (`mfa-attempts:{user_id}`, same fixed-window pattern
   as `is_locked`/`record_login_failure`, separate constant from the
   password one so a TOTP brute-force doesn't share a budget with password
   guessing).
3. Decrypt `mfa_secret`, check the submitted code against the current TOTP
   window (`totp-rs` default: 30s step, ±1 step skew).
4. If the TOTP check fails, fall back to recovery codes: Argon2-verify
   `code` against each stored hash; on match, `consume_recovery_code` and
   proceed as authenticated.
5. On success: delete the challenge token, clear the attempt counter, issue
   access/refresh tokens exactly like the tail of today's `login()`.

**`enroll_mfa(user_id)`** — generate a random secret, build the
`otpauth://totp/...?issuer=feri-irawansyah` URL, render the QR data URI,
AES-GCM-encrypt the raw secret and persist via `save_mfa_secret`.
`mfa_enabled` stays `false` until confirmed — an abandoned enrollment never
locks anyone out.

**`confirm_mfa(user_id, code)`** — decrypt the pending secret, verify the
first code, and only then generate 10 recovery codes, Argon2-hash each,
`enable_mfa(...)`, and return the plaintext codes — the only moment they
ever exist outside the user's own notes.

Recovery code shape: 8 characters, uppercase alphanumeric, ambiguous
characters (`0`/`O`, `1`/`I`/`L`) excluded from the charset, rendered with a
dash after the 4th character for readability — e.g. `XCV4-7QRM`. Generated
from a CSPRNG (`rand::rngs::OsRng`), not `Uuid`, since the charset is custom.

**`disable_mfa(user_id, code)`** — requires a valid current TOTP or recovery
code before clearing `mfa_secret`/`mfa_recovery_codes`/`mfa_enabled`, so a
hijacked browser session alone can't turn MFA off.

### 4.4 Server wiring (`server/src/main.rs`)

`MFA_ENC_KEY` read from env next to `JWT_SECRET`, threaded into
`AuthServiceDeps`. Fails loudly at boot if missing or the wrong length
(32 bytes for AES-256) — same posture as `JWT_SECRET`, not a soft-degrade
like the Valkey connection.

### 4.5 Views

`views/src/pages/admin/login.rs`:

- `login_action` return type follows `LoginOutcome`. On `MfaRequired`, no
  cookies are set; the challenge token goes back to the client and
  `LoginPage` swaps to a second `<ActionForm>` asking for the 6-digit code.
- New `verify_mfa_action(challenge_token, code)` — sets the two cookies the
  same way `login_action`'s tail does today.

`views/src/pages/admin/users.rs`:

- "2FA" column per row (on/off badge).
- "Enroll" flow: show the QR + manual secret, one code-confirm input: on
  success, show the 10 recovery codes once in a panel that makes clear they
  won't be shown again.
- "Disable" flow: prompts for a current code before calling `disable_mfa`.

### 4.6 Middleware

No change to `server/src/auth/middleware.rs` — it only ever sees
access/refresh cookies. MFA is entirely a gate in front of _issuing_ those
cookies in the first place.

### 4.7 Re-auth cadence (no "remember this device")

No device-trust/skip-MFA feature. `refresh_token` already expires after 7
days (`Duration::days(7)` in `AuthServiceImpl::login`/`refresh`) and isn't
extended on refresh — once it's gone, the browser has no valid cookie left
and hits `login()` from scratch, which re-checks `mfa_enabled` and issues a
fresh `MfaRequired` challenge regardless of how recently the user last
solved one. So the existing 7-day session lifetime _is_ the re-auth cadence
— nothing new to build here, just confirming `verify_mfa` never gets
bypassed by the refresh path in `AuthMiddleware`.

## 5. Rollout plan

1. Migration + extend `UserView`/contracts — compiles, no behavior change.
2. Implement enroll/confirm/disable end to end, wire into `users.rs`.
   `mfa_enabled` exists per-account but `login()` doesn't consult it yet.
3. Implement the two-step `login()` / `verify_mfa()` + `LoginOutcome`,
   update `login.rs` UI.
4. `cargo sqlx prepare --workspace`. Manual pass: enroll on a test account,
   log out, log back in through the TOTP gate, burn a recovery code, trip
   the rate limit on purpose, disable MFA.
5. Document `MFA_ENC_KEY` generation (`openssl rand -base64 32`) in
   `.env`/README.

## 6. Decisions (resolved with Feri)

1. **Recovery code format** — 8-char uppercase alphanumeric, ambiguous
   characters excluded, dash after the 4th character (`XCV4-7QRM`). See §4.3.
2. **`MFA_ENC_KEY` rotation** — out of scope for v1; rotating the key
   invalidates every enrolled secret and forces re-enrollment. Accepted.
   In its place: re-auth (password + MFA) is already forced every 7 days by
   the existing `refresh_token` expiry, so there's no long-lived session to
   worry about rotating around. See §4.7.
3. **Session invalidation on enroll/disable** — other active sessions for
   the account are left alone; enrolling/disabling MFA doesn't force a
   logout elsewhere.
4. **"Remember this device"** — not built. Superseded by #2/§4.7: the
   7-day refresh-token lifetime already forces a full password+MFA login on
   a predictable cadence, so a separate device-trust mechanism would just
   duplicate that.
