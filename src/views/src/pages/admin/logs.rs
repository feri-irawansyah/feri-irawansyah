use leptos::prelude::*;

use super::layout::AdminLayout;

#[server]
pub async fn admin_read_logs(
    date: String,
    level_filter: String,
    tail: usize,
) -> Result<Vec<String>, ServerFnError> {
    crate::pages::admin::require_admin().await?;

    use tokio::fs;

    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let target_date = if date.is_empty() {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    } else {
        date
    };

    let path = format!("{log_dir}/app.{target_date}.log");
    let content = fs::read_to_string(&path).await.map_err(|e| {
        ServerFnError::new(format!("Log file tidak ditemukan: {path} ({e})"))
    })?;

    let lines: Vec<String> = content
        .lines()
        .filter(|line| {
            if level_filter == "ALL" || level_filter.is_empty() {
                return true;
            }
            line.contains(&level_filter)
        })
        .map(|l| l.to_string())
        .collect();

    let tail = tail.max(1).min(5000);
    let start = lines.len().saturating_sub(tail);
    Ok(lines[start..].to_vec())
}

#[server]
pub async fn admin_list_log_dates() -> Result<Vec<String>, ServerFnError> {
    crate::pages::admin::require_admin().await?;

    use tokio::fs;

    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let mut dates = Vec::new();

    if let Ok(mut entries) = fs::read_dir(&log_dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name().to_string_lossy().to_string();
            // tracing-appender creates files named "app.YYYY-MM-DD.log"
            if let Some(rest) = name.strip_prefix("app.") {
                if let Some(date) = rest.strip_suffix(".log") {
                    dates.push(date.to_string());
                }
            }
        }
    }

    dates.sort_unstable_by(|a, b| b.cmp(a)); // newest first
    Ok(dates)
}

fn level_color(line: &str) -> &'static str {
    if line.contains("ERROR") {
        "text-red-400 hover:bg-red-500/10"
    } else if line.contains("WARN") {
        "text-yellow-400 hover:bg-yellow-500/10"
    } else if line.contains("INFO") {
        "text-green-400 hover:bg-green-500/10"
    } else if line.contains("DEBUG") {
        "text-blue-400 hover:bg-blue-500/10"
    } else {
        "text-fg/60 hover:bg-teal-500/5"
    }
}

#[allow(non_snake_case)]
#[component]
pub fn AdminLogsPage() -> impl IntoView {
    let selected_date = RwSignal::new(String::new()); // empty = today
    let level_filter = RwSignal::new("ALL".to_string());
    let tail = RwSignal::new(200usize);

    let dates_resource = LocalResource::new(admin_list_log_dates);

    let logs_resource = LocalResource::new(move || {
        admin_read_logs(selected_date.get(), level_filter.get(), tail.get())
    });

    view! {
        <AdminLayout>
            <div class="p-8 max-w-7xl">

                // Header
                <div class="mb-8">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        "System"
                    </span>
                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Log Management"</h2>
                    <p class="text-muted text-sm">"Baca log aplikasi per hari — text atau JSON"</p>
                </div>

                // Controls
                <div class="bg-surface border border-line rounded-2xl p-5 mb-6 flex flex-wrap items-end gap-4">

                    // Date picker
                    <div class="flex flex-col gap-1.5">
                        <label class="text-xs font-semibold text-muted uppercase tracking-wider">"Tanggal"</label>
                        <select
                            on:change=move |e| selected_date.set(event_target_value(&e))
                            class="px-3 py-2 bg-base border border-line rounded-lg text-fg text-sm focus:outline-none focus:border-teal-500 transition-colors cursor-pointer min-w-40"
                        >
                            <option value="">"Hari ini"</option>
                            {move || dates_resource.get().and_then(|r| r.ok()).unwrap_or_default()
                                .into_iter()
                                .map(|d| {
                                    let label = d.clone();
                                    view! { <option value=d>{label}</option> }
                                })
                                .collect_view()
                            }
                        </select>
                    </div>

                    // Level filter
                    <div class="flex flex-col gap-1.5">
                        <label class="text-xs font-semibold text-muted uppercase tracking-wider">"Level"</label>
                        <select
                            on:change=move |e| level_filter.set(event_target_value(&e))
                            class="px-3 py-2 bg-base border border-line rounded-lg text-fg text-sm focus:outline-none focus:border-teal-500 transition-colors cursor-pointer"
                        >
                            <option value="ALL">"Semua"</option>
                            <option value="ERROR">"ERROR"</option>
                            <option value="WARN">"WARN"</option>
                            <option value="INFO">"INFO"</option>
                            <option value="DEBUG">"DEBUG"</option>
                        </select>
                    </div>

                    // Tail N lines
                    <div class="flex flex-col gap-1.5">
                        <label class="text-xs font-semibold text-muted uppercase tracking-wider">"Baris Terakhir"</label>
                        <select
                            on:change=move |e| {
                                if let Ok(n) = event_target_value(&e).parse::<usize>() {
                                    tail.set(n);
                                }
                            }
                            class="px-3 py-2 bg-base border border-line rounded-lg text-fg text-sm focus:outline-none focus:border-teal-500 transition-colors cursor-pointer"
                        >
                            <option value="100">"100"</option>
                            <option value="200" selected>"200"</option>
                            <option value="500">"500"</option>
                            <option value="1000">"1000"</option>
                        </select>
                    </div>

                    // Refresh button
                    <button
                        type="button"
                        on:click=move |_| logs_resource.refetch()
                        class="flex items-center gap-2 px-4 py-2 bg-teal-500/15 text-teal-400 hover:bg-teal-500/25 rounded-lg text-sm font-medium transition-colors cursor-pointer"
                    >
                        <i class="bi bi-arrow-clockwise"></i>
                        "Refresh"
                    </button>

                    // Line count badge
                    <div class="ml-auto flex items-center gap-2">
                        {move || logs_resource.get().and_then(|r| r.ok()).map(|lines| view! {
                            <span class="text-xs text-muted bg-line px-3 py-1.5 rounded-full">
                                {lines.len()} " baris"
                            </span>
                        })}
                    </div>
                </div>

                // Log output
                <div class="bg-surface border border-line rounded-2xl overflow-hidden">
                    {move || match logs_resource.get() {
                        None => view! {
                            <div class="p-8 flex items-center justify-center gap-2 text-muted text-sm">
                                <i class="bi bi-arrow-repeat animate-spin"></i>
                                "Memuat log..."
                            </div>
                        }.into_any(),
                        Some(Err(e)) => view! {
                            <div class="p-6">
                                <div class="bg-red-500/10 border border-red-500/30 rounded-xl p-4 flex items-start gap-3">
                                    <i class="bi bi-exclamation-triangle-fill text-red-400 mt-0.5"></i>
                                    <p class="text-red-400 text-sm">{e.to_string()}</p>
                                </div>
                            </div>
                        }.into_any(),
                        Some(Ok(lines)) if lines.is_empty() => view! {
                            <div class="p-12 text-center text-muted text-sm">
                                <i class="bi bi-file-text text-2xl block mb-3 opacity-40"></i>
                                "Tidak ada log yang cocok dengan filter ini"
                            </div>
                        }.into_any(),
                        Some(Ok(lines)) => view! {
                            <div class="overflow-x-auto">
                                <pre class="font-mono text-xs leading-5 p-5 whitespace-pre-wrap break-all">
                                    {lines.into_iter().map(|line| {
                                        let color = level_color(&line);
                                        view! {
                                            <span class={format!("block px-1 rounded {color}")}>
                                                {line}
                                            </span>
                                        }
                                    }).collect_view()}
                                </pre>
                            </div>
                        }.into_any(),
                    }}
                </div>

            </div>
        </AdminLayout>
    }
}
