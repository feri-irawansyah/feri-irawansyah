use leptos::prelude::*;
use modules::cache::{CacheKeyInfo, CacheStats};

use crate::pages::admin::{AdminLayout, layout::ErrorCard};

#[server]
pub async fn cache_get_stats() -> Result<CacheStats, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::cache::CacheService;
    use std::sync::Arc;

    crate::pages::admin::require_admin().await?;

    let svc = extract::<Data<Arc<dyn CacheService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    svc.get_stats().await.map_err(ServerFnError::new)
}

#[server]
pub async fn cache_get_keys() -> Result<Vec<CacheKeyInfo>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::cache::CacheService;
    use std::sync::Arc;

    crate::pages::admin::require_admin().await?;

    let svc = extract::<Data<Arc<dyn CacheService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    svc.get_keys().await.map_err(ServerFnError::new)
}

#[server]
pub async fn cache_flush_all() -> Result<(), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::cache::CacheService;
    use std::sync::Arc;

    crate::pages::admin::require_admin().await?;

    let svc = extract::<Data<Arc<dyn CacheService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    svc.flush_all().await.map_err(ServerFnError::new)
}

#[server]
pub async fn cache_delete_key(key: String) -> Result<(), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::cache::CacheService;
    use std::sync::Arc;

    crate::pages::admin::require_admin().await?;

    let svc = extract::<Data<Arc<dyn CacheService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    svc.delete_key(key).await.map_err(ServerFnError::new)
}

fn fmt_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}

fn fmt_ttl(secs: i64) -> String {
    if secs == -1 {
        "No expiry".to_string()
    } else if secs < 0 {
        "Expired".to_string()
    } else if secs >= 3600 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{secs}s")
    }
}

#[allow(non_snake_case)]
#[component]
pub fn CacheManagementPage() -> impl IntoView {
    const FALLBACK_MAX: u64 = 33_554_432; // 32 MB — matches maxmemory in valkey.conf

    let tick = RwSignal::new(0u32);
    let confirm_flush = RwSignal::new(false);

    let flush_action = Action::new(|_: &()| cache_flush_all());
    let delete_action = Action::new(|key: &String| cache_delete_key(key.clone()));

    Effect::new(move |_| {
        if flush_action.value().get().is_some() {
            confirm_flush.set(false);
            tick.update(|t| *t += 1);
        }
    });
    Effect::new(move |_| {
        if delete_action.value().get().is_some() {
            tick.update(|t| *t += 1);
        }
    });

    let stats = Resource::new(move || tick.get(), |_| cache_get_stats());
    let keys = Resource::new(move || tick.get(), |_| cache_get_keys());

    view! {
        <AdminLayout>
            <div class="p-6 max-w-6xl">

                // ── Header ─────────────────────────────────────────────────
                <div class="flex items-center justify-between mb-6">
                    <div>
                        <h2 class="text-xl font-bold text-fg">"Cache Management"</h2>
                        <p class="text-sm text-muted mt-0.5">"Local Valkey — 32 MB maxmemory · allkeys-lru"</p>
                    </div>
                    <div class="flex items-center gap-2">
                        <button
                            on:click=move |_| tick.update(|t| *t += 1)
                            class="flex items-center gap-2 px-4 py-2 border border-line text-muted hover:border-teal-500 hover:text-fg rounded-lg text-sm font-medium transition-colors cursor-pointer"
                        >
                            <i class="bi bi-arrow-clockwise"></i>
                            "Refresh"
                        </button>
                        {move || if confirm_flush.get() {
                            view! {
                                <button
                                    on:click=move |_| { flush_action.dispatch(()); }
                                    disabled=move || flush_action.pending().get()
                                    class="flex items-center gap-2 px-4 py-2 bg-red-600 hover:bg-red-500 text-white rounded-lg text-sm font-medium transition-colors cursor-pointer disabled:opacity-50"
                                >
                                    <i class="bi bi-exclamation-triangle"></i>
                                    {move || if flush_action.pending().get() { "Flushing..." } else { "Confirm Flush All?" }}
                                </button>
                                <button
                                    on:click=move |_| confirm_flush.set(false)
                                    class="px-3 py-2 border border-line text-muted hover:border-teal-500 hover:text-fg rounded-lg text-sm font-medium transition-colors cursor-pointer"
                                >
                                    "Cancel"
                                </button>
                            }.into_any()
                        } else {
                            view! {
                                <button
                                    on:click=move |_| confirm_flush.set(true)
                                    class="flex items-center gap-2 px-4 py-2 border border-red-500/40 text-red-400 hover:bg-red-500/10 rounded-lg text-sm font-medium transition-colors cursor-pointer"
                                >
                                    <i class="bi bi-trash3"></i>
                                    "Flush All"
                                </button>
                            }.into_any()
                        }}
                    </div>
                </div>

                // ── Memory stats ────────────────────────────────────────────
                <Suspense fallback=|| view! {
                    <div class="h-28 rounded-xl bg-line/30 animate-pulse mb-4"></div>
                    <div class="grid grid-cols-2 lg:grid-cols-5 gap-4 mb-6">
                        <div class="h-20 rounded-xl bg-line/30 animate-pulse"></div>
                        <div class="h-20 rounded-xl bg-line/30 animate-pulse"></div>
                        <div class="h-20 rounded-xl bg-line/30 animate-pulse"></div>
                        <div class="h-20 rounded-xl bg-line/30 animate-pulse"></div>
                        <div class="h-20 rounded-xl bg-line/30 animate-pulse"></div>
                    </div>
                }>
                    {move || stats.get().map(|r| match r {
                        Err(e) => view! { <ErrorCard message=e.to_string() /> }.into_any(),
                        Ok(s) => {
                            let effective_max = if s.max_bytes > 0 { s.max_bytes } else { FALLBACK_MAX };
                            let pct = (s.used_bytes as f64 / effective_max as f64 * 100.0).min(100.0);
                            let hit_rate = if s.hits + s.misses > 0 {
                                s.hits as f64 / (s.hits + s.misses) as f64 * 100.0
                            } else {
                                0.0
                            };
                            let bar_color = if pct > 80.0 {
                                "bg-red-500"
                            } else if pct > 60.0 {
                                "bg-yellow-500"
                            } else {
                                "bg-teal-500"
                            };
                            let used_h = s.used_human.clone();
                            let peak_h = s.peak_human.clone();

                            view! {
                                // RAM bar
                                <div class="bg-surface border border-line rounded-xl p-5 mb-4">
                                    <div class="flex items-center justify-between mb-3">
                                        <span class="text-sm font-semibold text-fg">"RAM Usage"</span>
                                        <span class="text-sm text-muted">
                                            {used_h.clone()} " / " {fmt_bytes(effective_max)}
                                            " — " {format!("{pct:.1}")} "%"
                                        </span>
                                    </div>
                                    <div class="h-3 bg-line rounded-full overflow-hidden">
                                        <div
                                            class={format!("h-full rounded-full transition-all {bar_color}")}
                                            style={format!("width:{pct:.1}%")}
                                        ></div>
                                    </div>
                                    <div class="flex justify-between mt-2 text-[10px] text-muted">
                                        <span>"0"</span>
                                        <span>{fmt_bytes(effective_max / 4)}</span>
                                        <span>{fmt_bytes(effective_max / 2)}</span>
                                        <span>{fmt_bytes(effective_max * 3 / 4)}</span>
                                        <span>{fmt_bytes(effective_max)}</span>
                                    </div>
                                </div>

                                // Stat cards
                                <div class="grid grid-cols-2 lg:grid-cols-5 gap-4 mb-6">
                                    <div class="bg-surface border border-line rounded-xl p-4">
                                        <p class="text-[10px] text-muted uppercase tracking-wide mb-1">"Used RAM"</p>
                                        <p class="text-lg font-bold text-fg">{used_h}</p>
                                        <p class="text-xs text-teal-500 mt-1">{format!("{pct:.1}")}"%"</p>
                                    </div>
                                    <div class="bg-surface border border-line rounded-xl p-4">
                                        <p class="text-[10px] text-muted uppercase tracking-wide mb-1">"Peak RAM"</p>
                                        <p class="text-lg font-bold text-fg">{peak_h}</p>
                                        <p class="text-xs text-muted mt-1">{fmt_bytes(s.peak_bytes)}</p>
                                    </div>
                                    <div class="bg-surface border border-line rounded-xl p-4">
                                        <p class="text-[10px] text-muted uppercase tracking-wide mb-1">"Total Keys"</p>
                                        <p class="text-lg font-bold text-fg">{s.total_keys}</p>
                                        <p class="text-xs text-muted mt-1">"aktif di DB"</p>
                                    </div>
                                    <div class="bg-surface border border-line rounded-xl p-4">
                                        <p class="text-[10px] text-muted uppercase tracking-wide mb-1">"Hit Rate"</p>
                                        <p class="text-lg font-bold text-fg">{format!("{hit_rate:.1}")}"%"</p>
                                        <p class="text-xs text-muted mt-1">
                                            {s.hits}" hits · "{s.misses}" miss"
                                        </p>
                                    </div>
                                    <div class="bg-surface border border-line rounded-xl p-4">
                                        <p class="text-[10px] text-muted uppercase tracking-wide mb-1">"Fragmentation"</p>
                                        <p class="text-lg font-bold text-fg">{format!("{:.2}", s.fragmentation_ratio)}"x"</p>
                                        <p class="text-xs text-muted mt-1">{s.connected_clients}" clients"</p>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>

                // ── Key details table ────────────────────────────────────────
                <div class="bg-surface border border-line rounded-xl overflow-hidden">
                    <div class="px-5 py-3 border-b border-line flex items-center justify-between">
                        <h3 class="text-sm font-semibold text-fg">"Key Details"</h3>
                        <span class="text-xs text-muted">"sorted by memory · max 500 keys"</span>
                    </div>
                    <Suspense fallback=|| view! {
                        <div class="p-8 flex justify-center">
                            <div class="w-5 h-5 border-2 border-teal-500 border-t-transparent rounded-full animate-spin"></div>
                        </div>
                    }>
                        {move || keys.get().map(|r| match r {
                            Err(e) => view! { <div class="p-4"><ErrorCard message=e.to_string() /></div> }.into_any(),
                            Ok(ks) if ks.is_empty() => view! {
                                <p class="text-muted text-sm p-8 text-center">"Cache kosong"</p>
                            }.into_any(),
                            Ok(ks) => view! {
                                <div class="overflow-x-auto">
                                    <table class="w-full text-sm">
                                        <thead>
                                            <tr class="bg-line/20 border-b border-line">
                                                <th class="text-left px-5 py-2.5 text-[10px] font-semibold text-muted uppercase tracking-wide">"Key"</th>
                                                <th class="text-right px-5 py-2.5 text-[10px] font-semibold text-muted uppercase tracking-wide">"Memory"</th>
                                                <th class="text-right px-5 py-2.5 text-[10px] font-semibold text-muted uppercase tracking-wide">"TTL"</th>
                                                <th class="w-10"></th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {ks.into_iter().map(|k| {
                                                let ttl_str = fmt_ttl(k.ttl_secs);
                                                let ttl_color = if k.ttl_secs == -1 {
                                                    "text-muted"
                                                } else if k.ttl_secs >= 0 && k.ttl_secs < 60 {
                                                    "text-yellow-400"
                                                } else {
                                                    "text-fg"
                                                };
                                                let mem_str = fmt_bytes(k.mem_bytes);
                                                let key_clone = k.key.clone();
                                                view! {
                                                    <tr class="border-b border-line/40 hover:bg-teal-500/5 transition-colors group">
                                                        <td class="px-5 py-2.5 font-mono text-xs text-fg/80 max-w-xs truncate">{k.key}</td>
                                                        <td class="px-5 py-2.5 text-right text-muted tabular-nums">{mem_str}</td>
                                                        <td class={format!("px-5 py-2.5 text-right tabular-nums {ttl_color}")}>{ttl_str}</td>
                                                        <td class="px-3 py-2.5 text-right">
                                                            <button
                                                                on:click=move |_| { delete_action.dispatch(key_clone.clone()); }
                                                                class="opacity-0 group-hover:opacity-100 w-6 h-6 flex items-center justify-center rounded text-muted hover:text-red-400 hover:bg-red-500/10 transition-all cursor-pointer"
                                                                title="Delete key"
                                                            >
                                                                <i class="bi bi-trash3 text-xs"></i>
                                                            </button>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any(),
                        })}
                    </Suspense>
                </div>

            </div>
        </AdminLayout>
    }
}
