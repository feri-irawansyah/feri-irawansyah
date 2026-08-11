use crate::components::{CertCardSkeleton, TimelineCardSkeleton};
use crate::i18n::*;
use crate::seo::Seo;
use leptos::prelude::*;
use modules::certifications::CertView;
use modules::positions::PositionView;

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
async fn cert_svc()
-> Result<std::sync::Arc<dyn modules::certifications::CertService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::certifications::CertService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn get_all_positions() -> Result<Vec<PositionView>, ServerFnError> {
    position_svc()
        .await?
        .find_all_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_all_certifications() -> Result<Vec<CertView>, ServerFnError> {
    cert_svc()
        .await?
        .find_all_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[allow(non_snake_case)]
#[component]
pub fn ExperiencePage() -> impl IntoView {
    let i18n = use_i18n();
    let positions = Resource::new(|| (), |_| get_all_positions());
    let certs = Resource::new(|| (), |_| get_all_certifications());

    view! {
        <Seo
            title="Experience — Feri Irawansyah"
            description="Work history and certifications of Feri Irawansyah — Principal Engineer with a track record in Rust, backend, and infrastructure roles."
            path="/experience"
        />
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        {t!(i18n, experience.eyebrow)}
                    </span>
                    <h1 class="text-[1.85rem] sm:text-[2.25rem] font-extrabold text-fg mb-2">{t!(i18n, experience.title)}</h1>
                    <p class="text-muted text-[1.05rem]">
                        {t!(i18n, experience.subtitle)}
                    </p>
                </header>

                // ── Work History ────────────────────────────────────────────────
                <section class="mb-16">
                    <h2 class="text-2xl font-bold mb-8 text-fg">{t!(i18n, experience.work_history)}</h2>
                    <Suspense fallback=|| view! { <TimelineCardSkeleton count=3 /> }>
                        {move || positions.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="text-center text-muted py-12">{t!(i18n, experience.empty_work)}</p>
                            }.into_any(),
                            Ok(items) => {
                                // Group positions by their parent experience (company) so
                                // multiple title changes at the same company render as
                                // sub-entries of one card instead of separate cards.
                                let mut order: Vec<i32> = Vec::new();
                                let mut groups: std::collections::HashMap<i32, Vec<_>> =
                                    std::collections::HashMap::new();
                                for p in items {
                                    groups.entry(p.experience_id).or_insert_with(|| {
                                        order.push(p.experience_id);
                                        Vec::new()
                                    }).push(p);
                                }
                                let grouped: Vec<Vec<_>> = order
                                    .into_iter()
                                    .filter_map(|id| groups.remove(&id))
                                    .collect();

                                view! {
                                    <div class="flex flex-col">
                                        {grouped.into_iter().map(|group| {
                                            let head = group[0].clone();
                                            let is_current = group.iter().any(|p| p.end_date.is_none());
                                            let earliest_start = group.iter().map(|p| p.start_date).min().unwrap();
                                            let overall_period = if is_current {
                                                format!("{} – {}", earliest_start.format("%b %Y"), t_string!(i18n, experience.present))
                                            } else {
                                                let latest_end = group.iter().filter_map(|p| p.end_date).max().unwrap();
                                                format!("{} – {}", earliest_start.format("%b %Y"), latest_end.format("%b %Y"))
                                            };
                                            let dot_class = if is_current {
                                                "w-4 h-4 rounded-full bg-teal-500 ring-4 ring-teal-500/20 border-[3px] border-base shrink-0 mt-1 z-10"
                                            } else {
                                                "w-4 h-4 rounded-full bg-line border-[3px] border-base shrink-0 mt-1 z-10"
                                            };

                                            view! {
                                                <div class="relative flex gap-3 sm:gap-6 pb-8 before:content-[''] before:absolute before:left-1.75 before:top-5 before:bottom-0 before:w-0.5 before:bg-line last:before:hidden">
                                                    <div class=dot_class></div>
                                                    <div class="flex-1 min-w-0 bg-surface border border-line rounded-2xl p-4 sm:p-6 hover:border-teal-500/40 transition-colors">

                                                        // Company header
                                                        <div class="flex flex-wrap items-center gap-2 mb-1">
                                                            {(!head.image_src.is_empty()).then(|| view! {
                                                                <div class="w-7 h-7 rounded-sm bg-white/90 flex items-center justify-center shrink-0 overflow-hidden">
                                                                    <img src=head.image_src.clone() alt=head.company.clone()
                                                                        class="w-full h-full object-contain"/>
                                                                </div>
                                                            })}
                                                            <h3 class="text-[1.05rem] font-bold text-fg">{head.company.clone()}</h3>
                                                            {is_current.then(|| view! {
                                                                <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-semibold bg-green-500/15 text-green-500 whitespace-nowrap">
                                                                    <span class="w-1.5 h-1.5 rounded-full bg-green-400 animate-pulse"></span>
                                                                    {t!(i18n, experience.current_badge)}
                                                                </span>
                                                            })}
                                                        </div>
                                                        <p class="text-xs text-muted mb-5">
                                                            <i class="bi bi-calendar3 mr-1"></i>
                                                            {overall_period}
                                                        </p>

                                                        // Sub: individual positions/title changes at this company
                                                        <div class="flex flex-col gap-5 pl-5 border-l-2 border-line ml-1">
                                                            {group.into_iter().map(|p| {
                                                                let sub_current = p.end_date.is_none();
                                                                let period = match p.end_date {
                                                                    Some(end) => format!(
                                                                        "{} – {}",
                                                                        p.start_date.format("%b %Y"),
                                                                        end.format("%b %Y")
                                                                    ),
                                                                    None => format!("{} – {}", p.start_date.format("%b %Y"), t_string!(i18n, experience.present)),
                                                                };
                                                                let desc = p.description.clone();
                                                                let sub_dot_class = if sub_current {
                                                                    "absolute -left-[25px] top-1.5 w-2.5 h-2.5 rounded-full bg-teal-500 border-2 border-surface"
                                                                } else {
                                                                    "absolute -left-[25px] top-1.5 w-2.5 h-2.5 rounded-full bg-line border-2 border-surface"
                                                                };
                                                                view! {
                                                                    <div class="relative min-w-0">
                                                                        <span class=sub_dot_class></span>
                                                                        <h4 class="text-[0.95rem] font-semibold text-fg mb-1.5">{p.title}</h4>
                                                                        <div class="flex flex-wrap items-center gap-2 mb-2">
                                                                            <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium bg-teal-500/10 text-teal-500">
                                                                                {p.job_position.clone()}
                                                                            </span>
                                                                            <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium bg-line text-muted">
                                                                                {p.job_type.clone()}
                                                                            </span>
                                                                        </div>
                                                                        <div class="text-xs text-muted mb-2 flex flex-col gap-0.5">
                                                                            <span><i class="bi bi-calendar3 mr-1"></i>{period}</span>
                                                                            {(!p.address.is_empty()).then(|| view! {
                                                                                <span><i class="bi bi-geo-alt mr-1"></i>{p.address.clone()}</span>
                                                                            })}
                                                                        </div>
                                                                        {(!desc.is_empty()).then(|| view! {
                                                                            <ul class="list-disc list-inside space-y-1">
                                                                                {desc.into_iter().map(|item| view! {
                                                                                    <li class="text-[0.875rem] text-muted leading-relaxed">{item}</li>
                                                                                }).collect_view()}
                                                                            </ul>
                                                                        })}
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            },
                            Err(e) => view! {
                                <p class="text-red-400 py-4">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </section>

                // ── Certifications ──────────────────────────────────────────────
                <section class="mb-16">
                    <h2 class="text-2xl font-bold mb-8 text-fg">{t!(i18n, experience.certifications)}</h2>
                    <Suspense fallback=|| view! { <CertCardSkeleton count=6 /> }>
                        {move || certs.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="text-center text-muted py-12">{t!(i18n, experience.empty_certs)}</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="grid grid-cols-[repeat(auto-fill,minmax(260px,1fr))] gap-4">
                                    {items.into_iter().map(|c| view! {
                                        <div class="group bg-surface border border-line rounded-2xl p-5 flex gap-4 items-start hover:border-teal-500/50 hover:-translate-y-0.5 transition-all duration-200">
                                            {if !c.image_src.is_empty() {
                                                view! {
                                                    <div class="w-12 h-12 rounded-lg bg-white/90 flex items-center justify-center shrink-0 overflow-hidden">
                                                        <img src=c.image_src.clone() alt=c.title.clone()
                                                            class="w-full h-full object-contain"/>
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="w-12 h-12 rounded-lg bg-teal-500/15 flex items-center justify-center shrink-0">
                                                        <i class="bi bi-patch-check-fill text-teal-500 text-lg"></i>
                                                    </div>
                                                }.into_any()
                                            }}
                                            <div class="flex-1 min-w-0">
                                                <h3 class="text-[0.95rem] font-bold text-fg mb-0.5 group-hover:text-teal-500 transition-colors">{c.title}</h3>
                                                <p class="text-sm text-muted line-clamp-2">{c.description.clone()}</p>
                                                <p class="text-xs text-muted mt-1.5 mb-3">
                                                    <i class="bi bi-calendar3 mr-1"></i>
                                                    {c.start_date.format("%b %Y").to_string()}
                                                </p>
                                                {(!c.url_docs.is_empty()).then(|| view! {
                                                    <a href=c.url_docs.clone() target="_blank"
                                                        class="inline-flex items-center gap-1.5 text-[0.85rem] font-semibold text-teal-500 hover:gap-2.5 transition-all no-underline whitespace-nowrap">
                                                        {t!(i18n, experience.view_certificate)}
                                                        <i class="bi bi-arrow-right text-[0.8rem]"></i>
                                                    </a>
                                                })}
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="text-red-400 py-4">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </section>
            </div>
        </div>
    }
}
