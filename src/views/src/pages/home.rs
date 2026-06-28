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
        .recent(6)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_skills() -> Result<Vec<SkillView>, ServerFnError> {
    skill_svc()
        .await?
        .list()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn HomePage() -> impl IntoView {
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
        <div>
            // ── Hero ───────────────────────────────────────────────────────────
            <section class="min-h-screen flex flex-col justify-center bg-base relative overflow-hidden py-16">
                // Background glow
                <div class="absolute top-1/4 right-1/4 w-[500px] h-[500px] bg-teal-600/10 rounded-full blur-[120px] pointer-events-none"></div>

                <div class="w-full max-w-6xl mx-auto px-12 grid grid-cols-2 gap-12 items-center">
                    // ── Left: Text ──────────────────────────────────────────
                    <div>
                        <p class="text-muted text-[0.95rem] mb-3">"Hi, I am"</p>
                        <h1 class="text-4xl font-bold text-fg mb-2">"Feri Irawansyah"</h1>
                        <h2 class="text-[clamp(2.5rem,4vw,3.5rem)] font-extrabold text-teal-500 leading-tight mb-6">
                            "Principal Engineer"
                        </h2>

                        // Social icons
                        <div class="flex gap-3 mb-8">
                            <a href="https://github.com/feri-irawansyah" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 transition-colors no-underline">
                                <i class="bi bi-github text-[1.1rem]"></i>
                            </a>
                            <a href="https://linkedin.com/in/feri-irawansyah" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 transition-colors no-underline">
                                <i class="bi bi-linkedin text-[1.1rem]"></i>
                            </a>
                            <a href="mailto:ir15y4hh@gmail.com"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 transition-colors no-underline">
                                <i class="bi bi-envelope-fill text-[1.1rem]"></i>
                            </a>
                        </div>

                        // CTA buttons
                        <div class="flex gap-4 mb-12">
                            <a href="mailto:ir15y4hh@gmail.com"
                                class="px-6 py-2.5 bg-teal-600 hover:bg-teal-500 text-white rounded-lg text-[0.9rem] font-semibold transition-colors no-underline">
                                "Hire Me"
                            </a>
                            <a href="/notes"
                                class="px-6 py-2.5 border border-line text-muted hover:border-teal-500 hover:text-fg rounded-lg text-[0.9rem] font-semibold transition-colors no-underline">
                                "Read Notes"
                            </a>
                        </div>

                        // Stats
                        <div class="flex gap-8 pt-8 border-t border-line">
                            <div>
                                <p class="text-2xl font-extrabold text-teal-500">"5+"</p>
                                <p class="text-sm text-muted mt-0.5">"Years Experience"</p>
                            </div>
                            <div class="w-px bg-line"></div>
                            <div>
                                <p class="text-2xl font-extrabold text-teal-500">"20+"</p>
                                <p class="text-sm text-muted mt-0.5">"Projects Done"</p>
                            </div>
                            <div class="w-px bg-line"></div>
                            <div>
                                <p class="text-2xl font-extrabold text-teal-500">"50+"</p>
                                <p class="text-sm text-muted mt-0.5">"Articles Written"</p>
                            </div>
                        </div>
                    </div>

                    // ── Right: Circular image ───────────────────────────────
                    <div class="flex items-center justify-center">
                        <div class="relative">
                            // Outer glow rings
                            <div class="absolute -inset-3  puddle-frame border border-teal-500/30" style="animation-delay:-2s"></div>
                            <div class="absolute -inset-6  puddle-frame border border-teal-500/25" style="animation-delay:-4s"></div>
                            <div class="absolute -inset-10 puddle-frame border border-teal-500/20" style="animation-delay:-6s"></div>
                            <div class="absolute -inset-14 puddle-frame border border-teal-500/12" style="animation-delay:-8s"></div>
                            <div class="absolute -inset-18 puddle-frame border border-teal-500/8" style="animation-delay:-10s"></div>
                            <div class="absolute -inset-24 puddle-frame border border-teal-500/5" style="animation-delay:-12s"></div>
                            // Puddle-shaped image
                            <div class="w-[460px] h-[340px] puddle-frame overflow-hidden border-4 border-teal-600/30 bg-surface">
                                <img
                                    src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/hero-bg.webp"
                                    alt="Feri Irawansyah"
                                    class="w-full h-full object-cover object-right scale-125"
                                />
                            </div>
                        </div>
                    </div>
                </div>

                // ── Skill icon marquee (bottom of hero) ──────────────────────
                <div class="mt-16 w-full max-w-6xl mx-auto px-12 overflow-hidden">
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

            // ── Recent Notes ────────────────────────────────────────────────────
            <section class="py-24 bg-surface" id="notes">
                <div class="max-w-6xl mx-auto px-12">
                    <div class="grid grid-cols-[1fr_1.4fr] gap-20 items-start">

                        // ── Left: heading + nav ─────────────────────────────
                        <div class="sticky top-24 pt-2">
                            <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-4 block">
                                "Notes"
                            </span>
                            <h2 class="text-3xl font-extrabold text-fg mb-4">"Recent Notes"</h2>
                            <p class="text-muted leading-relaxed">
                                "Thoughts, learnings, and technical writings on software engineering."
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
                                class="inline-flex items-center gap-2 text-[0.9rem] text-muted hover:text-teal-500 transition-colors no-underline font-medium">
                                "All Notes"
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
                                        <p class="text-muted py-12">"No notes yet."</p>
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
                                                {visible.into_iter().map(|(idx, n)| {
                                                    let img_url = format!(
                                                        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
                                                        n.slug
                                                    );
                                                    view! {
                                                        <a href=format!("/notes/{}", n.slug)
                                                            class="group relative rounded-xl overflow-hidden no-underline flex flex-col h-60 border border-line hover:border-transparent transition-colors duration-300">
                                                            <img
                                                                src=img_url
                                                                alt=n.title.clone()
                                                                class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                                                                loading="lazy"
                                                                on:error=move |_e: leptos::ev::ErrorEvent| {
                                                                    #[cfg(target_arch = "wasm32")]
                                                                    {
                                                                        use leptos::web_sys;
                                                                        use wasm_bindgen::JsCast;
                                                                        if let Some(img) = _e.target()
                                                                            .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                                                        {
                                                                            img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                                                        }
                                                                    }
                                                                }
                                                            />
                                                            <div class="absolute inset-0 bg-white/10 dark:bg-black/55 group-hover:bg-black/10 transition-colors duration-500"></div>
                                                            <div class="absolute inset-0 bg-linear-to-t from-gray-900/80 to-transparent pointer-events-none"></div>
                                                            {match idx % 6 {
                                                                0 => view! { <div class="absolute inset-0 bg-linear-to-br from-teal-600 to-teal-400 opacity-0 group-hover:opacity-80 transition-opacity duration-500"></div> }.into_any(),
                                                                1 => view! { <div class="absolute inset-0 bg-linear-to-br from-amber-500 to-amber-300 opacity-0 group-hover:opacity-80 transition-opacity duration-500"></div> }.into_any(),
                                                                2 => view! { <div class="absolute inset-0 bg-linear-to-br from-purple-600 to-purple-400 opacity-0 group-hover:opacity-80 transition-opacity duration-500"></div> }.into_any(),
                                                                3 => view! { <div class="absolute inset-0 bg-linear-to-br from-green-600 to-green-400 opacity-0 group-hover:opacity-80 transition-opacity duration-500"></div> }.into_any(),
                                                                4 => view! { <div class="absolute inset-0 bg-linear-to-br from-blue-600 to-blue-400 opacity-0 group-hover:opacity-80 transition-opacity duration-500"></div> }.into_any(),
                                                                _ => view! { <div class="absolute inset-0 bg-linear-to-br from-rose-600 to-rose-400 opacity-0 group-hover:opacity-80 transition-opacity duration-500"></div> }.into_any(),
                                                            }}
                                                            <div class="relative z-10 mt-auto p-5 flex flex-col gap-1.5">
                                                                <div class="flex items-center gap-2">
                                                                    <span class="text-xs font-semibold text-teal-300 group-hover:text-white/90 uppercase tracking-wide transition-colors duration-300">
                                                                        {n.category.clone()}
                                                                    </span>
                                                                    <span class="text-xs text-white/60  group-hover:text-white/60 transition-colors duration-300">
                                                                        {n.last_update.format("%d %b %Y").to_string()}
                                                                    </span>
                                                                </div>
                                                                <h3 class="text-[0.95rem] font-bold text-white group-hover:text-white transition-colors duration-300 leading-snug">
                                                                    {n.title.clone()}
                                                                </h3>
                                                                {(!n.description.is_empty()).then(|| view! {
                                                                    <p class="text-[0.82rem] text-white/70 group-hover:text-white/75 transition-colors duration-300 leading-relaxed line-clamp-2">
                                                                        {n.description.clone()}
                                                                    </p>
                                                                })}
                                                            </div>
                                                        </a>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_any()
                                    },
                                    Err(e) => view! {
                                        <p class="text-red-400 py-4">"Failed to load: " {e.to_string()}</p>
                                    }.into_any(),
                                })}
                            </Suspense>
                        </div>

                    </div>
                </div>
            </section>

        </div>
    }
}
