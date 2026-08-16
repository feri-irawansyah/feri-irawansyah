use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};

#[server]
pub async fn check_already_logged_in() -> Result<bool, ServerFnError> {
    use actix_web::{HttpMessage, HttpRequest};
    use leptos_actix::extract;
    use modules::auth::Claims;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if req.extensions().get::<Claims>().is_some() {
        leptos_actix::redirect("/admin");
        return Ok(true);
    }

    Ok(false)
}

/// What the client needs to decide which form to show next. Mirrors
/// `modules::auth::LoginOutcome` but stays local to the view layer — the
/// wire shape a `#[server]` fn returns doesn't need to be the same type the
/// service layer uses internally.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LoginStep {
    LoggedIn,
    MfaRequired { challenge_token: String },
}

/// Shared tail of `login_action` (no MFA / already past it) and
/// `verify_mfa_action` — sets the two session cookies from a
/// `LoginResult`. Only compiled server-side (both callers are `#[server]`
/// bodies), but defined at module scope since it's identical either way.
#[cfg(feature = "ssr")]
fn set_session_cookies(
    result: &modules::auth::LoginResult,
    secure: &str,
) -> Result<(), ServerFnError> {
    use actix_web::http::header::{HeaderValue, SET_COOKIE};
    use leptos_actix::ResponseOptions;

    let response = use_context::<ResponseOptions>()
        .ok_or_else(|| ServerFnError::new("No response context"))?;

    response.append_header(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "access_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=900{secure}",
            result.access_token
        ))
        .unwrap(),
    );
    response.append_header(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800{secure}",
            result.refresh_token
        ))
        .unwrap(),
    );

    Ok(())
}

#[cfg(feature = "ssr")]
async fn is_prod() -> Result<bool, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;

    let leptos_options = extract::<Data<leptos::config::LeptosOptions>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(leptos_options.env == leptos::config::Env::PROD)
}

#[server]
pub async fn login_action(email: String, password: String) -> Result<LoginStep, ServerFnError> {
    use actix_web::{HttpRequest, web::Data};
    use leptos_actix::extract;
    use modules::auth::LoginOutcome;
    use std::sync::Arc;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let auth_svc = extract::<Data<Arc<dyn modules::auth::AuthService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    // Secure requires HTTPS — only set it in production (behind Nginx/TLS),
    // otherwise the cookie would be silently dropped during local `cargo
    // leptos watch` dev (plain http://127.0.0.1:3000).
    let secure = if is_prod().await? { "; Secure" } else { "" };

    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let outcome = auth_svc
        .login(&email, &password, &ip)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match outcome {
        // No cookies yet — the caller only has a password-verified
        // identity, not a session. See modules::auth::LoginOutcome.
        LoginOutcome::MfaRequired { challenge_token } => {
            Ok(LoginStep::MfaRequired { challenge_token })
        }
        LoginOutcome::Authenticated(result) => {
            set_session_cookies(&result, secure)?;
            Ok(LoginStep::LoggedIn)
        }
    }
}

#[server]
pub async fn verify_mfa_action(challenge_token: String, code: String) -> Result<(), ServerFnError> {
    use actix_web::{HttpRequest, web::Data};
    use leptos_actix::extract;
    use std::sync::Arc;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let auth_svc = extract::<Data<Arc<dyn modules::auth::AuthService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let secure = if is_prod().await? { "; Secure" } else { "" };

    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let result = auth_svc
        .verify_mfa(&challenge_token, &code, &ip)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    set_session_cookies(&result, secure)
}

#[allow(non_snake_case)]
#[component]
pub fn LoginPage() -> impl IntoView {
    let already_logged_in = Resource::new_blocking(|| (), |_| check_already_logged_in());
    let login = ServerAction::<LoginAction>::new();
    let verify_mfa = ServerAction::<VerifyMfaAction>::new();
    let navigate = use_navigate();

    // Set once login_action comes back with MfaRequired — switches the view
    // from the password form to the TOTP form. `None` = still on step 1.
    let challenge_token = RwSignal::new(None::<String>);

    // SSR handled inside check_already_logged_in via leptos_actix::redirect.
    // This Effect handles client-side navigation (e.g. navigating to /admin/login via router).
    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if already_logged_in.get().is_some_and(|r| r.unwrap_or(false)) {
            navigate_clone("/admin", NavigateOptions::default());
        }
    });

    let navigate_after_login = navigate.clone();
    Effect::new(move |_| {
        login.value().with(|v| {
            if let Some(Ok(step)) = v {
                match step {
                    LoginStep::LoggedIn => {
                        navigate_after_login("/admin", NavigateOptions::default());
                    }
                    LoginStep::MfaRequired {
                        challenge_token: token,
                    } => {
                        challenge_token.set(Some(token.clone()));
                    }
                }
            }
        });
    });

    Effect::new(move |_| {
        if verify_mfa
            .value()
            .with(|v| v.as_ref().map(|r| r.is_ok()).unwrap_or(false))
        {
            navigate("/admin", NavigateOptions::default());
        }
    });

    let login_error_msg = move || {
        login.value().with(|v| {
            v.as_ref()
                .and_then(|r| r.as_ref().err())
                .map(|e| e.to_string())
        })
    };

    let mfa_error_msg = move || {
        verify_mfa.value().with(|v| {
            v.as_ref()
                .and_then(|r| r.as_ref().err())
                .map(|e| e.to_string())
        })
    };

    view! {
        <Suspense fallback=|| ()>
            {move || {
                // Saat SSR redirect sudah di-set, render kosong agar tidak flash form
                if already_logged_in.get().is_some_and(|r| r.unwrap_or(false)) {
                    return view! { <div></div> }.into_any();
                }
                view! {
                    <div class="min-h-screen flex items-center justify-center bg-base">
                        <div class="w-full max-w-sm px-8 py-10 bg-surface border border-line rounded-xl shadow-sm">
                            {move || {
                                if let Some(token) = challenge_token.get() {
                                    view! {
                                        <h1 class="text-2xl font-extrabold mb-1 text-fg">"Verifikasi 2FA"</h1>
                                        <p class="text-muted text-sm mb-8">"Masukkan kode dari aplikasi authenticator, atau salah satu recovery code"</p>

                                        <ActionForm action=verify_mfa>
                                            <div class="flex flex-col gap-4">
                                                <input type="hidden" name="challenge_token" value=token />
                                                <div>
                                                    <label r#for="mfa-code" class="block text-sm font-medium mb-1.5 text-fg">"Kode"</label>
                                                    <input
                                                        id="mfa-code"
                                                        type="text"
                                                        name="code"
                                                        required
                                                        autocomplete="one-time-code"
                                                        inputmode="text"
                                                        class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm tracking-widest"
                                                        placeholder="123456"
                                                    />
                                                </div>

                                                {move || mfa_error_msg().map(|e| view! {
                                                    <p class="text-red-400 text-sm py-1">{e}</p>
                                                })}

                                                <button
                                                    type="submit"
                                                    class="w-full py-2.5 mt-1 bg-teal-600 hover:bg-teal-500 text-white font-semibold rounded-lg transition-colors text-sm cursor-pointer disabled:opacity-60"
                                                    disabled=move || verify_mfa.pending().get()>
                                                    {move || if verify_mfa.pending().get() { "Memverifikasi..." } else { "Verifikasi" }}
                                                </button>

                                                <button
                                                    type="button"
                                                    class="text-muted text-sm hover:text-fg transition-colors cursor-pointer"
                                                    on:click=move |_| challenge_token.set(None)>
                                                    "← Kembali ke login"
                                                </button>
                                            </div>
                                        </ActionForm>
                                    }.into_any()
                                } else {
                                    view! {
                                        <h1 class="text-2xl font-extrabold mb-1 text-fg">"Admin"</h1>
                                        <p class="text-muted text-sm mb-8">"Masuk ke panel admin"</p>

                                        <ActionForm action=login>
                                            <div class="flex flex-col gap-4">
                                                <div>
                                                    <label r#for="login-email" class="block text-sm font-medium mb-1.5 text-fg">"Email"</label>
                                                    <input
                                                        id="login-email"
                                                        type="email"
                                                        name="email"
                                                        required
                                                        autocomplete="email"
                                                        class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                                        placeholder="admin@example.com"
                                                    />
                                                </div>
                                                <div>
                                                    <label r#for="login-password" class="block text-sm font-medium mb-1.5 text-fg">"Password"</label>
                                                    <input
                                                        id="login-password"
                                                        type="password"
                                                        name="password"
                                                        required
                                                        autocomplete="current-password"
                                                        class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                                        placeholder="••••••••"
                                                    />
                                                </div>

                                                {move || login_error_msg().map(|e| view! {
                                                    <p class="text-red-400 text-sm py-1">{e}</p>
                                                })}

                                                <button
                                                    type="submit"
                                                    class="w-full py-2.5 mt-1 bg-teal-600 hover:bg-teal-500 text-white font-semibold rounded-lg transition-colors text-sm cursor-pointer disabled:opacity-60"
                                                    disabled=move || login.pending().get()>
                                                    {move || if login.pending().get() { "Masuk..." } else { "Masuk" }}
                                                </button>
                                            </div>
                                        </ActionForm>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                }.into_any()
            }}
        </Suspense>
    }
}
