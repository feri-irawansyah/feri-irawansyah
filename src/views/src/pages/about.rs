use crate::i18n::*;
use crate::pages::journey::icon_for_slug;
use crate::pages::notes::get_notes_by_category;
use crate::seo::Seo;
use leptos::prelude::*;

/// Fixed narrative order for the journey story cards — independent of
/// `last_update` in the DB, since editing a story shouldn't reshuffle it.
const JOURNEY_ORDER: &[&str] = &["background", "meditate", "educational", "snakesystem"];

#[allow(non_snake_case)]
#[component]
pub fn AboutPage() -> impl IntoView {
    let i18n = use_i18n();
    let journeys = Resource::new(|| (), |_| get_notes_by_category("journey".to_string()));

    view! {
        <Seo
            title="About — Feri Irawansyah"
            description="Rust programmer, Principal Engineer, and entrepreneur. Self-taught from an Accounting background, now building Snakesystem — a product-focused development organization."
            path="/about"
        />
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">

                // ── Intro ──────────────────────────────────────────────────────
                <section id="intro" class="py-20 scroll-mt-4">
                    <div class="grid grid-cols-1 md:grid-cols-[280px_1fr] gap-14 items-start">

                        // Photo
                        <div class="flex flex-col items-center gap-4">
                            <div class="relative w-56 h-56 rounded-3xl overflow-hidden border-2 border-teal-600/40 bg-surface shadow-lg">
                                <div class="absolute -inset-0.5 rounded-3xl bg-linear-to-br from-teal-500/30 to-transparent pointer-events-none"></div>
                                <img
                                    src=crate::assets::hero_image_url()
                                    alt="Feri Irawansyah"
                                    class="w-full h-full object-cover object-right scale-110"
                                />
                            </div>
                            <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold bg-green-500/15 text-green-400 whitespace-nowrap">
                                <span class="w-1.5 h-1.5 rounded-full bg-green-400 animate-pulse"></span>
                                {t!(i18n, about.badge_available)}
                            </span>
                            <div class="flex flex-col gap-3 w-full max-w-xs">
                                <a href="https://wa.me/6282323443535" target="_blank"
                                    class="px-5 py-2 bg-teal-600 hover:bg-teal-500 text-white rounded-lg text-sm font-semibold text-center transition-colors no-underline">
                                    {t!(i18n, about.contact_cta)}
                                </a>
                                <div class="grid grid-cols-2 gap-3">
                                    <a href="/experience"
                                        class="px-3 py-2 border border-line text-muted hover:border-teal-500 hover:text-fg rounded-lg text-sm font-semibold text-center transition-colors no-underline">
                                        {t!(i18n, about.see_experience)}
                                    </a>
                                    <a href="/cv"
                                        class="inline-flex items-center justify-center gap-1.5 px-3 py-2 border border-line text-muted hover:border-teal-500 hover:text-fg rounded-lg text-sm font-semibold transition-colors no-underline">
                                        <i class="bi bi-file-person"></i>
                                        {t!(i18n, about.view_cv)}
                                    </a>
                                </div>
                            </div>
                        </div>

                        // Bio + details
                        <div>
                            <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                                {t!(i18n, about.eyebrow)}
                            </span>
                            <h1 class="text-[1.85rem] sm:text-[2.25rem] font-extrabold text-fg mb-1">"Feri Irawansyah"</h1>
                            <p class="text-teal-500 font-semibold text-lg mb-5">
                                {t!(i18n, about.role)}
                            </p>
                            <p class="text-muted leading-relaxed mb-8 max-w-prose">
                                {t!(i18n, about.bio_before)}
                                <span class="text-fg font-medium">"Snakesystem"</span>
                                {t!(i18n, about.bio_after)}
                            </p>

                            // Details grid
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-x-8 gap-y-3 text-sm">
                                <div class="flex items-center gap-2 text-muted">
                                    <i class="bi bi-calendar3 w-4 text-center text-teal-500 shrink-0"></i>
                                    <span class="text-muted/60 shrink-0 whitespace-nowrap">{t!(i18n, about.details.birthday_label)}</span>
                                    <span class="text-fg min-w-0">{t!(i18n, about.details.birthday_value)}</span>
                                </div>
                                <div class="flex items-center gap-2 text-muted">
                                    <i class="bi bi-geo-alt-fill w-4 text-center text-teal-500 shrink-0"></i>
                                    <span class="text-muted/60 shrink-0 whitespace-nowrap">{t!(i18n, about.details.location_label)}</span>
                                    <span class="text-fg min-w-0">{t!(i18n, about.details.location_value)}</span>
                                </div>
                                <div class="flex items-center gap-2 text-muted">
                                    <i class="bi bi-mortarboard-fill w-4 text-center text-teal-500 shrink-0"></i>
                                    <span class="text-muted/60 shrink-0 whitespace-nowrap">{t!(i18n, about.details.education_label)}</span>
                                    <span class="text-fg min-w-0">{t!(i18n, about.details.education_value)}</span>
                                </div>
                                <div class="flex items-center gap-2 text-muted">
                                    <i class="bi bi-whatsapp w-4 text-center text-teal-500 shrink-0"></i>
                                    <span class="text-muted/60 shrink-0 whitespace-nowrap">{t!(i18n, about.details.whatsapp_label)}</span>
                                    <a href="https://wa.me/6282323443535" target="_blank"
                                        class="text-fg hover:text-teal-500 transition-colors no-underline">
                                        "+62 823-2344-3535"
                                    </a>
                                </div>
                                <div class="flex items-center gap-2 text-muted">
                                    <i class="bi bi-telephone-fill w-4 text-center text-teal-500 shrink-0"></i>
                                    <span class="text-muted/60 shrink-0 whitespace-nowrap">{t!(i18n, about.details.phone_label)}</span>
                                    <a href="tel:+6282323443535"
                                        class="text-fg hover:text-teal-500 transition-colors no-underline">
                                        "+62 823-2344-3535"
                                    </a>
                                </div>
                                <div class="flex items-center gap-2 text-muted">
                                    <i class="bi bi-github w-4 text-center text-teal-500 shrink-0"></i>
                                    <span class="text-muted/60 shrink-0 whitespace-nowrap">{t!(i18n, about.details.github_label)}</span>
                                    <a href="https://github.com/feri-irawansyah" target="_blank"
                                        class="text-fg hover:text-teal-500 transition-colors no-underline">
                                        "@feri-irawansyah"
                                    </a>
                                </div>
                            </div>

                        </div>
                    </div>
                </section>

                // ── Divider ────────────────────────────────────────────────────
                <div class="border-t border-line"></div>

                // ── Journey ────────────────────────────────────────────────────
                <section id="journey" class="py-20 scroll-mt-4">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-4 block">
                        {t!(i18n, about.journey.eyebrow)}
                    </span>
                    <h2 class="text-[2rem] font-extrabold text-fg mb-12">{t!(i18n, about.journey.title)}</h2>

                    <Suspense fallback=|| view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
                            <div class="h-48 rounded-2xl bg-line/30 animate-pulse"></div>
                            <div class="h-48 rounded-2xl bg-line/30 animate-pulse"></div>
                            <div class="h-48 rounded-2xl bg-line/30 animate-pulse"></div>
                            <div class="h-48 rounded-2xl bg-line/30 animate-pulse"></div>
                        </div>
                    }>
                        {move || journeys.get().map(|r| match r {
                            Ok(mut items) => {
                                items.sort_by_key(|n| {
                                    JOURNEY_ORDER.iter().position(|s| *s == n.slug).unwrap_or(usize::MAX)
                                });
                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
                                        {items.into_iter().map(|n| {
                                            let icon = icon_for_slug(&n.slug);
                                            view! {
                                                <a href=format!("/about/journey/{}", n.slug)
                                                    class="group block bg-surface border border-line rounded-2xl p-8 hover:border-teal-500/50 transition-colors no-underline">
                                                    <div class="w-10 h-10 rounded-xl bg-teal-500/15 flex items-center justify-center mb-5">
                                                        <i class=format!("bi {} text-teal-500 text-lg", icon)></i>
                                                    </div>
                                                    <h3 class="text-lg font-bold text-fg mb-3">{n.title.clone()}</h3>
                                                    <p class="text-muted leading-relaxed text-[0.925rem]">
                                                        {n.description.clone()}
                                                    </p>
                                                    <span class="inline-flex items-center gap-1.5 mt-4 text-xs font-semibold text-teal-500 group-hover:gap-2.5 transition-all whitespace-nowrap">
                                                        {t!(i18n, about.journey.read_more)} <i class="bi bi-arrow-right"></i>
                                                    </span>
                                                </a>
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

                    // Pull quote
                    <blockquote class="mt-14 relative pl-6 border-l-4 border-teal-500">
                        <p class="text-[1.15rem] font-medium text-fg leading-relaxed italic">
                            {t!(i18n, about.journey.quote)}
                        </p>
                        <footer class="mt-3 text-sm text-muted">{t!(i18n, about.journey.quote_author)}</footer>
                    </blockquote>
                </section>

                // ── Divider ────────────────────────────────────────────────────
                <div class="border-t border-line"></div>

                // ── Quick links ────────────────────────────────────────────────
                <section class="py-16">
                    <h2 class="text-xl font-bold text-fg mb-8">{t!(i18n, common.explore.eyebrow)}</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
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
                        <a href="/cv"
                            class="group flex items-center gap-4 bg-surface border border-line rounded-xl p-5 hover:border-teal-500 transition-colors no-underline">
                            <div class="w-10 h-10 rounded-lg bg-teal-500/10 flex items-center justify-center group-hover:bg-teal-500/20 transition-colors shrink-0">
                                <i class="bi bi-file-person text-teal-500"></i>
                            </div>
                            <div class="min-w-0 flex-1">
                                <p class="text-sm font-semibold text-fg">"Curriculum Vitae"</p>
                                <p class="text-xs text-muted">"Preview &amp; download PDF"</p>
                            </div>
                            <i class="bi bi-arrow-right text-muted group-hover:text-teal-500 transition-colors ml-auto text-sm"></i>
                        </a>
                    </div>
                </section>

            </div>
        </div>
    }
}
