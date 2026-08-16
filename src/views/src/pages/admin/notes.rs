use leptos::prelude::*;
use modules::notes::NoteView;

use super::layout::{AdminLayout, TableErrorRow, TableSkeleton, pagination_footer};

#[cfg(feature = "ssr")]
use modules::Validate;

// ── Server functions (admin-guarded) ─────────────────────────────────────────
#[cfg(feature = "ssr")]
async fn note_svc() -> Result<std::sync::Arc<dyn modules::notes::NoteService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::notes::NoteService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn admin_list_notes() -> Result<Vec<NoteView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    note_svc()
        .await?
        .find_all_admin_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Paginated note listing for the admin table — real LIMIT/OFFSET in SQL.
#[server]
pub async fn admin_list_notes_page(page: i64) -> Result<(Vec<NoteView>, i64), ServerFnError> {
    crate::pages::admin::require_admin().await?;
    note_svc()
        .await?
        .find_all_admin_page_async(page, 10)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_create_note(
    input: modules::notes::NoteCommand,
) -> Result<NoteView, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    input
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    note_svc()
        .await?
        .create_async(input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_update_note(
    id: i32,
    input: modules::notes::NoteCommand,
) -> Result<Option<NoteView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    input
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    note_svc()
        .await?
        .update_async(id, input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_toggle_note(id: i32, enabled: bool) -> Result<(), ServerFnError> {
    crate::pages::admin::require_admin().await?;
    note_svc()
        .await?
        .toggle_enabled_async(id, enabled)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_delete_note(id: i32) -> Result<bool, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    note_svc()
        .await?
        .delete_async(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Page ──────────────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
pub fn AdminNotesPage() -> impl IntoView {
    const PAGE_SIZE: i64 = 10;
    let page = RwSignal::new(1i64);
    let refetch = RwSignal::new(0u32);
    let items_resource = LocalResource::new(move || {
        refetch.get();
        admin_list_notes_page(page.get())
    });

    let show_form = RwSignal::new(false);
    let editing_id = RwSignal::new(None::<i32>);
    let f_category = RwSignal::new(String::new());
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
        f_category.set(String::new());
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

    let open_edit = move |row: NoteView| {
        editing_id.set(Some(row.notes_id));
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

        let input = modules::notes::NoteCommand {
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
                Some(id) => admin_update_note(id, input).await.map(|_| ()),
                None => admin_create_note(input).await.map(|_| ()),
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
                .and_then(|w| w.confirm_with_message("Hapus note ini?").ok())
                .unwrap_or(false)
            {
                return;
            }
        }
        leptos::task::spawn_local(async move {
            if admin_delete_note(id).await.is_ok() {
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
                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Notes"</h2>
                    <p class="text-muted text-sm">
                        "Kelola metadata notes. Isi artikel (markdown) tetap di-host di GitHub — "
                        "kolom \"Content URL\" cuma nunjuk ke raw file-nya di sana."
                    </p>
                </div>

                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-lg font-bold text-fg">"Daftar Notes"</h3>
                    <button
                        on:click=open_create
                        class="inline-flex items-center gap-1.5 px-3.5 py-2 bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold rounded-lg transition-colors cursor-pointer">
                        <i class="bi bi-plus-lg"></i>
                        "Tambah Note"
                    </button>
                </div>

                {move || show_form.get().then(|| view! {
                    <form on:submit=on_submit class="bg-surface border border-line rounded-2xl p-5 mb-5">
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Category"</label>
                                <input required prop:value=f_category on:input=move |e| f_category.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="backend"/>
                            </div>
                            <div>
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Slug"</label>
                                <input required prop:value=f_slug on:input=move |e| f_slug.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="judul-artikel-slug"/>
                            </div>
                            <div class="sm:col-span-2">
                                <label class="block text-xs font-medium mb-1.5 text-fg">"Title"</label>
                                <input required prop:value=f_title on:input=move |e| f_title.set(event_target_value(&e))
                                    class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                    placeholder="Judul Artikel"/>
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
                                    placeholder="rust, backend, sqlx"/>
                            </div>
                            <div class="flex items-center gap-2">
                                <input type="checkbox" id="enabled" prop:checked=f_enabled
                                    on:change=move |e| f_enabled.set(event_target_checked(&e))
                                    class="w-4 h-4 accent-teal-600 cursor-pointer"/>
                                <label for="enabled" class="text-sm text-fg cursor-pointer">"Published (tampil di /notes)"</label>
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
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Category"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-24">"Status"</th>
                                        <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Aksi"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || match items_resource.get() {
                                        Some(Ok((rows, _))) if rows.is_empty() => view! {
                                            <tr><td colspan="5" class="px-5 py-6 text-muted text-center text-xs">"Belum ada note."</td></tr>
                                        }.into_any(),
                                        Some(Ok((rows, _))) => rows.into_iter().map(|row| {
                                            let row_for_edit = row.clone();
                                            let id = row.notes_id;
                                            let toggled = RwSignal::new(row.enabled);
                                            let toggling = RwSignal::new(false);
                                            let on_toggle = move |_| {
                                                let next = !toggled.get();
                                                toggling.set(true);
                                                leptos::task::spawn_local(async move {
                                                    if admin_toggle_note(id, next).await.is_ok() {
                                                        toggled.set(next);
                                                    }
                                                    toggling.set(false);
                                                });
                                            };
                                            view! {
                                                <tr class="border-b border-line last:border-0 hover:bg-teal-500/5 transition-colors">
                                                    <td class="px-5 py-3.5 text-muted font-mono text-xs">{row.notes_id}</td>
                                                    <td class="px-5 py-3.5 text-fg font-medium">{row.title.clone()}</td>
                                                    <td class="px-5 py-3.5">
                                                        <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium bg-line text-muted">
                                                            {row.category.clone()}
                                                        </span>
                                                    </td>
                                                    <td class="px-5 py-3.5">
                                                        <button
                                                            on:click=on_toggle
                                                            disabled=move || toggling.get()
                                                            title=move || if toggled.get() { "Klik untuk jadikan Draft" } else { "Klik untuk Publish" }
                                                            class="inline-flex items-center gap-1.5 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                                                        >
                                                            <span
                                                                class="relative inline-flex w-9 h-5 rounded-full transition-colors duration-200"
                                                                class=("bg-teal-500", move || toggled.get())
                                                                class=("bg-line", move || !toggled.get())
                                                            >
                                                                <span
                                                                    class="absolute top-0.5 left-0.5 w-4 h-4 rounded-full bg-white shadow transition-transform duration-200"
                                                                    class=("translate-x-4", move || toggled.get())
                                                                ></span>
                                                            </span>
                                                            <span
                                                                class="text-xs font-semibold"
                                                                class=("text-teal-400", move || toggled.get())
                                                                class=("text-muted", move || !toggled.get())
                                                            >
                                                                {move || if toggled.get() { "Published" } else { "Draft" }}
                                                            </span>
                                                        </button>
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
