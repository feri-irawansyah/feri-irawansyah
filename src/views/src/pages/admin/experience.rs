use leptos::prelude::*;
use modules::experience::ExperienceView;
use modules::positions::PositionView;

#[cfg(feature = "ssr")]
use modules::Validate;

use super::layout::{AdminLayout, TableErrorRow, TableSkeleton, pagination_footer};

// ── Server functions (admin-guarded) ─────────────────────────────────────────

#[cfg(feature = "ssr")]
async fn experience_svc()
-> Result<std::sync::Arc<dyn modules::experience::ExperienceService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::experience::ExperienceService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[cfg(feature = "ssr")]
async fn position_svc()
-> Result<std::sync::Arc<dyn modules::positions::PositionService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::positions::PositionService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[cfg(feature = "ssr")]
fn parse_dates(
    start_date: &str,
    end_date: &str,
) -> Result<(chrono::NaiveDate, Option<chrono::NaiveDate>), ServerFnError> {
    let start = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| ServerFnError::new(format!("Tanggal mulai tidak valid: {e}")))?;
    let end = if end_date.trim().is_empty() {
        None
    } else {
        Some(
            chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
                .map_err(|e| ServerFnError::new(format!("Tanggal selesai tidak valid: {e}")))?,
        )
    };
    Ok((start, end))
}

#[server]
pub async fn admin_list_experiences() -> Result<Vec<ExperienceView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    experience_svc()
        .await?
        .find_all_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Paginated experience listing for the admin table — actual LIMIT/OFFSET in
/// SQL, unlike `admin_list_experiences` above which still returns everything
/// unpaginated (kept as-is since PositionSection's company dropdown needs the
/// full list, not just one page of it).
#[server]
pub async fn admin_list_experiences_page(
    page: i64,
) -> Result<(Vec<ExperienceView>, i64), ServerFnError> {
    crate::pages::admin::require_admin().await?;
    experience_svc()
        .await?
        .find_page_async(page, 10)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_create_experience(
    title: String,
    company: String,
    url_docs: String,
    image_src: String,
    start_date: String,
    end_date: String,
) -> Result<ExperienceView, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let (start_date, end_date) = parse_dates(&start_date, &end_date)?;
    let input = modules::experience::ExperienceCommand {
        title,
        company,
        url_docs,
        image_src,
        start_date,
        end_date,
    };
    input
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    experience_svc()
        .await?
        .create_async(input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_update_experience(
    id: i32,
    title: String,
    company: String,
    url_docs: String,
    image_src: String,
    start_date: String,
    end_date: String,
) -> Result<Option<ExperienceView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let (start_date, end_date) = parse_dates(&start_date, &end_date)?;
    let input = modules::experience::ExperienceCommand {
        title,
        company,
        url_docs,
        image_src,
        start_date,
        end_date,
    };
    input
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    experience_svc()
        .await?
        .update_async(id, input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_delete_experience(id: i32) -> Result<bool, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    experience_svc()
        .await?
        .delete_async(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_list_positions() -> Result<Vec<PositionView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    position_svc()
        .await?
        .find_all_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Paginated position listing for the admin table — LIMIT/OFFSET happens in
/// SQL at the repository layer, then joined against experience in the service
/// layer as usual. `admin_list_positions` above stays untouched.
#[server]
pub async fn admin_list_positions_page(
    page: i64,
) -> Result<(Vec<PositionView>, i64), ServerFnError> {
    crate::pages::admin::require_admin().await?;
    position_svc()
        .await?
        .find_page_async(page, 10)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Raw form values for a position — dates/description arrive as plain
/// strings from HTML inputs and get parsed into `PositionCommand` below.
/// Bundled into one struct so the `#[server]` fns stay under clippy's
/// too-many-arguments threshold.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PositionFormInput {
    pub experience_id: i32,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
    pub address: String,
    pub job_position: String,
    pub job_type: String,
    pub sort_order: i32,
}

#[cfg(feature = "ssr")]
impl PositionFormInput {
    fn into_command(self) -> Result<modules::positions::PositionCommand, ServerFnError> {
        let (start_date, end_date) = parse_dates(&self.start_date, &self.end_date)?;
        let description: Vec<String> = self
            .description
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        Ok(modules::positions::PositionCommand {
            experience_id: self.experience_id,
            title: self.title,
            start_date,
            end_date,
            description,
            address: self.address,
            job_position: self.job_position,
            job_type: self.job_type,
            sort_order: self.sort_order,
        })
    }
}

#[server]
pub async fn admin_create_position(
    input: PositionFormInput,
) -> Result<PositionView, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let input = input.into_command()?;
    input
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    position_svc()
        .await?
        .create_async(input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_update_position(
    id: i32,
    input: PositionFormInput,
) -> Result<Option<PositionView>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let input = input.into_command()?;
    input
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    position_svc()
        .await?
        .update_async(id, input)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn admin_delete_position(id: i32) -> Result<bool, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    position_svc()
        .await?
        .delete_async(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Page ──────────────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
pub fn AdminExperiencePage() -> impl IntoView {
    let exp_refetch = RwSignal::new(0u32);
    let exp_resource = LocalResource::new(move || {
        exp_refetch.get();
        admin_list_experiences()
    });

    view! {
        <AdminLayout>
            <div class="p-8 max-w-6xl">
                <div class="mb-8">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        "Manajemen"
                    </span>
                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Experience"</h2>
                    <p class="text-muted text-sm">
                        "Kelola riwayat perusahaan (experience) dan posisi/jabatan di dalamnya."
                    </p>
                </div>

                <ExperienceSection exp_refetch=exp_refetch />

                <div class="mt-10">
                    <PositionSection exp_resource=exp_resource />
                </div>
            </div>
        </AdminLayout>
    }
}

// ── Experience (company) section ─────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
fn ExperienceSection(exp_refetch: RwSignal<u32>) -> impl IntoView {
    const PAGE_SIZE: i64 = 10;
    let page = RwSignal::new(1i64);
    let table_resource = LocalResource::new(move || {
        exp_refetch.get();
        admin_list_experiences_page(page.get())
    });
    let show_form = RwSignal::new(false);
    let editing_id = RwSignal::new(None::<i32>);
    let f_title = RwSignal::new(String::new());
    let f_company = RwSignal::new(String::new());
    let f_url = RwSignal::new(String::new());
    let f_image = RwSignal::new(String::new());
    let f_start = RwSignal::new(String::new());
    let f_end = RwSignal::new(String::new());
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

            let Some(input) = _ev
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            else {
                return;
            };
            let Some(files) = input.files() else { return };
            let Some(file) = files.get(0) else { return };

            let Ok(form_data) = FormData::new() else {
                return;
            };
            let _ = form_data.append_with_str("folder", "experience");
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
        f_company.set(String::new());
        f_url.set(String::new());
        f_image.set(String::new());
        f_start.set(String::new());
        f_end.set(String::new());
        error.set(None);
        upload_error.set(None);
    };

    let open_create = move |_| {
        reset_form();
        show_form.set(true);
    };

    let open_edit = move |row: ExperienceView| {
        editing_id.set(Some(row.id));
        f_title.set(row.title);
        f_company.set(row.company);
        f_url.set(row.url_docs);
        f_image.set(row.image_src);
        f_start.set(row.start_date.format("%Y-%m-%d").to_string());
        f_end.set(
            row.end_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
        );
        error.set(None);
        show_form.set(true);
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        saving.set(true);
        error.set(None);
        let title = f_title.get();
        let company = f_company.get();
        let url = f_url.get();
        let image = f_image.get();
        let start = f_start.get();
        let end = f_end.get();
        let id = editing_id.get();

        leptos::task::spawn_local(async move {
            let result = match id {
                Some(id) => admin_update_experience(id, title, company, url, image, start, end)
                    .await
                    .map(|_| ()),
                None => admin_create_experience(title, company, url, image, start, end)
                    .await
                    .map(|_| ()),
            };
            match result {
                Ok(()) => {
                    saving.set(false);
                    show_form.set(false);
                    exp_refetch.update(|n| *n += 1);
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
                .and_then(|w| w.confirm_with_message("Hapus experience ini? Position yang masih terhubung tidak akan ikut terhapus.").ok())
                .unwrap_or(false)
            {
                return;
            }
        }
        leptos::task::spawn_local(async move {
            if admin_delete_experience(id).await.is_ok() {
                exp_refetch.update(|n| *n += 1);
            }
        });
    };

    view! {
        <section>
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-bold text-fg">"Experience (Perusahaan)"</h3>
                <button
                    on:click=open_create
                    class="inline-flex items-center gap-1.5 px-3.5 py-2 bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold rounded-lg transition-colors cursor-pointer">
                    <i class="bi bi-plus-lg"></i>
                    "Tambah Experience"
                </button>
            </div>

            {move || show_form.get().then(|| view! {
                <form on:submit=on_submit class="bg-surface border border-line rounded-2xl p-5 mb-5">
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Title / Jabatan"</label>
                            <input required prop:value=f_title on:input=move |e| f_title.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Founder"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Nama Perusahaan"</label>
                            <input required prop:value=f_company on:input=move |e| f_company.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Snakesystem"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"URL Docs"</label>
                            <input prop:value=f_url on:input=move |e| f_url.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="https://..."/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Logo Perusahaan"</label>
                            <div class="flex items-center gap-3">
                                {move || (!f_image.get().is_empty()).then(|| view! {
                                    <div class="w-11 h-11 rounded-lg bg-white border border-line flex items-center justify-center shrink-0 overflow-hidden">
                                        <img src=f_image.get() alt="Preview" class="w-full h-full object-contain"/>
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
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Tanggal Mulai"</label>
                            <input type="date" required prop:value=f_start on:input=move |e| f_start.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Tanggal Selesai (kosongkan jika masih berjalan)"</label>
                            <input type="date" prop:value=f_end on:input=move |e| f_end.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm"/>
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
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Perusahaan"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Title"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Periode"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Aksi"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || match table_resource.get() {
                                    Some(Ok((rows, _))) if rows.is_empty() => view! {
                                        <tr><td colspan="5" class="px-5 py-6 text-muted text-center text-xs">"Belum ada experience."</td></tr>
                                    }.into_any(),
                                    Some(Ok((rows, _))) => rows.into_iter().map(|row| {
                                        let row_for_edit = row.clone();
                                        let id = row.id;
                                        let period = match row.end_date {
                                            Some(end) => format!("{} – {}", row.start_date.format("%b %Y"), end.format("%b %Y")),
                                            None => format!("{} – Present", row.start_date.format("%b %Y")),
                                        };
                                        view! {
                                            <tr class="border-b border-line last:border-0 hover:bg-teal-500/5 transition-colors">
                                                <td class="px-5 py-3.5 text-muted font-mono text-xs">{row.id}</td>
                                                <td class="px-5 py-3.5 text-fg font-medium">{row.company.clone()}</td>
                                                <td class="px-5 py-3.5 text-fg">{row.title.clone()}</td>
                                                <td class="px-5 py-3.5 text-muted text-xs">{period}</td>
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
                {pagination_footer(table_resource, page, PAGE_SIZE)}
            </div>
        </section>
    }
}

// ── Position section ─────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
fn PositionSection(
    exp_resource: LocalResource<Result<Vec<ExperienceView>, ServerFnError>>,
) -> impl IntoView {
    const PAGE_SIZE: i64 = 10;
    let page = RwSignal::new(1i64);
    let pos_refetch = RwSignal::new(0u32);
    let pos_resource = LocalResource::new(move || {
        pos_refetch.get();
        admin_list_positions_page(page.get())
    });

    let show_form = RwSignal::new(false);
    let editing_id = RwSignal::new(None::<i32>);
    let f_experience_id = RwSignal::new(String::new());
    let f_title = RwSignal::new(String::new());
    let f_start = RwSignal::new(String::new());
    let f_end = RwSignal::new(String::new());
    let f_description = RwSignal::new(String::new());
    let f_address = RwSignal::new(String::new());
    let f_job_position = RwSignal::new(String::new());
    let f_job_type = RwSignal::new(String::new());
    let f_sort_order = RwSignal::new(String::new());
    let saving = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    let reset_form = move || {
        editing_id.set(None);
        f_experience_id.set(String::new());
        f_title.set(String::new());
        f_start.set(String::new());
        f_end.set(String::new());
        f_description.set(String::new());
        f_address.set(String::new());
        f_job_position.set(String::new());
        f_job_type.set(String::new());
        f_sort_order.set("0".to_string());
        error.set(None);
    };

    let open_create = move |_| {
        reset_form();
        show_form.set(true);
    };

    let open_edit = move |row: PositionView| {
        editing_id.set(Some(row.id));
        f_experience_id.set(row.experience_id.to_string());
        f_title.set(row.title);
        f_start.set(row.start_date.format("%Y-%m-%d").to_string());
        f_end.set(
            row.end_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
        );
        f_description.set(row.description.join("\n"));
        f_address.set(row.address);
        f_job_position.set(row.job_position);
        f_job_type.set(row.job_type);
        f_sort_order.set(row.sort_order.to_string());
        error.set(None);
        show_form.set(true);
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(None);

        let Ok(experience_id) = f_experience_id.get().parse::<i32>() else {
            error.set(Some(
                "Pilih experience/perusahaan terlebih dahulu.".to_string(),
            ));
            return;
        };
        let sort_order = f_sort_order.get().parse::<i32>().unwrap_or(0);
        let title = f_title.get();
        let start = f_start.get();
        let end = f_end.get();
        let description = f_description.get();
        let address = f_address.get();
        let job_position = f_job_position.get();
        let job_type = f_job_type.get();
        let id = editing_id.get();

        let input = PositionFormInput {
            experience_id,
            title,
            start_date: start,
            end_date: end,
            description,
            address,
            job_position,
            job_type,
            sort_order,
        };

        saving.set(true);
        leptos::task::spawn_local(async move {
            let result = match id {
                Some(id) => admin_update_position(id, input).await.map(|_| ()),
                None => admin_create_position(input).await.map(|_| ()),
            };
            match result {
                Ok(()) => {
                    saving.set(false);
                    show_form.set(false);
                    pos_refetch.update(|n| *n += 1);
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
                .and_then(|w| w.confirm_with_message("Hapus position ini?").ok())
                .unwrap_or(false)
            {
                return;
            }
        }
        leptos::task::spawn_local(async move {
            if admin_delete_position(id).await.is_ok() {
                pos_refetch.update(|n| *n += 1);
            }
        });
    };

    view! {
        <section>
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-bold text-fg">"Positions"</h3>
                <button
                    on:click=open_create
                    class="inline-flex items-center gap-1.5 px-3.5 py-2 bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold rounded-lg transition-colors cursor-pointer">
                    <i class="bi bi-plus-lg"></i>
                    "Tambah Position"
                </button>
            </div>

            {move || show_form.get().then(|| view! {
                <form on:submit=on_submit class="bg-surface border border-line rounded-2xl p-5 mb-5">
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                        <div class="sm:col-span-2">
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Experience / Perusahaan"</label>
                            <select required prop:value=f_experience_id on:change=move |e| f_experience_id.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm">
                                <option value="">"— Pilih —"</option>
                                {move || exp_resource.get().and_then(|r| r.ok()).unwrap_or_default().into_iter().map(|exp| {
                                    view! { <option value=exp.id.to_string()>{format!("{} — {}", exp.company, exp.title)}</option> }
                                }).collect_view()}
                            </select>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Title / Jabatan"</label>
                            <input required prop:value=f_title on:input=move |e| f_title.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Web Developer"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Alamat"</label>
                            <input prop:value=f_address on:input=move |e| f_address.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Jakarta Selatan, DKI Jakarta"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Tanggal Mulai"</label>
                            <input type="date" required prop:value=f_start on:input=move |e| f_start.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Tanggal Selesai (kosongkan jika masih berjalan)"</label>
                            <input type="date" prop:value=f_end on:input=move |e| f_end.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Job Position (Fulltime/Internship/dst)"</label>
                            <input required prop:value=f_job_position on:input=move |e| f_job_position.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Fulltime"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Job Type (Onsite/Remote/Hybrid/Online)"</label>
                            <input required prop:value=f_job_type on:input=move |e| f_job_type.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Hybrid"/>
                        </div>
                        <div>
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Sort Order"</label>
                            <input type="number" prop:value=f_sort_order on:input=move |e| f_sort_order.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="0"/>
                        </div>
                        <div class="sm:col-span-2">
                            <label class="block text-xs font-medium mb-1.5 text-fg">"Description (satu poin per baris)"</label>
                            <textarea rows="4" prop:value=f_description on:input=move |e| f_description.set(event_target_value(&e))
                                class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                placeholder="Rewrite from VB Net to ASP .Net Core...\nTuning the database with Stored Procedures..."></textarea>
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
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Perusahaan"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Title"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider">"Periode"</th>
                                    <th class="text-left px-5 py-3.5 text-xs font-semibold text-muted uppercase tracking-wider w-28">"Aksi"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || match pos_resource.get() {
                                    Some(Ok((rows, _))) if rows.is_empty() => view! {
                                        <tr><td colspan="5" class="px-5 py-6 text-muted text-center text-xs">"Belum ada position."</td></tr>
                                    }.into_any(),
                                    Some(Ok((rows, _))) => rows.into_iter().map(|row| {
                                        let row_for_edit = row.clone();
                                        let id = row.id;
                                        let period = match row.end_date {
                                            Some(end) => format!("{} – {}", row.start_date.format("%b %Y"), end.format("%b %Y")),
                                            None => format!("{} – Present", row.start_date.format("%b %Y")),
                                        };
                                        view! {
                                            <tr class="border-b border-line last:border-0 hover:bg-teal-500/5 transition-colors">
                                                <td class="px-5 py-3.5 text-muted font-mono text-xs">{row.id}</td>
                                                <td class="px-5 py-3.5 text-fg font-medium">{row.company.clone()}</td>
                                                <td class="px-5 py-3.5 text-fg">{row.title.clone()}</td>
                                                <td class="px-5 py-3.5 text-muted text-xs">{period}</td>
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
                {pagination_footer(pos_resource, page, PAGE_SIZE)}
            </div>
        </section>
    }
}
