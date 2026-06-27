use leptos::prelude::*;
use modules::notes::NoteView;
use modules::skills::SkillView;

#[server]
pub async fn get_recent_notes() -> Result<Vec<NoteView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::notes::NoteRepository;
    use repositories::notes::NoteRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    NoteRepositoryImpl::new(pool.get_ref().clone())
        .find_recent(6)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_skills() -> Result<Vec<SkillView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::skills::SkillRepository;
    use repositories::skills::SkillRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    SkillRepositoryImpl::new(pool.get_ref().clone())
        .find_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn HomePage() -> impl IntoView {
    let notes = Resource::new(|| (), |_| get_recent_notes());
    let skills = Resource::new(|| (), |_| get_skills());

    view! {
        <div>
            // ── Hero ───────────────────────────────────────────────────────────
            <section class="min-h-screen flex flex-col justify-center bg-base relative overflow-hidden py-16">
                // Background glow
                <div class="absolute top-1/4 right-1/4 w-[500px] h-[500px] bg-violet-600/10 rounded-full blur-[120px] pointer-events-none"></div>

                <div class="w-full max-w-6xl mx-auto px-12 grid grid-cols-2 gap-12 items-center">
                    // ── Left: Text ──────────────────────────────────────────
                    <div>
                        <p class="text-muted text-[0.95rem] mb-3">"Hi, I am"</p>
                        <h1 class="text-4xl font-bold text-fg mb-2">"Feri Irawansyah"</h1>
                        <h2 class="text-[clamp(2.5rem,4vw,3.5rem)] font-extrabold text-violet-500 leading-tight mb-6">
                            "Principal Engineer"
                        </h2>

                        // Social icons
                        <div class="flex gap-3 mb-8">
                            <a href="https://github.com/feri-irawansyah" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-violet-500 hover:border-violet-500 transition-colors no-underline">
                                <i class="bi bi-github text-[1.1rem]"></i>
                            </a>
                            <a href="https://linkedin.com/in/feri-irawansyah" target="_blank"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-violet-500 hover:border-violet-500 transition-colors no-underline">
                                <i class="bi bi-linkedin text-[1.1rem]"></i>
                            </a>
                            <a href="mailto:ir15y4hh@gmail.com"
                                class="w-10 h-10 rounded-full border border-line flex items-center justify-center text-muted hover:text-violet-500 hover:border-violet-500 transition-colors no-underline">
                                <i class="bi bi-envelope-fill text-[1.1rem]"></i>
                            </a>
                        </div>

                        // CTA buttons
                        <div class="flex gap-4 mb-12">
                            <a href="mailto:ir15y4hh@gmail.com"
                                class="px-6 py-2.5 bg-violet-600 hover:bg-violet-500 text-white rounded-lg text-[0.9rem] font-semibold transition-colors no-underline">
                                "Hire Me"
                            </a>
                            <a href="/notes"
                                class="px-6 py-2.5 border border-line text-muted hover:border-violet-500 hover:text-fg rounded-lg text-[0.9rem] font-semibold transition-colors no-underline">
                                "Read Notes"
                            </a>
                        </div>

                        // Stats
                        <div class="flex gap-8 pt-8 border-t border-line">
                            <div>
                                <p class="text-2xl font-extrabold text-violet-500">"5+"</p>
                                <p class="text-sm text-muted mt-0.5">"Years Experience"</p>
                            </div>
                            <div class="w-px bg-line"></div>
                            <div>
                                <p class="text-2xl font-extrabold text-violet-500">"20+"</p>
                                <p class="text-sm text-muted mt-0.5">"Projects Done"</p>
                            </div>
                            <div class="w-px bg-line"></div>
                            <div>
                                <p class="text-2xl font-extrabold text-violet-500">"50+"</p>
                                <p class="text-sm text-muted mt-0.5">"Articles Written"</p>
                            </div>
                        </div>
                    </div>

                    // ── Right: Circular image ───────────────────────────────
                    <div class="flex items-center justify-center">
                        <div class="relative">
                            // Outer glow ring
                            <div class="absolute -inset-3 rounded-full border border-violet-500/20"></div>
                            <div class="absolute -inset-6 rounded-full border border-violet-500/10"></div>
                            // Circle image
                            <div class="w-[360px] h-[360px] rounded-full overflow-hidden border-4 border-violet-600/30 bg-surface">
                                <img
                                    src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/hero-bg.webp"
                                    alt="Feri Irawansyah"
                                    class="w-full h-full object-cover"
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
                                                            <img
                                                                src=s.image_src.clone()
                                                                alt=s.title.clone()
                                                                class="w-10 h-10 object-contain opacity-50 group-hover/skill:opacity-100 transition-opacity"
                                                            />
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="w-10 h-10 rounded-full bg-line flex items-center justify-center text-muted text-xs font-bold opacity-50 group-hover/skill:opacity-100 transition-opacity">
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
                    <div class="text-center mb-14">
                        <h2 class="text-3xl font-extrabold text-fg mb-3">"Recent Notes"</h2>
                        <p class="text-muted max-w-md mx-auto">
                            "Thoughts, learnings, and technical writings on software engineering."
                        </p>
                    </div>
                    <Suspense fallback=|| view! {
                        <div class="text-center text-muted py-8">"Loading notes..."</div>
                    }>
                        {move || notes.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="text-center text-muted py-12">"No notes yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] gap-5">
                                    {items.into_iter().map(|n| {
                                        let img_url = format!(
                                            "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
                                            n.slug
                                        );
                                        view! {
                                        <a href=format!("/notes/{}", n.slug)
                                            class="group bg-base border border-line rounded-xl overflow-hidden flex flex-col hover:border-violet-500 transition-colors no-underline">
                                            // Thumbnail
                                            <div class="w-full h-40 overflow-hidden bg-surface shrink-0">
                                                <img
                                                    src=img_url
                                                    alt=n.title.clone()
                                                    class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                                                    loading="lazy"
                                                    on:error=move |_e: leptos::ev::ErrorEvent| {
                                                        #[cfg(target_arch = "wasm32")]
                                                        {
                                                            use wasm_bindgen::JsCast;
                                                            if let Some(img) = _e.target()
                                                                .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                                            {
                                                                img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                                            }
                                                        }
                                                    }
                                                />
                                            </div>
                                            // Content
                                            <div class="p-5 flex flex-col gap-2 flex-1">
                                                <div class="flex items-center gap-2">
                                                    <span class="text-xs font-semibold text-violet-400 uppercase tracking-wide">
                                                        {n.category.clone()}
                                                    </span>
                                                    <span class="text-xs text-muted">
                                                        {n.last_update.format("%d %b %Y").to_string()}
                                                    </span>
                                                </div>
                                                <h3 class="text-[0.95rem] font-semibold text-fg group-hover:text-violet-400 transition-colors leading-snug">
                                                    {n.title.clone()}
                                                </h3>
                                                {(!n.description.is_empty()).then(|| view! {
                                                    <p class="text-[0.85rem] text-muted leading-relaxed line-clamp-2 flex-1">
                                                        {n.description.clone()}
                                                    </p>
                                                })}
                                                {(!n.hashtag.is_empty()).then(|| {
                                                    let tags = n.hashtag.iter().take(3).cloned().collect::<Vec<_>>();
                                                    view! {
                                                        <div class="flex gap-1.5 flex-wrap mt-auto pt-1">
                                                            {tags.into_iter().map(|tag| view! {
                                                                <span class="text-xs px-2 py-0.5 rounded-full bg-line text-muted">
                                                                    {tag}
                                                                </span>
                                                            }).collect_view()}
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        </a>
                                    }}).collect_view()}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="text-red-400 py-4">"Failed to load: " {e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                    <div class="mt-12 text-center">
                        <a href="/notes"
                            class="inline-flex items-center gap-2 px-6 py-2.5 border border-line text-muted hover:border-violet-500 hover:text-fg rounded-lg text-[0.9rem] font-medium transition-colors no-underline">
                            "All Notes"
                            <i class="bi bi-arrow-right"></i>
                        </a>
                    </div>
                </div>
            </section>

        </div>
    }
}
