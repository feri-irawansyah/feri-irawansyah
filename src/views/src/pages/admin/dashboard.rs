use leptos::prelude::*;
use modules::auth::Claims;

use super::layout::AdminLayout;

#[server]
pub async fn get_admin_session() -> Result<Claims, ServerFnError> {
    crate::pages::admin::require_admin().await
}

/// Public DTOs crossing the `#[server]` fn boundary — the actual GA4 fetching
/// logic lives in `crate::features::analytics`, which is ssr-only and so
/// can't hold a type the client build also needs to know about.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DailyStat {
    pub date: String,
    pub active_users: i64,
    pub page_views: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TopPage {
    pub path: String,
    pub title: String,
    pub views: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrafficSource {
    pub source: String,
    pub medium: String,
    pub sessions: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeviceStat {
    pub device: String,
    pub active_users: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CountryStat {
    pub country: String,
    pub active_users: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EngagementSummary {
    pub bounce_rate: f64,
    pub avg_session_duration: f64,
    pub engagement_rate: f64,
    pub new_users: i64,
}

#[server]
pub async fn admin_get_visitor_stats() -> Result<Vec<DailyStat>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let rows = crate::features::analytics::fetch_daily_stats(30)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows
        .into_iter()
        .map(|(date, active_users, page_views)| DailyStat {
            date,
            active_users,
            page_views,
        })
        .collect())
}

#[server]
pub async fn admin_get_realtime_users() -> Result<i64, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    crate::features::analytics::fetch_realtime_active_users()
        .await
        .map_err(ServerFnError::new)
}

#[server]
pub async fn admin_get_engagement_summary() -> Result<EngagementSummary, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let (bounce_rate, avg_session_duration, engagement_rate, new_users) =
        crate::features::analytics::fetch_engagement_summary(30)
            .await
            .map_err(ServerFnError::new)?;
    Ok(EngagementSummary {
        bounce_rate,
        avg_session_duration,
        engagement_rate,
        new_users,
    })
}

#[server]
pub async fn admin_get_top_pages() -> Result<Vec<TopPage>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let rows = crate::features::analytics::fetch_top_pages(30, 8)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows
        .into_iter()
        .map(|(path, title, views)| TopPage { path, title, views })
        .collect())
}

#[server]
pub async fn admin_get_traffic_sources() -> Result<Vec<TrafficSource>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let rows = crate::features::analytics::fetch_traffic_sources(30, 8)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows
        .into_iter()
        .map(|(source, medium, sessions)| TrafficSource {
            source,
            medium,
            sessions,
        })
        .collect())
}

#[server]
pub async fn admin_get_device_breakdown() -> Result<Vec<DeviceStat>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let rows = crate::features::analytics::fetch_device_breakdown(30)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows
        .into_iter()
        .map(|(device, active_users)| DeviceStat {
            device,
            active_users,
        })
        .collect())
}

#[server]
pub async fn admin_get_geo_breakdown() -> Result<Vec<CountryStat>, ServerFnError> {
    crate::pages::admin::require_admin().await?;
    let rows = crate::features::analytics::fetch_geo_breakdown(30, 8)
        .await
        .map_err(ServerFnError::new)?;
    Ok(rows
        .into_iter()
        .map(|(country, active_users)| CountryStat {
            country,
            active_users,
        })
        .collect())
}

#[allow(non_snake_case)]
#[component]
pub fn AdminDashboard() -> impl IntoView {
    let session = Resource::new_blocking(|| (), |_| get_admin_session());
    let visitor_stats = Resource::new(|| (), |_| admin_get_visitor_stats());
    let realtime_users = Resource::new(|| (), |_| admin_get_realtime_users());
    let engagement = Resource::new(|| (), |_| admin_get_engagement_summary());
    let top_pages = Resource::new(|| (), |_| admin_get_top_pages());
    let traffic_sources = Resource::new(|| (), |_| admin_get_traffic_sources());
    let device_breakdown = Resource::new(|| (), |_| admin_get_device_breakdown());
    let geo_breakdown = Resource::new(|| (), |_| admin_get_geo_breakdown());

    view! {
        <AdminLayout>
            <Suspense fallback=|| view! {
                <div class="p-8">
                    <div class="animate-pulse space-y-4">
                        <div class="h-8 w-48 bg-line rounded-lg"></div>
                        <div class="h-4 w-64 bg-line rounded"></div>
                    </div>
                </div>
            }>
                {move || session.get().map(|result| match result {
                    Ok(claims) => view! {
                        <div class="p-8 max-w-7xl">

                            // Greeting
                            <div class="mb-10 flex flex-col lg:flex-row lg:items-center lg:justify-between gap-6">
                                <div>
                                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                                        "Panel Admin"
                                    </span>
                                    <h2 class="text-3xl font-extrabold text-fg mb-1">"Selamat datang kembali!"</h2>
                                    <p class="text-muted text-sm">{claims.email}</p>
                                </div>

                                <div class="w-full lg:w-auto">
                                    <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-3 lg:text-right">
                                        "Engagement · 30 Hari Terakhir"
                                    </p>
                                    <Suspense fallback=|| view! {
                                        <div class="h-20 w-full lg:w-105 rounded-2xl bg-line/30 animate-pulse"></div>
                                    }>
                                        {move || engagement.get().map(|r| match r {
                                            Ok(e) => view! { <EngagementCard summary=e/> }.into_any(),
                                            Err(e) => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-4 text-center text-muted text-sm">
                                                    "Engagement belum bisa dimuat: " {e.to_string()}
                                                </div>
                                            }.into_any(),
                                        })}
                                    </Suspense>
                                </div>
                            </div>

                            // Stats grid — about-page card style
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-10">
                                <div class="bg-surface border border-line rounded-2xl p-6 hover:border-teal-500/50 transition-colors">
                                    <div class="w-10 h-10 rounded-xl bg-teal-500/15 flex items-center justify-center mb-4">
                                        <i class="bi bi-shield-check text-teal-500 text-lg"></i>
                                    </div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-wider mb-1">"Status Sesi"</p>
                                    <p class="text-2xl font-bold text-fg">"Aktif"</p>
                                    <p class="text-xs text-muted mt-1">"Token valid · admin"</p>
                                </div>

                                <div class="bg-surface border border-line rounded-2xl p-6 hover:border-teal-500/50 transition-colors">
                                    <div class="w-10 h-10 rounded-xl bg-teal-500/15 flex items-center justify-center mb-4">
                                        <i class="bi bi-broadcast text-teal-500 text-lg"></i>
                                    </div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-wider mb-1">"Pengunjung Sekarang"</p>
                                    <p class="text-2xl font-bold text-fg">
                                        <Suspense fallback=|| view! { "—" }>
                                            {move || realtime_users.get().map(|r| match r {
                                                Ok(n) => n.to_string(),
                                                Err(_) => "—".to_string(),
                                            })}
                                        </Suspense>
                                    </p>
                                    <p class="text-xs text-muted mt-1">"Realtime · Google Analytics"</p>
                                </div>

                                <div class="bg-surface border border-line rounded-2xl p-6 hover:border-teal-500/50 transition-colors">
                                    <div class="w-10 h-10 rounded-xl bg-teal-500/15 flex items-center justify-center mb-4">
                                        <i class="bi bi-eye text-teal-500 text-lg"></i>
                                    </div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-wider mb-1">"Page Views"</p>
                                    <p class="text-2xl font-bold text-fg">
                                        <Suspense fallback=|| view! { "—" }>
                                            {move || visitor_stats.get().map(|r| match r {
                                                Ok(stats) => stats.iter().map(|s| s.page_views).sum::<i64>().to_string(),
                                                Err(_) => "—".to_string(),
                                            })}
                                        </Suspense>
                                    </p>
                                    <p class="text-xs text-muted mt-1">"30 hari terakhir"</p>
                                </div>
                            </div>

                            // Visitor analytics
                            <div class="mb-10">
                                <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-4">
                                    "Visitor Analytics · 30 Hari Terakhir"
                                </p>
                                <Suspense fallback=|| view! {
                                    <div class="h-56 rounded-2xl bg-line/30 animate-pulse"></div>
                                }>
                                    {move || visitor_stats.get().map(|r| match r {
                                        Ok(stats) if stats.is_empty() => view! {
                                            <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                "Belum ada data visitor."
                                            </div>
                                        }.into_any(),
                                        Ok(stats) => view! { <VisitorChart stats=stats/> }.into_any(),
                                        Err(e) => view! {
                                            <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                <i class="bi bi-exclamation-triangle text-teal-500 mr-1.5"></i>
                                                "Analytics belum bisa dimuat: " {e.to_string()}
                                            </div>
                                        }.into_any(),
                                    })}
                                </Suspense>
                            </div>

                            // Top pages + traffic sources
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-10">
                                <div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-4">
                                        "Halaman Terpopuler"
                                    </p>
                                    <Suspense fallback=|| view! {
                                        <div class="h-56 rounded-2xl bg-line/30 animate-pulse"></div>
                                    }>
                                        {move || top_pages.get().map(|r| match r {
                                            Ok(rows) if rows.is_empty() => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum ada data."
                                                </div>
                                            }.into_any(),
                                            Ok(rows) => view! { <TopPagesCard pages=rows/> }.into_any(),
                                            Err(e) => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum bisa dimuat: " {e.to_string()}
                                                </div>
                                            }.into_any(),
                                        })}
                                    </Suspense>
                                </div>

                                <div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-4">
                                        "Sumber Traffic"
                                    </p>
                                    <Suspense fallback=|| view! {
                                        <div class="h-56 rounded-2xl bg-line/30 animate-pulse"></div>
                                    }>
                                        {move || traffic_sources.get().map(|r| match r {
                                            Ok(rows) if rows.is_empty() => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum ada data."
                                                </div>
                                            }.into_any(),
                                            Ok(rows) => view! { <TrafficSourcesCard sources=rows/> }.into_any(),
                                            Err(e) => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum bisa dimuat: " {e.to_string()}
                                                </div>
                                            }.into_any(),
                                        })}
                                    </Suspense>
                                </div>
                            </div>

                            // Device + geo breakdown
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-10">
                                <div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-4">
                                        "Perangkat"
                                    </p>
                                    <Suspense fallback=|| view! {
                                        <div class="h-40 rounded-2xl bg-line/30 animate-pulse"></div>
                                    }>
                                        {move || device_breakdown.get().map(|r| match r {
                                            Ok(rows) if rows.is_empty() => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum ada data."
                                                </div>
                                            }.into_any(),
                                            Ok(rows) => view! { <DeviceBreakdownCard devices=rows/> }.into_any(),
                                            Err(e) => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum bisa dimuat: " {e.to_string()}
                                                </div>
                                            }.into_any(),
                                        })}
                                    </Suspense>
                                </div>

                                <div>
                                    <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-4">
                                        "Negara Pengunjung"
                                    </p>
                                    <Suspense fallback=|| view! {
                                        <div class="h-40 rounded-2xl bg-line/30 animate-pulse"></div>
                                    }>
                                        {move || geo_breakdown.get().map(|r| match r {
                                            Ok(rows) if rows.is_empty() => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum ada data."
                                                </div>
                                            }.into_any(),
                                            Ok(rows) => view! { <GeoBreakdownCard countries=rows/> }.into_any(),
                                            Err(e) => view! {
                                                <div class="bg-surface border border-line rounded-2xl p-6 text-center text-muted text-sm">
                                                    "Belum bisa dimuat: " {e.to_string()}
                                                </div>
                                            }.into_any(),
                                        })}
                                    </Suspense>
                                </div>
                            </div>

                            // Quick nav — about explore-more style
                            <div>
                                <p class="text-xs font-semibold text-muted uppercase tracking-widest mb-4">"Akses Cepat"</p>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                                    <a href="/admin/users"
                                        class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-4 hover:border-teal-500/50 transition-colors no-underline">
                                        <div class="w-10 h-10 rounded-lg bg-teal-500/15 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                            <i class="bi bi-people-fill text-teal-500"></i>
                                        </div>
                                        <div>
                                            <p class="text-sm font-semibold text-fg">"Manajemen User"</p>
                                            <p class="text-xs text-muted">"Lihat & kelola semua user"</p>
                                        </div>
                                        <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                                    </a>

                                    <a href="/"
                                        class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-4 hover:border-teal-500/50 transition-colors no-underline">
                                        <div class="w-10 h-10 rounded-lg bg-teal-500/15 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                            <i class="bi bi-globe text-teal-500"></i>
                                        </div>
                                        <div>
                                            <p class="text-sm font-semibold text-fg">"Lihat Website"</p>
                                            <p class="text-xs text-muted">"Buka halaman publik"</p>
                                        </div>
                                        <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                                    </a>
                                </div>
                            </div>

                        </div>
                    }.into_any(),
                    Err(_) => view! {
                        <div class="p-8">
                            <p class="text-muted text-sm">"Mengalihkan..."</p>
                        </div>
                    }.into_any(),
                })}
            </Suspense>
        </AdminLayout>
    }
}

#[allow(non_snake_case)]
#[component]
fn VisitorChart(stats: Vec<DailyStat>) -> impl IntoView {
    let total_users: i64 = stats.iter().map(|s| s.active_users).sum();
    let total_views: i64 = stats.iter().map(|s| s.page_views).sum();

    // ── SVG line chart geometry (all precomputed as plain values/strings) ──
    let width = 600.0_f64;
    let height = 180.0_f64;
    let pad_top = 10.0_f64;
    let pad_left = 28.0_f64;
    let pad_right = 10.0_f64;
    let pad_bottom = 22.0_f64;
    let plot_w = width - pad_left - pad_right;
    let plot_h = height - pad_top - pad_bottom;

    let n = stats.len();
    let max_val = stats
        .iter()
        .map(|s| s.active_users)
        .max()
        .unwrap_or(1)
        .max(1) as f64;

    let points: Vec<(f64, f64)> = stats
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let x = pad_left
                + if n > 1 {
                    i as f64 / (n - 1) as f64 * plot_w
                } else {
                    plot_w / 2.0
                };
            let y = pad_top + plot_h - (s.active_users as f64 / max_val * plot_h);
            (x, y)
        })
        .collect();

    let polyline = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");

    let gridlines = (0..=3)
        .map(|i| {
            let y = pad_top + plot_h * (i as f64 / 3.0);
            view! {
                <line x1=pad_left.to_string() y1=y.to_string() x2=(width - pad_right).to_string() y2=y.to_string()
                    class="stroke-line" stroke-width="1"/>
            }
        })
        .collect_view();

    let dots = points
        .iter()
        .zip(stats.iter())
        .map(|((x, y), s)| {
            let title = format!(
                "{}: {} pengguna aktif · {} tayangan",
                s.date, s.active_users, s.page_views
            );
            view! {
                <circle cx=x.to_string() cy=y.to_string() r="3" class="fill-teal-500">
                    <title>{title}</title>
                </circle>
            }
        })
        .collect_view();

    let label_step = (n / 6).max(1);
    let labels = stats
        .iter()
        .enumerate()
        .filter(|(i, _)| i % label_step == 0)
        .map(|(i, s)| {
            let x = points[i].0;
            view! {
                <text x=x.to_string() y=(height - 4.0).to_string() class="fill-muted" font-size="9" text-anchor="middle">
                    {s.date.clone()}
                </text>
            }
        })
        .collect_view();

    view! {
        <div class="bg-surface border border-line rounded-2xl p-6">
            <div class="grid grid-cols-2 gap-6 mb-5">
                <div>
                    <p class="text-xs font-semibold text-muted uppercase tracking-wider mb-1">"Active Users"</p>
                    <p class="text-2xl font-bold text-fg">{total_users.to_string()}</p>
                </div>
                <div>
                    <p class="text-xs font-semibold text-muted uppercase tracking-wider mb-1">"Page Views"</p>
                    <p class="text-2xl font-bold text-fg">{total_views.to_string()}</p>
                </div>
            </div>
            <div class="w-full overflow-x-auto">
                <svg viewBox=format!("0 0 {width} {height}") class="w-full h-44" preserveAspectRatio="none">
                    {gridlines}
                    <polyline points=polyline fill="none" class="stroke-teal-500" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round"/>
                    {dots}
                    {labels}
                </svg>
            </div>
        </div>
    }
}

/// Formats a seconds duration (GA4's `averageSessionDuration`) as `"1m 24s"`.
fn format_duration(secs: f64) -> String {
    let total = secs.round().max(0.0) as i64;
    format!("{}m {}s", total / 60, total % 60)
}

#[allow(non_snake_case)]
#[component]
fn EngagementCard(summary: EngagementSummary) -> impl IntoView {
    view! {
        <div class="bg-surface border border-line rounded-2xl p-4 grid grid-cols-2 sm:grid-cols-4 gap-4 lg:w-105">
            <div>
                <p class="text-[11px] font-semibold text-muted uppercase tracking-wider mb-1">"Bounce Rate"</p>
                <p class="text-lg font-bold text-fg">{format!("{:.0}%", summary.bounce_rate * 100.0)}</p>
            </div>
            <div>
                <p class="text-[11px] font-semibold text-muted uppercase tracking-wider mb-1">"Durasi Sesi"</p>
                <p class="text-lg font-bold text-fg">{format_duration(summary.avg_session_duration)}</p>
            </div>
            <div>
                <p class="text-[11px] font-semibold text-muted uppercase tracking-wider mb-1">"Engagement"</p>
                <p class="text-lg font-bold text-fg">{format!("{:.0}%", summary.engagement_rate * 100.0)}</p>
            </div>
            <div>
                <p class="text-[11px] font-semibold text-muted uppercase tracking-wider mb-1">"New Users"</p>
                <p class="text-lg font-bold text-fg">{summary.new_users.to_string()}</p>
            </div>
        </div>
    }
}

/// A ranked row with an optional sublabel and a proportional background bar —
/// shared by the Top Pages / Traffic Sources list cards below.
fn bar_row(
    rank: usize,
    label: String,
    sublabel: Option<String>,
    value: i64,
    max: i64,
) -> impl IntoView {
    let pct = if max > 0 {
        (value as f64 / max as f64 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    view! {
        <div class="relative">
            <div class="absolute inset-y-0 left-0 bg-teal-500/10 rounded-md" style=format!("width: {pct}%")></div>
            <div class="relative flex items-center justify-between gap-3 px-3 py-2.5">
                <div class="flex items-center gap-2.5 min-w-0">
                    <span class="text-xs text-muted w-4 shrink-0">{rank.to_string()}</span>
                    <div class="min-w-0">
                        <p class="text-sm text-fg truncate">{label}</p>
                        {sublabel.map(|s| view! { <p class="text-xs text-muted truncate">{s}</p> })}
                    </div>
                </div>
                <span class="text-sm font-semibold text-fg shrink-0">{value.to_string()}</span>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn TopPagesCard(pages: Vec<TopPage>) -> impl IntoView {
    let max = pages.iter().map(|p| p.views).max().unwrap_or(1).max(1);
    let rows = pages
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            let label = if p.title.is_empty() {
                p.path.clone()
            } else {
                p.title
            };
            bar_row(i + 1, label, Some(p.path), p.views, max)
        })
        .collect_view();
    view! {
        <div class="bg-surface border border-line rounded-2xl divide-y divide-line/60 overflow-hidden">
            {rows}
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn TrafficSourcesCard(sources: Vec<TrafficSource>) -> impl IntoView {
    let max = sources.iter().map(|s| s.sessions).max().unwrap_or(1).max(1);
    let rows = sources
        .into_iter()
        .enumerate()
        .map(|(i, s)| bar_row(i + 1, s.source, Some(s.medium), s.sessions, max))
        .collect_view();
    view! {
        <div class="bg-surface border border-line rounded-2xl divide-y divide-line/60 overflow-hidden">
            {rows}
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn DeviceBreakdownCard(devices: Vec<DeviceStat>) -> impl IntoView {
    let max = devices
        .iter()
        .map(|d| d.active_users)
        .max()
        .unwrap_or(1)
        .max(1);
    let rows = devices
        .into_iter()
        .map(|d| {
            let icon = match d.device.to_lowercase().as_str() {
                "desktop" => "bi-display",
                "mobile" => "bi-phone",
                "tablet" => "bi-tablet",
                _ => "bi-question-circle",
            };
            let pct = (d.active_users as f64 / max as f64 * 100.0).clamp(0.0, 100.0);
            view! {
                <div class="flex items-center gap-3">
                    <i class=format!("bi {icon} text-teal-500 text-sm w-5 text-center shrink-0")></i>
                    <span class="text-sm text-fg w-20 shrink-0 capitalize">{d.device}</span>
                    <div class="flex-1 h-2 bg-line rounded-full overflow-hidden">
                        <div class="h-full bg-teal-500 rounded-full" style=format!("width: {pct}%")></div>
                    </div>
                    <span class="text-sm font-semibold text-fg w-10 text-right shrink-0">{d.active_users.to_string()}</span>
                </div>
            }
        })
        .collect_view();
    view! {
        <div class="bg-surface border border-line rounded-2xl p-6 space-y-3.5">
            {rows}
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn GeoBreakdownCard(countries: Vec<CountryStat>) -> impl IntoView {
    let max = countries
        .iter()
        .map(|c| c.active_users)
        .max()
        .unwrap_or(1)
        .max(1);
    let rows = countries
        .into_iter()
        .enumerate()
        .map(|(i, c)| bar_row(i + 1, c.country, None, c.active_users, max))
        .collect_view();
    view! {
        <div class="bg-surface border border-line rounded-2xl divide-y divide-line/60 overflow-hidden">
            {rows}
        </div>
    }
}
