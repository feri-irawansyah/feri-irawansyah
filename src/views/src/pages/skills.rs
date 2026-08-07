use crate::components::SkillCardSkeleton;
use crate::i18n::*;
use crate::seo::Seo;
use leptos::prelude::*;
use modules::skills::SkillView;

#[cfg(feature = "ssr")]
async fn skill_svc() -> Result<std::sync::Arc<dyn modules::skills::SkillService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::skills::SkillService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn get_all_skills() -> Result<Vec<SkillView>, ServerFnError> {
    skill_svc()
        .await?
        .list()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

const PAGE_SIZE: usize = 9;

#[allow(non_snake_case)]
#[component]
pub fn SkillsPage() -> impl IntoView {
    let i18n = use_i18n();
    let skills = Resource::new(|| (), |_| get_all_skills());
    let (page, set_page) = signal(0usize);

    let prev_disabled = Memo::new(move |_| page.get() == 0);
    let next_disabled = Memo::new(move |_| {
        skills
            .get()
            .and_then(|r| r.ok())
            .map(|items| (page.get() + 1) * PAGE_SIZE >= items.len())
            .unwrap_or(true)
    });

    view! {
        <Seo
            title="Tech Stack — Feri Irawansyah"
            description="Technologies and tools Feri Irawansyah uses to build robust applications — Rust, Leptos, PostgreSQL, and more."
            path="/skills"
        />
        <div class="py-4">
            <div class="max-w-6xl mx-auto px-6 md:px-12">
                <div class="grid grid-cols-1 lg:grid-cols-[360px_1fr] gap-14 items-start py-16">

                    // ── Left: heading + nav ─────────────────────────────
                    <div class="lg:sticky lg:top-24">
                        <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-4 block">
                            {t!(i18n, skills.eyebrow)}
                        </span>
                        <h1 class="text-[2.25rem] font-extrabold text-fg leading-tight mb-5">
                            {t!(i18n, skills.title_before)}
                            <span class="text-teal-500">{t!(i18n, skills.title_highlight)}</span>
                        </h1>
                        <p class="text-muted leading-relaxed max-w-prose mb-10">
                            {t!(i18n, skills.subtitle)}
                        </p>

                        // Prev / Next
                        <div class="flex items-center gap-3">
                            <button
                                on:click=move |_| set_page.update(|p| *p = p.saturating_sub(1))
                                prop:disabled=prev_disabled
                                class="w-10 h-10 rounded-full border cursor-pointer border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
                                <i class="bi bi-arrow-left"></i>
                            </button>
                            <button
                                on:click=move |_| {
                                    let max_page = skills.get()
                                        .and_then(|r| r.ok())
                                        .map(|items| items.len().saturating_sub(1) / PAGE_SIZE)
                                        .unwrap_or(0);
                                    set_page.update(|p| *p = (*p + 1).min(max_page));
                                }
                                prop:disabled=next_disabled
                                class="w-10 h-10 rounded-full border cursor-pointer border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
                                <i class="bi bi-arrow-right"></i>
                            </button>
                        </div>
                    </div>

                    // ── Right: cards ─────────────────────────────────────
                    <div>
                        <Suspense fallback=|| view! { <SkillCardSkeleton count=9 /> }>
                            {move || skills.get().map(|r| match r {
                                Ok(items) if items.is_empty() => view! {
                                    <p class="text-center text-muted py-16">{t!(i18n, skills.empty)}</p>
                                }.into_any(),
                                Ok(items) => {
                                    let start = page.get() * PAGE_SIZE;
                                    let visible: Vec<_> = items.into_iter().skip(start).take(PAGE_SIZE).collect();
                                    view! {
                                        <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-5">
                                            {visible.into_iter().map(|s| {
                                                let pct = s.progress.min(100);
                                                view! {
                                                    <div
                                                        class="group bg-surface border border-line rounded-xl p-6 flex flex-col items-center gap-4 hover:border-teal-500 hover:-translate-y-0.5 transition-all duration-200"
                                                        title=format!("{} — {}%", s.title.clone(), pct)
                                                    >
                                                        <div class="w-full flex items-center justify-between">
                                                            <div class="flex items-center gap-0.5">
                                                                {(1..=5).map(|i| {
                                                                    view! {
                                                                        <i class=if i <= s.star {
                                                                            "bi bi-star-fill text-amber-400 text-[0.65rem]"
                                                                        } else {
                                                                            "bi bi-star text-muted/30 text-[0.65rem]"
                                                                        }></i>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                            {(s.star >= 4).then(|| view! {
                                                                <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-amber-400/15 text-amber-400 text-[0.6rem] font-semibold whitespace-nowrap">
                                                                    <i class="bi bi-star-fill text-[0.55rem]"></i>
                                                                    {t!(i18n, skills.favorite)}
                                                                </span>
                                                            })}
                                                        </div>
                                                        {(!s.image_src.is_empty()).then(|| view! {
                                                            <div class="w-14 h-14 rounded-xl bg-white shadow-sm flex items-center justify-center p-2">
                                                                <img src=s.image_src.clone() alt=s.title.clone()
                                                                    class="w-full h-full object-contain"/>
                                                            </div>
                                                        })}
                                                        <p class="text-[0.9rem] font-semibold text-fg text-center">{s.title.clone()}</p>
                                                        <div class="w-full h-2 rounded-full bg-base overflow-hidden">
                                                            <div
                                                                class="h-full rounded-full bg-teal-600 group-hover:bg-teal-400 transition-all duration-500"
                                                                style=format!("width: {}%", pct)
                                                            ></div>
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
                    </div>

                </div>
            </div>
        </div>
    }
}
