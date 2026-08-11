use crate::components::SkillCardSkeleton;
use crate::i18n::*;
use crate::seo::Seo;
use leptos::prelude::*;
use leptos_i18n::I18nContext;
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
        .find_all_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Groups the tech-stack grid by the manually-set `star` rating (0-5) rather
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SkillTier {
    Favorite,
    Familiar,
    UsedBefore,
}

impl SkillTier {
    const ALL: [SkillTier; 3] = [SkillTier::Favorite, SkillTier::Familiar, SkillTier::UsedBefore];

    /// Boundaries: star == 5 favorite, star == 4 familiar, star < 4 used-before.
    pub(crate) fn for_star(star: i32) -> Self {
        if star >= 5 {
            SkillTier::Favorite
        } else if star >= 4 {
            SkillTier::Familiar
        } else {
            SkillTier::UsedBefore
        }
    }

    fn anchor(self) -> &'static str {
        match self {
            SkillTier::Favorite => "tier-favorite",
            SkillTier::Familiar => "tier-familiar",
            SkillTier::UsedBefore => "tier-used-before",
        }
    }
}

/// Splits `items` into non-empty `(tier, skills)` groups, tier order fixed
/// (favorite → familiar → used-before) regardless of input order.
pub(crate) fn group_by_tier(items: &[SkillView]) -> Vec<(SkillTier, Vec<SkillView>)> {
    SkillTier::ALL
        .into_iter()
        .map(|tier| {
            let group: Vec<SkillView> = items
                .iter()
                .filter(|s| SkillTier::for_star(s.star) == tier)
                .cloned()
                .collect();
            (tier, group)
        })
        .filter(|(_, group)| !group.is_empty())
        .collect()
}

fn tier_label(i18n: I18nContext<Locale>, tier: SkillTier) -> impl IntoView {
    match tier {
        SkillTier::Favorite => t!(i18n, skills.favorite).into_any(),
        SkillTier::Familiar => t!(i18n, skills.familiar).into_any(),
        SkillTier::UsedBefore => t!(i18n, skills.used_before).into_any(),
    }
}

#[allow(non_snake_case)]
#[component]
pub fn SkillsPage() -> impl IntoView {
    let i18n = use_i18n();
    let skills = Resource::new(|| (), |_| get_all_skills());

    view! {
        <Seo
            title="Tech Stack — Feri Irawansyah"
            description="Technologies and tools Feri Irawansyah uses to build robust applications — Rust, Leptos, PostgreSQL, and more."
            path="/skills"
        />
        <div class="py-4">
            <div class="max-w-6xl mx-auto px-6 md:px-12">
                <div class="grid grid-cols-1 lg:grid-cols-[360px_1fr] gap-14 items-start py-16">

                    // ── Left: heading + quick nav ─────────────────────────
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

                        <Suspense fallback=|| ()>
                            {move || skills.get().and_then(|r| r.ok()).map(|items| {
                                let groups = group_by_tier(&items);
                                view! {
                                    <nav class="flex flex-col gap-1">
                                        {groups.into_iter().map(|(tier, group)| {
                                            let count = group.len();
                                            view! {
                                                <a href=format!("#{}", tier.anchor())
                                                    class="flex items-center justify-between px-3 py-2 rounded-lg text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 transition-colors no-underline">
                                                    <span>{tier_label(i18n, tier)}</span>
                                                    <span class="text-xs text-muted/70">{count.to_string()}</span>
                                                </a>
                                            }
                                        }).collect_view()}
                                    </nav>
                                }
                            })}
                        </Suspense>
                    </div>

                    // ── Right: sections, one per tier ─────────────────────
                    <div>
                        <Suspense fallback=|| view! { <SkillCardSkeleton count=9 /> }>
                            {move || skills.get().map(|r| match r {
                                Ok(items) if items.is_empty() => view! {
                                    <p class="text-center text-muted py-16">{t!(i18n, skills.empty)}</p>
                                }.into_any(),
                                Ok(items) => {
                                    let groups = group_by_tier(&items);
                                    view! {
                                        <div class="flex flex-col gap-14">
                                            {groups.into_iter().map(|(tier, group)| {
                                                let count = group.len();
                                                view! {
                                                    <section id=tier.anchor() class="scroll-mt-24">
                                                        <div class="flex items-center gap-3 mb-5">
                                                            <h2 class="text-xl font-bold text-fg">{tier_label(i18n, tier)}</h2>
                                                            <span class="text-xs text-muted bg-line rounded-full px-2 py-0.5">
                                                                {count.to_string()}
                                                            </span>
                                                        </div>
                                                        <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-5">
                                                            {group.into_iter().map(|s| skill_card(i18n, s)).collect_view()}
                                                        </div>
                                                    </section>
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

fn skill_card(i18n: I18nContext<Locale>, s: SkillView) -> impl IntoView {
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
                {(SkillTier::for_star(s.star) == SkillTier::Favorite).then(|| view! {
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
}
