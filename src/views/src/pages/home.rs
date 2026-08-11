use crate::i18n::*;
use crate::seo::Seo;
use leptos::prelude::*;
use modules::notes::NoteView;
use modules::skills::SkillView;

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
pub async fn get_recent_notes() -> Result<Vec<NoteView>, ServerFnError> {
    note_svc()
        .await?
        .recent_async(6)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_skills() -> Result<Vec<SkillView>, ServerFnError> {
    skill_svc()
        .await?
        .find_all_async()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[allow(non_snake_case)]
#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();
    let notes = Resource::new(|| (), |_| get_recent_notes());
    let skills = Resource::new(|| (), |_| get_skills());
    let (page, set_page) = signal(0usize);
    let (go_next, set_go_next) = signal(true);
    let (nav_tick, set_nav_tick) = signal(0u32);
    let prev_disabled = Memo::new(move |_| page.get() == 0);
    let next_disabled = Memo::new(move |_| {
        notes
            .get()
            .and_then(|r| r.ok())
            .map(|items| page.get() * 2 + 2 >= items.len())
            .unwrap_or(true)
    });

    view! {
        <Seo
            title="Feri Irawansyah — Principal Engineer & Rust Programmer"
            description="Portfolio of Feri Irawansyah — Rust programmer, Principal Engineer, and founder of Snakesystem, building high-performance web applications and APIs."
            path="/"
        />
        <div>
            // ── Hero ───────────────────────────────────────────────────────────
            <section class="min-h-screen flex flex-col justify-center bg-base relative overflow-hidden py-16">
                // Background glow
                <div class="absolute top-1/4 right-1/4 w-125 h-125 bg-teal-600/10 rounded-full blur-[120px] pointer-events-none"></div>

                <div class="w-full max-w-6xl mx-auto px-6 md:px-12 grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
                    // ── Left: Text ──────────────────────────────────────────
                    <div class="text-center md:text-left order-2 md:order-1">
                        <p class="text-muted text-[0.95rem] mb-3">{t!(i18n, home.hero.greeting)}</p>
                        <h1 class="text-4xl font-bold text-fg mb-2">"Feri Irawansyah"</h1>
                        <h2 class="text-[clamp(2.5rem,4vw,3.5rem)] font-extrabold text-teal-500 leading-tight mb-6">
                            {t!(i18n, home.hero.role)}
                        </h2>

                        // Social icons
                        <div class="flex gap-3 mb-8 justify-center md:justify-start">
                            <a href="https://github.com/feri-irawansyah" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 transition-colors no-underline">
                                <i class="bi bi-github text-[1.1rem]"></i>
                            </a>
                            <a href="https://linkedin.com/in/feri-irawansyah" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 transition-colors no-underline">
                                <i class="bi bi-linkedin text-[1.1rem]"></i>
                            </a>
                            <a href="https://wa.me/6282323443535" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 transition-colors no-underline">
                                <i class="bi bi-whatsapp text-[1.1rem]"></i>
                            </a>
                        </div>

                        // CTA buttons
                        <div class="flex gap-3 mb-12 justify-center md:justify-start flex-wrap">
                            <a href="https://wa.me/6282323443535" target="_blank"
                                class="inline-flex items-center gap-2 px-6 py-3 bg-teal-600 hover:bg-teal-500 text-white rounded-xl text-sm font-semibold transition-all duration-200 no-underline shadow-lg shadow-teal-900/30 hover:shadow-teal-800/40 hover:-translate-y-0.5 whitespace-nowrap">
                                <i class="bi bi-whatsapp text-xs"></i>
                                {t!(i18n, home.hero.contact_cta)}
                            </a>
                            <a href="/notes"
                                class="inline-flex items-center gap-2 px-6 py-3 border border-line text-muted hover:border-teal-500 hover:text-teal-400 rounded-xl text-sm font-semibold transition-all duration-200 no-underline hover:-translate-y-0.5 whitespace-nowrap">
                                {t!(i18n, home.hero.explore_notes)}
                                <i class="bi bi-arrow-right text-xs"></i>
                            </a>
                        </div>

                        // Stats
                        <div class="flex gap-8 pt-8 border-t border-line justify-center md:justify-start">
                            <div>
                                <p class="text-2xl font-extrabold text-teal-500">"5+"</p>
                                <p class="text-sm text-muted mt-0.5">{t!(i18n, home.hero.stats.years)}</p>
                            </div>
                            <div class="w-px bg-line"></div>
                            <div>
                                <p class="text-2xl font-extrabold text-teal-500">"20+"</p>
                                <p class="text-sm text-muted mt-0.5">{t!(i18n, home.hero.stats.projects)}</p>
                            </div>
                            <div class="w-px bg-line"></div>
                            <div>
                                <p class="text-2xl font-extrabold text-teal-500">"50+"</p>
                                <p class="text-sm text-muted mt-0.5">{t!(i18n, home.hero.stats.articles)}</p>
                            </div>
                        </div>
                    </div>

                    // ── Right: Circular image ───────────────────────────────
                    <div class="flex items-center justify-center order-1 md:order-2">
                        <div class="relative">
                            // Outer glow rings
                            <div class="absolute -inset-3  puddle-frame border border-teal-500/30" style="animation-delay:-2s"></div>
                            <div class="absolute -inset-6  puddle-frame border border-teal-500/25" style="animation-delay:-4s"></div>
                            <div class="absolute -inset-10 puddle-frame border border-teal-500/20" style="animation-delay:-6s"></div>
                            <div class="absolute -inset-14 puddle-frame border border-teal-500/12" style="animation-delay:-8s"></div>
                            <div class="absolute -inset-18 puddle-frame border border-teal-500/8" style="animation-delay:-10s"></div>
                            <div class="absolute -inset-24 puddle-frame border border-teal-500/5" style="animation-delay:-12s"></div>
                            // Puddle-shaped image
                            <div class="w-70 h-52 sm:w-90 sm:h-68 md:w-115 md:h-85 puddle-frame overflow-hidden border-4 border-teal-600/30 bg-surface">
                                <img
                                    src=crate::assets::hero_image_url()
                                    alt="Feri Irawansyah"
                                    class="w-full h-full object-cover object-right scale-125"
                                />
                            </div>
                        </div>
                    </div>
                </div>

                // ── Skill icon marquee (bottom of hero) ──────────────────────
                <div class="mt-16 w-full max-w-6xl mx-auto px-6 md:px-12 overflow-hidden">
                    <Suspense fallback=|| view! { <div></div> }>
                        {move || skills.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! { <div></div> }.into_any(),
                            Ok(items) => {
                                let doubled: Vec<_> = items.iter().chain(items.iter()).cloned().collect();
                                view! {
                                    <div class="marquee-inner">
                                        {doubled.into_iter().map(|s| {
                                            view! {
                                                <div class="relative group/skill shrink-0">
                                                    {if !s.image_src.is_empty() {
                                                        view! {
                                                            <div class="w-12 h-12 rounded-lg bg-white/10 flex items-center justify-center p-1.5 dark:opacity-70 group-hover/skill:opacity-100 transition-opacity">
                                                                <img
                                                                    src=s.image_src.clone()
                                                                    alt=s.title.clone()
                                                                    class="w-full h-full object-contain"
                                                                />
                                                            </div>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="w-12 h-12 rounded-lg bg-white/10 flex items-center justify-center text-muted text-xs font-bold opacity-50 group-hover/skill:opacity-100 transition-opacity">
                                                                {s.title.chars().next().unwrap_or('?').to_string()}
                                                            </div>
                                                        }.into_any()
                                                    }}
                                                    <span class="absolute -top-8 left-1/2 -translate-x-1/2 px-2 py-1 rounded bg-surface border border-line text-xs text-fg whitespace-nowrap opacity-0 group-hover/skill:opacity-100 transition-opacity pointer-events-none z-10">
                                                        {s.title.clone()}
                                                    </span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            },
                            Err(_) => view! { <div></div> }.into_any(),
                        })}
                    </Suspense>
                </div>
            </section>

            // ── About Me teaser ─────────────────────────────────────────────────
            <section class="py-24" id="about">
                <div class="max-w-2xl mx-auto px-6 md:px-12 text-center">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        {t!(i18n, home.about.eyebrow)}
                    </span>
                    <h2 class="text-[2rem] font-extrabold text-fg mb-5">{t!(i18n, home.about.title)}</h2>
                    <p class="text-muted leading-relaxed mb-6 mx-auto">
                        {t!(i18n, home.about.description_before)}
                        <span class="text-fg font-medium">"Snakesystem"</span>
                        {t!(i18n, home.about.description_after)}
                    </p>
                    <blockquote class="relative pl-5 border-l-4 border-teal-500 text-left mx-auto mb-7 max-w-md">
                        <p class="text-[1.05rem] font-medium text-fg leading-relaxed italic">
                            {t!(i18n, home.about.quote)}
                        </p>
                    </blockquote>
                    <a href="/about"
                        class="inline-flex items-center gap-2 px-6 py-3 border border-line text-muted hover:border-teal-500 hover:text-fg rounded-xl text-sm font-semibold transition-all duration-200 no-underline hover:-translate-y-0.5 whitespace-nowrap">
                        {t!(i18n, home.about.cta)}
                        <i class="bi bi-arrow-right text-xs"></i>
                    </a>
                </div>
            </section>

            // ── Recent Notes ────────────────────────────────────────────────────
            <section class="py-24 bg-surface" id="notes">
                <div class="max-w-6xl mx-auto px-6 md:px-12">
                    <div class="grid grid-cols-1 md:grid-cols-[1fr_1.4fr] gap-10 md:gap-20 items-start">

                        // ── Left: heading + nav ─────────────────────────────
                        <div class="md:sticky md:top-24 pt-2">
                            <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-4 block">
                                {t!(i18n, home.notes.eyebrow)}
                            </span>
                            <h2 class="text-3xl font-extrabold text-fg mb-4">{t!(i18n, home.notes.title)}</h2>
                            <p class="text-muted leading-relaxed">
                                {t!(i18n, home.notes.description)}
                            </p>

                            // Prev / Next
                            <div class="flex items-center gap-3 mt-10 mb-8">
                                <button
                                    on:click=move |_| {
                                        set_go_next.set(false);
                                        set_nav_tick.update(|t| *t += 1);
                                        set_page.update(|p| *p = p.saturating_sub(1));
                                    }
                                    prop:disabled=prev_disabled
                                    class="w-10 h-10 rounded-full border cursor-pointer border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
                                    <i class="bi bi-arrow-left"></i>
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_go_next.set(true);
                                        set_nav_tick.update(|t| *t += 1);
                                        let max_page = notes.get()
                                            .and_then(|r| r.ok())
                                            .map(|items| items.len().saturating_sub(1) / 2)
                                            .unwrap_or(0);
                                        set_page.update(|p| *p = (*p + 1).min(max_page));
                                    }
                                    prop:disabled=next_disabled
                                    class="w-10 h-10 rounded-full border cursor-pointer border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed">
                                    <i class="bi bi-arrow-right"></i>
                                </button>
                            </div>

                            <a href="/notes"
                                class="inline-flex items-center gap-2 text-[0.9rem] text-muted hover:text-teal-500 transition-colors no-underline font-medium whitespace-nowrap">
                                {t!(i18n, home.notes.all_link)}
                                <i class="bi bi-arrow-right text-sm"></i>
                            </a>
                        </div>

                        // ── Right: cards ─────────────────────────────────────
                        <div>
                            <Suspense fallback=|| view! {
                                <div class="flex flex-col gap-5">
                                    <div class="h-60 rounded-xl bg-line/30 animate-pulse"></div>
                                    <div class="h-60 rounded-xl bg-line/30 animate-pulse"></div>
                                </div>
                            }>
                                {move || notes.get().map(|r| match r {
                                    Ok(items) if items.is_empty() => view! {
                                        <p class="text-muted py-12">{t!(i18n, home.notes.empty)}</p>
                                    }.into_any(),
                                    Ok(items) => {
                                        let start = page.get() * 2;
                                        let visible: Vec<_> = items.into_iter().enumerate()
                                            .skip(start).take(2).collect();
                                        view! {
                                            <div
                                                class="flex flex-col gap-5"
                                                class=("notes-anim-up-a",   move || go_next.get() &&  nav_tick.get() % 2 == 0)
                                                class=("notes-anim-up-b",   move || go_next.get() &&  nav_tick.get() % 2 != 0)
                                                class=("notes-anim-down-a", move || !go_next.get() && nav_tick.get() % 2 == 0)
                                                class=("notes-anim-down-b", move || !go_next.get() && nav_tick.get() % 2 != 0)
                                            >
                                                {visible.into_iter().map(|(_, n)| {
                                                    let img_url = crate::assets::note_cover_url(&n.slug);
                                                    view! {
                                                        <a href=format!("/notes/{}", n.slug)
                                                            class="group flex flex-col sm:flex-row gap-4 sm:gap-5 items-start bg-surface border border-line rounded-2xl p-4 sm:p-5 hover:border-teal-500/50 transition-colors no-underline">
                                                            <div class="w-full h-44 sm:w-50 sm:h-32.5 rounded-lg overflow-hidden shrink-0 bg-base border border-line">
                                                                <img
                                                                    src=img_url
                                                                    alt=n.title.clone()
                                                                    class="w-full h-full object-cover"
                                                                    loading="lazy"
                                                                    on:error=move |_e: leptos::ev::ErrorEvent| {
                                                                        #[cfg(target_arch = "wasm32")]
                                                                        {
                                                                            use leptos::web_sys;
                                                                            use wasm_bindgen::JsCast;
                                                                            if let Some(img) = _e.target()
                                                                                .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                                                            {
                                                                                img.set_src(&crate::assets::note_default_cover_url());
                                                                            }
                                                                        }
                                                                    }
                                                                />
                                                            </div>
                                                            <div class="flex-1 min-w-0">
                                                                <div class="flex gap-3 items-center mb-1.5">
                                                                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-[0.06em]">
                                                                        {n.category.clone()}
                                                                    </span>
                                                                    <span class="text-xs text-muted">
                                                                        {n.last_update.format("%d %b %Y").to_string()}
                                                                    </span>
                                                                </div>
                                                                <h3 class="text-[1.05rem] font-bold mb-1.5 text-fg group-hover:text-teal-500 transition-colors leading-snug">
                                                                    {n.title.clone()}
                                                                </h3>
                                                                {(!n.description.is_empty()).then(|| view! {
                                                                    <p class="text-[0.875rem] text-muted mb-2.5 line-clamp-2 leading-relaxed">
                                                                        {n.description.clone()}
                                                                    </p>
                                                                })}
                                                                <span class="inline-flex items-center gap-1 text-xs font-semibold text-teal-500">
                                                                    {t!(i18n, home.notes.read_more)}
                                                                    <i class="bi bi-arrow-right group-hover:translate-x-0.5 transition-transform duration-300"></i>
                                                                </span>
                                                            </div>
                                                        </a>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_any()
                                    },
                                    Err(e) => view! {
                                        <p class="text-red-400 py-4">{t!(i18n, home.notes.load_error)} {e.to_string()}</p>
                                    }.into_any(),
                                })}
                            </Suspense>
                        </div>

                    </div>
                </div>
            </section>

            // ── Explore More ────────────────────────────────────────────────────
            <section class="py-24">
                <div class="max-w-6xl mx-auto px-6 md:px-12">
                    <div class="text-center mb-12">
                        <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                            {t!(i18n, common.explore.eyebrow)}
                        </span>
                        <h2 class="text-[2rem] font-extrabold text-fg">{t!(i18n, common.explore.title)}</h2>
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                        <a href="/about"
                            class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-5 hover:border-teal-500 transition-colors no-underline">
                            <div class="w-10 h-10 rounded-lg bg-teal-500/10 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                <i class="bi bi-person-fill text-teal-500"></i>
                            </div>
                            <div class="min-w-0 flex-1">
                                <p class="text-sm font-semibold text-fg">{t!(i18n, common.explore.about.title)}</p>
                                <p class="text-xs text-muted">{t!(i18n, common.explore.about.subtitle)}</p>
                            </div>
                            <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                        </a>
                        <a href="/skills"
                            class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-5 hover:border-teal-500 transition-colors no-underline">
                            <div class="w-10 h-10 rounded-lg bg-teal-500/10 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                <i class="bi bi-cpu text-teal-500"></i>
                            </div>
                            <div class="min-w-0 flex-1">
                                <p class="text-sm font-semibold text-fg">{t!(i18n, common.explore.skills.title)}</p>
                                <p class="text-xs text-muted">{t!(i18n, common.explore.skills.subtitle)}</p>
                            </div>
                            <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                        </a>
                        <a href="/experience"
                            class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-5 hover:border-teal-500 transition-colors no-underline">
                            <div class="w-10 h-10 rounded-lg bg-teal-500/10 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                <i class="bi bi-person-workspace text-teal-500"></i>
                            </div>
                            <div class="min-w-0 flex-1">
                                <p class="text-sm font-semibold text-fg">{t!(i18n, common.explore.experience.title)}</p>
                                <p class="text-xs text-muted">{t!(i18n, common.explore.experience.subtitle)}</p>
                            </div>
                            <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                        </a>
                        <a href="/portfolio"
                            class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-5 hover:border-teal-500 transition-colors no-underline">
                            <div class="w-10 h-10 rounded-lg bg-teal-500/10 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                <i class="bi bi-grid-fill text-teal-500"></i>
                            </div>
                            <div class="min-w-0 flex-1">
                                <p class="text-sm font-semibold text-fg">{t!(i18n, common.explore.portfolio.title)}</p>
                                <p class="text-xs text-muted">{t!(i18n, common.explore.portfolio.subtitle)}</p>
                            </div>
                            <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                        </a>
                    </div>
                </div>
            </section>

        </div>
    }
}
