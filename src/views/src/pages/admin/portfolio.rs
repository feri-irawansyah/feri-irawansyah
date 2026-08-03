use leptos::prelude::*;
use modules::portfolio::PortfolioView;
use modules::skills::SkillView;

use super::layout::{pagination_footer, AdminLayout};

#[cfg(feature = "ssr")]
use modules::Validate;

// ── Server functions (admin-guarded) ─────────────────────────────────────────

#[cfg(feature = "ssr")]
async fn portfolio_svc()
-> Result<std::sync::Arc<dyn modules::portfolio::PortfolioService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::portfolio::PortfolioService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn admin_list_portfolio() -> Result<Vec<PortfolioView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    portfolio_svc()
        .await?
        .list()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Paginated project listing for the admin table — real LIMIT/OFFSET in SQL.
#[server]
pub async fn admin_list_portfolio_page(
    page: i64,
) -> Result<(Vec<PortfolioView>, i64), ServerFnError> {
    crate::pages::admin::require_admin().await?;
    portfolio_svc()
        .await?
        .list_page(page, 10)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_create_portfolio(
    input: modules::portfolio::PortfolioCommand,
) -> Result<PortfolioView, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    input.validate().map_err(|e| ServerFnError::new(e.to_string()))?;
    portfolio_svc()
        .await?
        .create(input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_update_portfolio(
    id: i32,
    input: modules::portfolio::PortfolioCommand,
) -> Result<Option<PortfolioView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    input.validate().map_err(|e| ServerFnError::new(e.to_string()))?;
    portfolio_svc()
        .await?
        .update(id, input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_delete_portfolio(id: i32) -> Result<bool, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    portfolio_svc()
        .await?
        .delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Page ──────────────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
pub fn AdminPortfolioPage() -> impl IntoView {
    const PAGE_SIZE: i64 = 10;
    let page = RwSignal::new(1i64);
    let refetch = RwSignal::new(0u32);
    let items_resource = LocalResource::new(move || {
        refetch.get();
        admin_list_portfolio_page(page.get())
    });
    let skills_resource = LocalResource::new(crate::pages::skills::get_all_skills);

    let show_form = RwSignal::new(false);
    let editing_id = RwSignal::new(None::<i32>);
    let f_title = RwSignal::new(String::new());
    let f_slug = RwSignal::new(String::new());
    let f_description = RwSignal::new(String::new());
    let f_url_docs = RwSignal::new(String::new());
    let f_image = RwSignal::new(String::new());
    let f_tech = RwSignal::new(Vec::<i32>::new());
    let f_pined = RwSignal::new(false);
    let f_sort_order = RwSignal::new(String::new());
    let f_details = RwSignal::new(String::new());
    let saving = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);
    let uploading = RwSignal::new(false);
    let upload_error = RwSignal::new(None::<String>);

    let on_file_change = move |_ev: leptos::ev::Event| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::{FormData, HtmlInputElement};

            #[derive(serde::Deserialize)]
            struct UploadResponse {
                url: Option<String>,
                error: Option<String>,
            }

            let Some(input) = _ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) else {
                return;
            };
            let Some(files) = input.files() else { return };
            let Some(file) = files.get(0) else { return };

            let Ok(form_data) = FormData::new() else { return };
            let _ = form_data.append_with_str("folder", "portfolio");
            let _ = form_data.append_with_blob_and_filename("file", &file, &file.name());

            uploading.set(true);
            upload_error.set(None);

            leptos::task::spawn_local(async move {
                let outcome = async {
                    let resp = gloo_net::http::Request::post("/api/admin/uploads")
                        .body(form_data)
                        .map_err(|e| e.to_string())?
                        .send()
                        .await
                        .map_err(|e| e.to_string())?;
                    let parsed: UploadResponse = resp.json().await.map_err(|e| e.to_string())?;
                    if let Some(url) = parsed.url {
                        Ok(url)
                    } else {
                        Err(parsed.error.unwrap_or_else(|| "Upload gagal".to_string()))
                    }
                }
                .await;

                match outcome {
                    Ok(url) => f_image.set(url),
                    Err(e) => upload_error.set(Some(e)),
                }
                uploading.set(false);
            });
        }
    };

    let reset_form = move || {
        editing_id.set(None);
        f_title.set(String::new());
        f_slug.set(String::new());
        f_description.set(String::new());
        f_url_docs.set(String::new());
        f_image.set(String::new());
        f_tech.set(Vec::new());
        f_pined.set(false);
        f_sort_order.set("0".to_string());
        f_details.set(String::new());
        error.set(None);
        upload_error.set(None);
    };

    let open_create = move |_| {
        reset_form();
        show_form.set(true);
    };

    let open_edit = move |row: PortfolioView| {
        editing_id.set(Some(row.portfolio_id));
        f_title.set(row.title);
        f_slug.set(row.slug);
        f_description.set(row.description);
        f_url_docs.set(row.url_docs);
        f_image.set(row.image_src);
        f_tech.set(row.tech);
        f_pined.set(row.pined);
        f_sort_order.set(row.sort_order.to_string());
        f_details.set(row.details);
        error.set(None);
        show_form.set(true);
    };

    let toggle_tech = move |skill_id: i32| {
        f_tech.update(|v| {
            if let Some(pos) = v.iter().position(|&id| id == skill_id) {
                v.remove(pos);
            } else {
                v.push(skill_id);
            }
        });
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(None);

        let sort_order = f_sort_order.get().parse::<i32>().unwrap_or(0);
        let input = modules::portfolio::PortfolioCommand {
            title: f_title.get(),
            slug: f_slug.get(),
            description: f_description.get(),
            url_docs: f_url_docs.get(),
            image_src: f_image.get(),
            tech: f_tech.get(),
            pined: f_pined.get(),
            sort_order,
            details: f_details.get(),
        };
        let id = editing_id.get();

        saving.set(true);
        leptos::task::spawn_local(async move {
            let result = match id {
                Some(id) => admin_update_portfolio(id, input).await.map(|_| ()),
                None => admin_create_portfolio(input).await.map(|_| ()),
            };
            match result {
                Ok(()) => {
                    saving.set(false);
                    show_form.set(false);
                    refetch.update(|n| *n += 1);
                }
                Err(e) => {
                    saving.set(false);
                    error.set(Some(e.to_string()));
                }
            }
        });
    };

    let on_delete = move |id: i32| {
        #[cfg(target_arch = "wasm32")]
        {
            if !leptos::web_sys::window()
                .and_then(|w| w.confirm_with_message("Hapus project ini?").ok())
                .unwrap_or(false)
            {
                return;
            }
        }
        leptos::task::spawn_local(async move {
            if admin_delete_portfolio(id).await.is_ok() {
                refetch.update(|n| *n += 1);
            }
        });
    };

    view! {
        <AdminLayout>
            <div class="p-8 max-w-6xl">
                <div class="mb-8">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        "Manajemen"
                    </span>
                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Portfolio"</h2>
                    <p class="text-muted text-sm">"Kelola project yang ditampilkan di halaman Portfolio."</p>
                </div>

                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-bold text-fg">"Daftar Project"</h3>
                    <button
                        on:click=open_create
                        class="inline-flex items-center gap-1.5 px-3.5 py-2 bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold rounded-lg transition-colors cursor-pointer">
                        <i class="bi bi-plus-lg"></i>
                        "Tambah Project"
                    </button>
                </div>

                {move || show_form.get().then(|| view! {
                    <form on:submit=on_submit class="bg-surface border border-line rounded-2xl p-5 mb-5">
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Title"</label>
                                <input required prop:value=f_title on:input=move |e| f_title.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="Trash App"/>
                            </div>
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Slug"</label>
                                <input required prop:value=f_slug on:input=move |e| f_slug.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="trash-app"/>
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Description (teaser singkat)"</label>
                                <textarea rows="2" prop:value=f_description on:input=move |e| f_description.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"></textarea>
                            </div>
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"URL Docs / Live Link"</label>
                                <input prop:value=f_url_docs on:input=move |e| f_url_docs.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="https://..."/>
                            </div>
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Sort Order"</label>
                                <input type="number" prop:value=f_sort_order on:input=move |e| f_sort_order.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="0"/>
                            </div>

                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Screenshot"</label>
                                <div class="flex items-center gap-3">
                                    {move || (!f_image.get().is_empty()).then(|| view! {
                                        <div class="w-16 h-11 rounded-lg bg-white border border-line flex items-center justify-center shrink-0 overflow-hidden">
                                            <img src=f_image.get() alt="Preview" class="w-full h-full object-cover"/>
                                        </div>
                                    })}
                                    <input type="file" accept="image/*" on:change=on_file_change
                                        class="flex-1 text-sm text-muted file:mr-3 file:px-3 file:py-1.5 file:rounded-lg file:border-0 file:bg-teal-600 file:text-white file:text-xs file:font-semibold file:cursor-pointer hover:file:bg-teal-500 cursor-pointer"/>
                                </div>
                                {move || uploading.get().then(|| view! {
                                    <p class="text-xs text-muted mt-1.5"><i class="bi bi-arrow-repeat animate-spin mr-1"></i>"Mengunggah ke Supabase..."</p>
                                })}
                                {move || upload_error.get().map(|e| view! {
                                    <p class="text-xs text-red-400 mt-1.5">{e}</p>
                                })}
                            </div>

                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Tech Stack"</label>
                                <div class="flex flex-wrap gap-2">
                                    {move || skills_resource.get().and_then(|r| r.ok()).unwrap_or_default().into_iter().map(|s: SkillView| {
                                        let id = s.skill_id;
                                        let title = s.title.clone();
                                        let active = move || f_tech.get().contains(&id);
                                        view! {
                                            <button
                                                type="button"
                                                on:click=move |_| toggle_tech(id)
                                                class="px-2.5 py-1 rounded-full text-xs font-medium border transition-colors cursor-pointer"
                                                class=("bg-teal-500/15", active)
                                                class=("border-teal-500", active)
                                                class=("text-teal-500", active)
                                                class=("border-line", move || !active())
                                                class=("text-muted", move || !active())>
                                                {title}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            <div class="flex items-center gap-2">
                                <input type="checkbox" id="pined" prop:checked=f_pined
                                    on:change=move |e| f_pined.set(event_target_checked(&e))
                                    class="w-4 h-4 accent-teal-600 cursor-pointer"/>
                                <label for="pined" class="text-sm text-fg cursor-pointer">"Featured (pin ke atas)"</label>
                            </div>

                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Details (opsional, konten panjang)"</label>
                                <textarea rows="5" prop:value=f_details on:input=move |e| f_details.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"></textarea>
                            </div>
                        </div>

                        {move || error.get().map(|e| view! {
                            <p class="text-red-400 text-sm mb-3">{e}</p>
                        })}

                        <div class="flex items-center gap-2">
                            <button type="submit" disabled=move || saving.get()
                                class="px-4 py-2 bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold rounded-lg transition-colors cursor-pointer disabled:opacity-60">
                                {move || if saving.get() { "Menyimpan..." } else { "Simpan" }}
                            </button>
                            <button type="button" on:click=move |_| show_form.set(false)
                                class="px-4 py-2 border border-line text-muted hover:text-fg text-sm font-semibold rounded-lg transition-colors cursor-pointer">
                                "Batal"
                            </button>
                        </div>
                    </form>
                })}

                <div class="bg-surface border border-line rounded-2xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <div class="overflow-y-auto max-h-104">
                            <table class="w-full text-sm text-fg">
                                <thead class="sticky top-0 bg-surface z-10">
                                    <tr class="border-b border-line">
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-16">"ID"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Title"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Slug"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-24">"Featured"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Aksi"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || match items_resource.get() {
                                        Some(Ok((rows, _))) if rows.is_empty() => view! {
                                            <tr><td colspan="5" class="px-5 py-6 text-muted text-center text-xs">"Belum ada project."</td></tr>
                                        }.into_any(),
                                        Some(Ok((rows, _))) => rows.into_iter().map(|row| {
                                            let row_for_edit = row.clone();
                                            let id = row.portfolio_id;
                                            view! {
                                                <tr class="border-b border-line last:border-0 hover:bg-teal-500/5 transition-colors">
                                                    <td class="px-5 py-3.5 text-muted font-mono text-xs">{row.portfolio_id}</td>
                                                    <td class="px-5 py-3.5 text-fg font-medium">{row.title.clone()}</td>
                                                    <td class="px-5 py-3.5 text-muted text-xs">{row.slug.clone()}</td>
                                                    <td class="px-5 py-3.5">
                                                        {row.pined.then(|| view! {
                                                            <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-teal-500/15 text-teal-400">
                                                                <i class="bi bi-star-fill text-[0.65rem]"></i>
                                                            </span>
                                                        })}
                                                    </td>
                                                    <td class="px-5 py-3.5">
                                                        <div class="flex items-center gap-3">
                                                            <button on:click=move |_| open_edit(row_for_edit.clone())
                                                                class="text-teal-500 hover:text-teal-400 cursor-pointer">
                                                                <i class="bi bi-pencil-square"></i>
                                                            </button>
                                                            <button on:click=move |_| on_delete(id)
                                                                class="text-red-400 hover:text-red-300 cursor-pointer">
                                                                <i class="bi bi-trash3"></i>
                                                            </button>
                                                        </div>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view().into_any(),
                                        Some(Err(e)) => view! {
                                            <tr><td colspan="5" class="px-5 py-6 text-red-400 text-center text-xs">{e.to_string()}</td></tr>
                                        }.into_any(),
                                        None => view! {
                                            <tr><td colspan="5" class="px-5 py-6 text-muted text-center text-xs">"Memuat..."</td></tr>
                                        }.into_any(),
                                    }}
                                </tbody>
                            </table>
                        </div>
                    </div>
                    {pagination_footer(items_resource, page, PAGE_SIZE)}
                </div>
            </div>
        </AdminLayout>
    }
}
