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
        })
        .collect())
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
                                </tr>
                            </thead>
                            <tbody>
                                <For
                                    each=move || rows.get()
                                    key=|row| row.id
                                    children=move |row| {
                                        let role = row.role.clone();
                                        let status = row.status.clone();
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
            </div>
        </AdminLayout>
    }
}
