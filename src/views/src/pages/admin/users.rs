use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use super::layout::AdminLayout;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct UserTableRow {
    pub id: i32,
    pub email: String,
    pub fullname: String,
    pub role: String,
    pub status: String,
    pub mfa_enabled: bool,
}

#[server]
pub async fn get_users(offset: i64) -> Result<Vec<UserTableRow>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;

    crate::pages::admin::require_admin().await?;

    let user_svc = extract::<Data<Arc<dyn modules::users::UserService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let users = user_svc
        .find_all_async(20, offset)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users
        .into_iter()
        .map(|u| UserTableRow {
            id: u.id,
            email: u.email,
            fullname: u.fullname,
            role: if u.client_category == 1 {
                "Admin".into()
            } else {
                "Visitor".into()
            },
            status: if u.disable_login {
                "Disabled".into()
            } else {
                "Active".into()
            },
            mfa_enabled: u.mfa_enabled == Some(true),
        })
        .collect())
}

/// Who's viewing the page — used to only ever show the 2FA management panel
/// for the caller's own row. Enrolling/disabling MFA always acts on
/// `require_admin()`'s own `Claims.sub`, never a client-supplied id (see
/// `enroll_mfa_action`/`confirm_mfa_action`/`disable_mfa_action`), so this
/// is purely a UI affordance, not the security boundary.
#[server]
pub async fn whoami() -> Result<i32, ServerFnError> {
    let claims = crate::pages::admin::require_admin().await?;
    Ok(claims.sub)
}

#[server]
pub async fn enroll_mfa_action() -> Result<modules::auth::MfaEnrollmentView, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;

    let claims = crate::pages::admin::require_admin().await?;
    let auth_svc = extract::<Data<Arc<dyn modules::auth::AuthService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    auth_svc
        .enroll_mfa(claims.sub)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn confirm_mfa_action(code: String) -> Result<Vec<String>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;

    let claims = crate::pages::admin::require_admin().await?;
    let auth_svc = extract::<Data<Arc<dyn modules::auth::AuthService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    auth_svc
        .confirm_mfa(claims.sub, &code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn disable_mfa_action(code: String) -> Result<(), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;

    let claims = crate::pages::admin::require_admin().await?;
    let auth_svc = extract::<Data<Arc<dyn modules::auth::AuthService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    auth_svc
        .disable_mfa(claims.sub, &code)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Local state machine for the "your account security" panel. Nothing here
/// is persisted client-side across a reload — every step re-derives from
/// what the server just returned.
#[derive(Clone, PartialEq)]
enum MfaPanel {
    Closed,
    /// QR + manual secret shown, waiting for the first code to confirm.
    Confirming(modules::auth::MfaEnrollmentView),
    /// Confirmed — recovery codes shown exactly once.
    RecoveryCodes(Vec<String>),
    /// Already enabled — asking for a code to turn it back off.
    Disabling,
}

#[allow(non_snake_case)]
#[component]
pub fn UsersPage() -> impl IntoView {
    let rows: RwSignal<Vec<UserTableRow>> = RwSignal::new(vec![]);
    let offset: RwSignal<i64> = RwSignal::new(0);
    let has_more: RwSignal<bool> = RwSignal::new(true);
    let is_loading: RwSignal<bool> = RwSignal::new(true);

    // LocalResource: client-only, never SSR — avoids hydration mismatch on admin pages
    let load_more = LocalResource::new(move || {
        let off = offset.get();
        get_users(off)
    });
    let own_id = LocalResource::new(whoami);

    // Append each batch and manage loading state via dedicated signal
    Effect::new(move |_| match load_more.get() {
        Some(Ok(new_rows)) => {
            if new_rows.len() < 20 {
                has_more.set(false);
            }
            rows.update(|r| r.extend(new_rows));
            is_loading.set(false);
        }
        Some(Err(_)) => {
            is_loading.set(false);
        }
        None => {
            is_loading.set(true);
        }
    });

    // Infinite scroll: window scroll listener, only compiled/runs in WASM
    #[cfg(target_arch = "wasm32")]
    {
        // WASM is single-threaded — wrapping non-Send JS types to satisfy Leptos's
        // on_cleanup bound (which requires Send + Sync for SSR/hydrate compatibility).
        struct SendSync<T>(T);
        unsafe impl<T> Send for SendSync<T> {}
        unsafe impl<T> Sync for SendSync<T> {}

        let offset_s = offset;
        let has_more_s = has_more;
        let is_loading_s = is_loading;

        Effect::new(move |_| {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let Some(win) = leptos::web_sys::window() else {
                return;
            };

            let handler = Closure::wrap(Box::new(move || {
                let Some(win) = leptos::web_sys::window() else {
                    return;
                };
                let Some(doc) = win.document() else {
                    return;
                };
                let Some(doc_el) = doc.document_element() else {
                    return;
                };

                let at_bottom = doc_el.scroll_top() + doc_el.client_height()
                    >= doc_el.scroll_height() - 150;

                if at_bottom && has_more_s.get_untracked() && !is_loading_s.get_untracked() {
                    offset_s.update(|o| *o += 20);
                }
            }) as Box<dyn Fn()>);

            let fn_ref = SendSync(
                handler.as_ref().unchecked_ref::<leptos::web_sys::js_sys::Function>().clone()
            );
            let _ = win.add_event_listener_with_callback("scroll", &fn_ref.0);

            let win_clone = SendSync(win.clone());
            let handler = SendSync(handler);
            on_cleanup(move || {
                let _ = win_clone.0.remove_event_listener_with_callback("scroll", &fn_ref.0);
                drop(handler);
            });
        });
    }

    // ── Own-account MFA panel state ─────────────────────────────────────
    let panel: RwSignal<MfaPanel> = RwSignal::new(MfaPanel::Closed);
    let panel_error: RwSignal<Option<String>> = RwSignal::new(None);
    let code_input: RwSignal<String> = RwSignal::new(String::new());

    let start_enroll = Action::new(|_: &()| async move { enroll_mfa_action().await });
    let confirm = Action::new(|code: &String| {
        let code = code.clone();
        async move { confirm_mfa_action(code).await }
    });
    let disable = Action::new(|code: &String| {
        let code = code.clone();
        async move { disable_mfa_action(code).await }
    });

    Effect::new(move |_| {
        if let Some(result) = start_enroll.value().get() {
            match result {
                Ok(enrollment) => {
                    panel_error.set(None);
                    code_input.set(String::new());
                    panel.set(MfaPanel::Confirming(enrollment));
                }
                Err(e) => panel_error.set(Some(e.to_string())),
            }
        }
    });

    Effect::new(move |_| {
        if let Some(result) = confirm.value().get() {
            match result {
                Ok(recovery_codes) => {
                    panel_error.set(None);
                    rows.update(|rs| {
                        if let Some(id) = own_id.get().and_then(|r| r.ok())
                            && let Some(row) = rs.iter_mut().find(|r| r.id == id) {
                                row.mfa_enabled = true;
                            }
                    });
                    panel.set(MfaPanel::RecoveryCodes(recovery_codes));
                }
                Err(e) => panel_error.set(Some(e.to_string())),
            }
        }
    });

    Effect::new(move |_| {
        if let Some(result) = disable.value().get() {
            match result {
                Ok(()) => {
                    panel_error.set(None);
                    rows.update(|rs| {
                        if let Some(id) = own_id.get().and_then(|r| r.ok())
                            && let Some(row) = rs.iter_mut().find(|r| r.id == id) {
                                row.mfa_enabled = false;
                            }
                    });
                    panel.set(MfaPanel::Closed);
                }
                Err(e) => panel_error.set(Some(e.to_string())),
            }
        }
    });

    let close_panel = move || {
        panel.set(MfaPanel::Closed);
        panel_error.set(None);
        code_input.set(String::new());
    };

    view! {
        <AdminLayout>
            <div class="p-8 max-w-6xl">

                // Section header
                <div class="mb-8">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        "Manajemen"
                    </span>
                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Users"</h2>
                    <p class="text-muted text-sm">"Semua pengguna terdaftar — admin & visitor"</p>
                </div>

                // Table card
                <div class="bg-surface border border-line rounded-2xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full text-sm text-fg">
                            <thead>
                                <tr class="border-b border-line">
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-16">"ID"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Email"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Nama Lengkap"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Role"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Status"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"2FA"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-40"></th>
                                </tr>
                            </thead>
                            <tbody>
                                <For
                                    each=move || rows.get()
                                    key=|row| (row.id, row.mfa_enabled)
                                    children=move |row| {
                                        let role = row.role.clone();
                                        let status = row.status.clone();
                                        let row_id = row.id;
                                        let mfa_enabled = row.mfa_enabled;
                                        let role_class = if role == "Admin" {
                                            "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-teal-500/15 text-teal-400"
                                        } else {
                                            "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-line text-muted"
                                        };
                                        let status_class = if status == "Active" {
                                            "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-green-500/15 text-green-400"
                                        } else {
                                            "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-red-500/15 text-red-400"
                                        };
                                        let dot_class = if status == "Active" {
                                            "w-1.5 h-1.5 rounded-full bg-green-400"
                                        } else {
                                            "w-1.5 h-1.5 rounded-full bg-red-400"
                                        };
                                        let mfa_class = if mfa_enabled {
                                            "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-teal-500/15 text-teal-400"
                                        } else {
                                            "inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-line text-muted"
                                        };
                                        view! {
                                            <tr class="border-b border-line last:border-0 hover:bg-teal-500/5 transition-colors">
                                                <td class="px-5 py-3.5 text-muted font-mono text-xs">{row.id}</td>
                                                <td class="px-5 py-3.5 text-fg">{row.email}</td>
                                                <td class="px-5 py-3.5 text-fg font-medium">{row.fullname}</td>
                                                <td class="px-5 py-3.5">
                                                    <span class=role_class>{role}</span>
                                                </td>
                                                <td class="px-5 py-3.5">
                                                    <span class=status_class>
                                                        <span class=dot_class></span>
                                                        {status}
                                                    </span>
                                                </td>
                                                <td class="px-5 py-3.5">
                                                    <span class=mfa_class>
                                                        {if mfa_enabled { "Aktif" } else { "Nonaktif" }}
                                                    </span>
                                                </td>
                                                <td class="px-5 py-3.5 text-right">
                                                    {move || {
                                                        if own_id.get().and_then(|r| r.ok()) != Some(row_id) {
                                                            return view! { <span></span> }.into_any();
                                                        }
                                                        if mfa_enabled {
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="text-xs font-semibold text-red-400 hover:text-red-300 transition-colors cursor-pointer"
                                                                    on:click=move |_| {
                                                                        panel_error.set(None);
                                                                        code_input.set(String::new());
                                                                        panel.set(MfaPanel::Disabling);
                                                                    }>
                                                                    "Nonaktifkan 2FA"
                                                                </button>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="text-xs font-semibold text-teal-500 hover:text-teal-400 transition-colors cursor-pointer disabled:opacity-60"
                                                                    disabled=move || start_enroll.pending().get()
                                                                    on:click=move |_| {
                                                                        panel_error.set(None);
                                                                        start_enroll.dispatch(());
                                                                    }>
                                                                    "Aktifkan 2FA"
                                                                </button>
                                                            }.into_any()
                                                        }
                                                    }}
                                                </td>
                                            </tr>
                                        }
                                    }
                                />
                            </tbody>
                        </table>
                    </div>

                    // Footer
                    <div class="px-5 py-4 border-t border-line">
                        {move || {
                            if is_loading.get() {
                                view! {
                                    <div class="flex items-center gap-2 text-muted text-xs">
                                        <i class="bi bi-arrow-repeat animate-spin"></i>
                                        "Memuat data..."
                                    </div>
                                }.into_any()
                            } else if rows.with(|r| r.is_empty()) {
                                view! {
                                    <p class="text-muted text-xs">"Belum ada user."</p>
                                }.into_any()
                            } else if !has_more.get() {
                                view! {
                                    <p class="text-muted text-xs">
                                        "Semua data dimuat · "
                                        <span class="font-semibold text-teal-500">
                                            {rows.with(|r| r.len())}
                                            " user"
                                        </span>
                                    </p>
                                }.into_any()
                            } else {
                                view! {
                                    <p class="text-muted text-xs">"Scroll untuk muat lebih banyak..."</p>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>

                // ── MFA panel (own account only) ───────────────────────
                {move || match panel.get() {
                    MfaPanel::Closed => view! { <div></div> }.into_any(),
                    MfaPanel::Confirming(enrollment) => {
                        let qr = enrollment.qr_data_uri.clone();
                        let secret = enrollment.secret_base32.clone();
                        view! {
                            <div class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
                                <div class="w-full max-w-sm bg-surface border border-line rounded-xl p-6">
                                    <h3 class="text-lg font-bold text-fg mb-1">"Scan QR ini"</h3>
                                    <p class="text-muted text-xs mb-4">"Pakai Google Authenticator, Authy, atau aplikasi TOTP lainnya"</p>
                                    <img src=qr alt="QR code untuk enrollment MFA" class="w-48 h-48 mx-auto mb-3 rounded-lg bg-white p-2" />
                                    <p class="text-muted text-xs text-center mb-4">
                                        "Atau masukkan manual: "
                                        <span class="font-mono text-fg">{secret}</span>
                                    </p>
                                    <label r#for="mfa-confirm-code" class="block text-sm font-medium mb-1.5 text-fg">"Kode dari aplikasi"</label>
                                    <input
                                        id="mfa-confirm-code"
                                        type="text"
                                        inputmode="numeric"
                                        autocomplete="one-time-code"
                                        class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm tracking-widest mb-3"
                                        placeholder="123456"
                                        prop:value=move || code_input.get()
                                        on:input=move |ev| code_input.set(event_target_value(&ev))
                                    />
                                    {move || panel_error.get().map(|e| view! {
                                        <p class="text-red-400 text-sm mb-3">{e}</p>
                                    })}
                                    <div class="flex gap-2">
                                        <button
                                            type="button"
                                            class="flex-1 py-2.5 bg-teal-600 hover:bg-teal-500 text-white font-semibold rounded-lg transition-colors text-sm cursor-pointer disabled:opacity-60"
                                            disabled=move || confirm.pending().get()
                                            on:click=move |_| { confirm.dispatch(code_input.get_untracked()); }>
                                            {move || if confirm.pending().get() { "Memverifikasi..." } else { "Konfirmasi" }}
                                        </button>
                                        <button
                                            type="button"
                                            class="px-4 py-2.5 text-muted hover:text-fg text-sm transition-colors cursor-pointer"
                                            on:click=move |_| close_panel()>
                                            "Batal"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                    MfaPanel::RecoveryCodes(codes) => {
                        view! {
                            <div class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
                                <div class="w-full max-w-sm bg-surface border border-line rounded-xl p-6">
                                    <h3 class="text-lg font-bold text-fg mb-1">"2FA aktif — simpan recovery code ini"</h3>
                                    <p class="text-muted text-xs mb-4">
                                        "Tiap kode cuma bisa dipakai sekali, buat jaga-jaga kalau HP hilang. Ini satu-satunya kesempatan lihat kode ini."
                                    </p>
                                    <div class="grid grid-cols-2 gap-2 font-mono text-sm text-fg bg-base border border-line rounded-lg p-3 mb-4">
                                        {codes.into_iter().map(|c| view! { <span>{c}</span> }).collect_view()}
                                    </div>
                                    <button
                                        type="button"
                                        class="w-full py-2.5 bg-teal-600 hover:bg-teal-500 text-white font-semibold rounded-lg transition-colors text-sm cursor-pointer"
                                        on:click=move |_| close_panel()>
                                        "Sudah disimpan"
                                    </button>
                                </div>
                            </div>
                        }.into_any()
                    }
                    MfaPanel::Disabling => {
                        view! {
                            <div class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
                                <div class="w-full max-w-sm bg-surface border border-line rounded-xl p-6">
                                    <h3 class="text-lg font-bold text-fg mb-1">"Nonaktifkan 2FA"</h3>
                                    <p class="text-muted text-xs mb-4">"Masukkan kode dari authenticator, atau salah satu recovery code, untuk konfirmasi."</p>
                                    <input
                                        type="text"
                                        autocomplete="one-time-code"
                                        class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm tracking-widest mb-3"
                                        placeholder="123456"
                                        prop:value=move || code_input.get()
                                        on:input=move |ev| code_input.set(event_target_value(&ev))
                                    />
                                    {move || panel_error.get().map(|e| view! {
                                        <p class="text-red-400 text-sm mb-3">{e}</p>
                                    })}
                                    <div class="flex gap-2">
                                        <button
                                            type="button"
                                            class="flex-1 py-2.5 bg-red-600 hover:bg-red-500 text-white font-semibold rounded-lg transition-colors text-sm cursor-pointer disabled:opacity-60"
                                            disabled=move || disable.pending().get()
                                            on:click=move |_| { disable.dispatch(code_input.get_untracked()); }>
                                            {move || if disable.pending().get() { "Menonaktifkan..." } else { "Nonaktifkan" }}
                                        </button>
                                        <button
                                            type="button"
                                            class="px-4 py-2.5 text-muted hover:text-fg text-sm transition-colors cursor-pointer"
                                            on:click=move |_| close_panel()>
                                            "Batal"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </AdminLayout>
    }
}
