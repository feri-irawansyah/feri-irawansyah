use leptos::prelude::*;
use modules::laboratory::LaboratoryView;

use super::layout::{pagination_footer, AdminLayout, TableErrorRow, TableSkeleton};

#[cfg(feature = "ssr")]
use modules::Validate;

/// The 5 fixed lab categories — kept in sync with `pages::laboratory::CATEGORIES`
/// (slug + Supabase image), but this is just (slug, admin dropdown label).
const CATEGORY_OPTIONS: &[(&str, &str)] = &[
    ("performance", "Performance"),
    ("security", "Security & DevOps"),
    ("architecture", "Architecture"),
    ("rendering", "Rendering"),
    ("restapi", "REST API"),
];

// ── Server functions (admin-guarded) ─────────────────────────────────────────

#[cfg(feature = "ssr")]
async fn laboratory_svc()
-> Result<std::sync::Arc<dyn modules::laboratory::LaboratoryService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::laboratory::LaboratoryService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

/// Paginated listing for the admin table — real LIMIT/OFFSET in SQL.
#[server]
pub async fn admin_list_laboratory_page(
    page: i64,
) -> Result<(Vec<LaboratoryView>, i64), ServerFnError> {
    crate::pages::admin::require_admin().await?;
    laboratory_svc()
        .await?
        .find_all_admin_page_async(page, 10)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_create_laboratory(
    input: modules::laboratory::LaboratoryCommand,
) -> Result<LaboratoryView, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    input.validate().map_err(|e| ServerFnError::new(e.to_string()))?;
    laboratory_svc()
        .await?
        .create_async(input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_update_laboratory(
    id: i32,
    input: modules::laboratory::LaboratoryCommand,
) -> Result<Option<LaboratoryView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    input.validate().map_err(|e| ServerFnError::new(e.to_string()))?;
    laboratory_svc()
        .await?
        .update_async(id, input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_delete_laboratory(id: i32) -> Result<bool, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    laboratory_svc()
        .await?
        .delete_async(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Page ──────────────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
pub fn AdminLaboratoryPage() -> impl IntoView {
    const PAGE_SIZE: i64 = 10;
    let page = RwSignal::new(1i64);
    let refetch = RwSignal::new(0u32);
    let items_resource = LocalResource::new(move || {
        refetch.get();
        admin_list_laboratory_page(page.get())
    });

    let show_form = RwSignal::new(false);
    let editing_id = RwSignal::new(None::<i32>);
    let f_category = RwSignal::new(CATEGORY_OPTIONS[0].0.to_string());
    let f_title = RwSignal::new(String::new());
    let f_slug = RwSignal::new(String::new());
    let f_content = RwSignal::new(String::new());
    let f_description = RwSignal::new(String::new());
    let f_hashtag = RwSignal::new(String::new());
    let f_enabled = RwSignal::new(true);
    let saving = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    let reset_form = move || {
        editing_id.set(None);
        f_category.set(CATEGORY_OPTIONS[0].0.to_string());
        f_title.set(String::new());
        f_slug.set(String::new());
        f_content.set(String::new());
        f_description.set(String::new());
        f_hashtag.set(String::new());
        f_enabled.set(true);
        error.set(None);
    };

    let open_create = move |_| {
        reset_form();
        show_form.set(true);
    };

    let open_edit = move |row: LaboratoryView| {
        editing_id.set(Some(row.lab_id));
        f_category.set(row.category);
        f_title.set(row.title);
        f_slug.set(row.slug);
        f_content.set(row.content);
        f_description.set(row.description);
        f_hashtag.set(row.hashtag.join(", "));
        f_enabled.set(row.enabled);
        error.set(None);
        show_form.set(true);
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(None);

        let hashtag: Vec<String> = f_hashtag
            .get()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let input = modules::laboratory::LaboratoryCommand {
            category: f_category.get(),
            title: f_title.get(),
            slug: f_slug.get(),
            content: f_content.get(),
            description: f_description.get(),
            hashtag,
            enabled: f_enabled.get(),
        };
        let id = editing_id.get();

        saving.set(true);
        leptos::task::spawn_local(async move {
            let result = match id {
                Some(id) => admin_update_laboratory(id, input).await.map(|_| ()),
                None => admin_create_laboratory(input).await.map(|_| ()),
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
                .and_then(|w| w.confirm_with_message("Hapus eksperimen ini?").ok())
                .unwrap_or(false)
            {
                return;
            }
        }
        leptos::task::spawn_local(async move {
            if admin_delete_laboratory(id).await.is_ok() {
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
                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Laboratorium"</h2>
                    <p class="text-muted text-sm">
                        "Kelola metadata eksperimen lab. Isi docs (markdown) tetap di-host di GitHub — "
                        "kolom \"Content URL\" cuma nunjuk ke raw file-nya di sana."
                    </p>
                </div>

                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-bold text-fg">"Daftar Eksperimen"</h3>
                    <button
                        on:click=open_create
                        class="inline-flex items-center gap-1.5 px-3.5 py-2 bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold rounded-lg transition-colors cursor-pointer">
                        <i class="bi bi-plus-lg"></i>
                        "Tambah Eksperimen"
                    </button>
                </div>

                {move || show_form.get().then(|| view! {
                    <form on:submit=on_submit class="bg-surface border border-line rounded-2xl p-5 mb-5">
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Category"</label>
                                <select
                                    prop:value=f_category
                                    on:change=move |e| f_category.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm">
                                    {CATEGORY_OPTIONS.iter().map(|(slug, label)| view! {
                                        <option value=*slug>{*label}</option>
                                    }).collect_view()}
                                </select>
                            </div>
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Slug"</label>
                                <input required prop:value=f_slug on:input=move |e| f_slug.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="judul-eksperimen-slug"/>
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Title"</label>
                                <input required prop:value=f_title on:input=move |e| f_title.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="Judul Eksperimen"/>
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Content URL (raw GitHub markdown)"</label>
                                <input required prop:value=f_content on:input=move |e| f_content.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/.../README.md"/>
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Description (teaser)"</label>
                                <textarea rows="2" prop:value=f_description on:input=move |e| f_description.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"></textarea>
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Hashtags (pisah pakai koma)"</label>
                                <input prop:value=f_hashtag on:input=move |e| f_hashtag.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="rust, benchmark, tokio"/>
                            </div>
                            <div class="flex items-center gap-2">
                                <input type="checkbox" id="enabled" prop:checked=f_enabled
                                    on:change=move |e| f_enabled.set(event_target_checked(&e))
                                    class="w-4 h-4 accent-teal-600 cursor-pointer"/>
                                <label for="enabled" class="text-sm text-fg cursor-pointer">"Published (tampil di /laboratory)"</label>
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
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-32">"Category"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-24">"Status"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Aksi"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || match items_resource.get() {
                                        Some(Ok((rows, _))) if rows.is_empty() => view! {
                                            <tr><td colspan="5" class="px-5 py-6 text-muted text-center text-xs">"Belum ada eksperimen."</td></tr>
                                        }.into_any(),
                                        Some(Ok((rows, _))) => rows.into_iter().map(|row| {
                                            let row_for_edit = row.clone();
                                            let id = row.lab_id;
                                            let enabled = row.enabled;
                                            view! {
                                                <tr class="border-b border-line last:border-0 hover:bg-teal-500/5 transition-colors">
                                                    <td class="px-5 py-3.5 text-muted font-mono text-xs">{row.lab_id}</td>
                                                    <td class="px-5 py-3.5 text-fg font-medium">{row.title.clone()}</td>
                                                    <td class="px-5 py-3.5">
                                                        <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium bg-line text-muted whitespace-nowrap">
                                                            {row.category.clone()}
                                                        </span>
                                                    </td>
                                                    <td class="px-5 py-3.5">
                                                        {if enabled {
                                                            view! {
                                                                <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-green-500/15 text-green-400 whitespace-nowrap">
                                                                    "Published"
                                                                </span>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-line text-muted whitespace-nowrap">
                                                                    "Hidden"
                                                                </span>
                                                            }.into_any()
                                                        }}
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
                                        Some(Err(e)) => view! { <TableErrorRow cols=5 message=e.to_string() /> }.into_any(),
                                        None => view! { <TableSkeleton cols=5 rows=6 /> }.into_any(),
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
