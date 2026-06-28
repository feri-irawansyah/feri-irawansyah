use crate::components::MarkdownContent;
use crate::markdown::{HeadingItem, MarkdownResult};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use modules::notes::NoteView;

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

#[server]
pub async fn fetch_markdown_html(url: String) -> Result<MarkdownResult, ServerFnError> {
    crate::markdown::process(&url)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_notes_page(page: i64) -> Result<(Vec<NoteView>, i64), ServerFnError> {
    note_svc()
        .await?
        .list_page(page, 8)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_note_by_slug(slug: String) -> Result<Option<NoteView>, ServerFnError> {
    note_svc()
        .await?
        .get_by_slug(&slug)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Pages ────────────────────────────────────────────────────────────────────

#[component]
pub fn NotesPage() -> impl IntoView {
    let current_page = RwSignal::new(1i64);
    let notes = Resource::new(move || current_page.get(), get_notes_page);

    let total_pages = Memo::new(move |_| {
        notes
            .get()
            .and_then(|r| r.ok())
            .map(|(_, total)| ((total + 7) / 8).max(1))
            .unwrap_or(1)
    });

    // Defined outside view! to avoid Leptos macro misparse of `>` in attribute expressions
    let on_prev = move |_: leptos::ev::MouseEvent| {
        let p = current_page.get();
        if p > 1 {
            current_page.set(p - 1);
            #[cfg(target_arch = "wasm32")]
            { let _ = leptos::web_sys::window().map(|w| w.scroll_to_with_x_and_y(0.0, 0.0)); }
        }
    };
    let on_next = move |_: leptos::ev::MouseEvent| {
        let p = current_page.get();
        if p < total_pages.get() {
            current_page.set(p + 1);
            #[cfg(target_arch = "wasm32")]
            { let _ = leptos::web_sys::window().map(|w| w.scroll_to_with_x_and_y(0.0, 0.0)); }
        }
    };

    view! {
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">"Notes"</h1>
                    <p class="text-muted text-[1.05rem]">
                        "Articles, learnings, and thoughts on software development."
                    </p>
                </header>

                <Suspense fallback=|| view! {
                    <div class="text-center text-muted py-8">"Loading notes..."</div>
                }>
                    {move || notes.get().map(|r| match r {
                        Ok((items, _)) if items.is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>"No notes published yet."</p>
                            </div>
                        }.into_any(),
                        Ok((items, _)) => view! {
                            <div class="flex items-center justify-between pb-6 border-b border-line">
                                <button
                                    disabled={move || current_page.get() <= 1}
                                    on:click=on_prev
                                    class="inline-flex items-center gap-1.5 px-4 py-2 text-sm text-muted border border-line rounded hover:border-teal-500 hover:text-fg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                                >
                                    <i class="bi bi-arrow-left text-[0.85rem]"></i>
                                    "Previous"
                                </button>
                                <span class="text-sm text-muted">
                                    "Page " {move || current_page.get()} " of " {move || total_pages.get()}
                                </span>
                                <button
                                    disabled={move || current_page.get() >= total_pages.get()}
                                    on:click=on_next
                                    class="inline-flex items-center gap-1.5 px-4 py-2 text-sm text-muted border border-line rounded hover:border-teal-500 hover:text-fg transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                                >
                                    "Next"
                                    <i class="bi bi-arrow-right text-[0.85rem]"></i>
                                </button>
                            </div>
                            <div class="flex flex-col">
                                {items.into_iter().map(|n| view! {
                                    <NoteCard note=n />
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">"Failed to load: " {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn NoteCard(note: NoteView) -> impl IntoView {
    let href = format!("/notes/{}", note.slug);
    let img_url = format!(
        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
        note.slug
    );
    view! {
        <article class="py-8 border-b border-line first:border-t first:border-line">
            <a href=href class="flex gap-5 items-start group no-underline">
                <div class="w-[200px] h-[130px] rounded-lg overflow-hidden shrink-0 bg-surface border border-line">
                    <img
                        src=img_url
                        alt=note.title.clone()
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
                                    img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                }
                            }
                        }
                    />
                </div>
                <div class="flex-1 min-w-0">
                    <div class="flex gap-3 items-center mb-1.5">
                        <span class="text-xs font-semibold text-teal-400 uppercase tracking-[0.06em]">
                            {note.category.clone()}
                        </span>
                        <span class="text-xs text-muted">
                            {note.last_update.format("%d %b %Y").to_string()}
                        </span>
                    </div>
                    <h2 class="text-[1.05rem] font-bold mb-1.5 text-fg group-hover:text-teal-400 transition-colors leading-snug">
                        {note.title.clone()}
                    </h2>
                    <p class="text-[0.875rem] text-muted mb-2.5 line-clamp-2 leading-relaxed">
                        {note.description.clone()}
                    </p>
                    <div class="flex flex-wrap gap-1.5">
                        {note.hashtag.into_iter().take(4).map(|tag| view! {
                            <span class="text-xs px-2 py-0.5 rounded-full bg-line text-muted">
                                "#" {tag}
                            </span>
                        }).collect_view()}
                    </div>
                </div>
            </a>
        </article>
    }
}

#[component]
pub fn NotePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());
    let note = Resource::new(slug, get_note_by_slug);

    let content_url = move || {
        note.get()
            .and_then(|r| r.ok())
            .flatten()
            .map(|n| n.content)
            .unwrap_or_default()
    };
    let content_html = Resource::new(content_url, |url| async move {
        if url.is_empty() {
            Ok(MarkdownResult {
                html: String::new(),
                headings: vec![],
            })
        } else {
            fetch_markdown_html(url).await
        }
    });

    let headings = move || {
        content_html
            .get()
            .and_then(|r| r.ok())
            .map(|r| r.headings)
            .unwrap_or_default()
    };

    let toc_search = RwSignal::new(String::new());

    view! {
        <div class="py-4">
            <div class="sticky top-0 z-40 bg-base/80 backdrop-blur-sm border-b border-line xl:hidden">
                <div class="max-w-6xl mx-auto px-6 py-3">
                    <a href="/notes"
                        class="inline-flex items-center gap-1.5 text-sm text-muted hover:text-fg transition-colors no-underline">
                        <i class="bi bi-arrow-left text-[0.9rem]"></i>
                        "Back to Notes"
                    </a>
                </div>
            </div>
            <div class="max-w-6xl mx-auto px-6">
                <Suspense fallback=|| view! {
                    <div class="text-center text-muted py-8">"Loading..."</div>
                }>
                    {move || note.get().map(|r| match r {
                        Ok(Some(n)) => view! {
                            <NoteDetail
                                note=n
                                content_html=content_html
                                headings=headings
                                toc_search=toc_search
                            />
                        }.into_any(),
                        Ok(None) => view! {
                            <div class="py-24 text-center">
                                <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
                                <p class="text-muted my-4 mb-8">"Note not found."</p>
                                <a href="/notes"
                                    class="inline-flex items-center gap-1.5 px-[1.4rem] py-[0.6rem] border border-line text-muted hover:border-teal-500 hover:text-fg rounded text-[0.9rem] font-medium transition-colors">
                                    "← Back to Notes"
                                </a>
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">{e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn NoteDetail(
    note: NoteView,
    content_html: Resource<Result<MarkdownResult, ServerFnError>>,
    headings: impl Fn() -> Vec<HeadingItem> + Copy + Send + Sync + 'static,
    toc_search: RwSignal<String>,
) -> impl IntoView {
    let img_url = format!(
        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
        note.slug
    );
    view! {
        <div class="flex gap-10 items-start">
            <article class="flex-1 min-w-0 pb-12">
                <header class="relative w-full h-[380px] overflow-hidden mb-10">
                    <img
                        src=img_url
                        alt=note.title.clone()
                        class="absolute inset-0 w-full h-full object-cover"
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
                    <div class="absolute inset-0 bg-linear-to-t from-base via-base/70 to-transparent"></div>
                    <div class="absolute bottom-0 left-0 right-0 px-8 pb-8">
                        <div class="flex gap-4 items-center mb-3">
                            <span class="text-xs font-semibold text-teal-400 uppercase tracking-[0.06em]">
                                {note.category.clone()}
                            </span>
                            <span class="text-xs text-muted">
                                {note.last_update.format("%d %B %Y").to_string()}
                            </span>
                        </div>
                        <h1 class="text-3xl font-extrabold mb-2 leading-tight text-white">{note.title}</h1>
                        <p class="text-white/70 text-[1.05rem] mb-4">{note.description}</p>
                        <div class="flex flex-wrap gap-1.5">
                            {note.hashtag.into_iter().map(|tag| view! {
                                <span class="text-xs px-2.5 py-0.5 rounded-full bg-white/10 text-white/70 backdrop-blur-sm">
                                    "#" {tag}
                                </span>
                            }).collect_view()}
                        </div>
                    </div>
                </header>
                <div class="mb-8 border-b border-line"></div>
                <Suspense fallback=|| view! {
                    <div class="text-muted text-sm py-4">"Loading content..."</div>
                }>
                    {move || content_html.get().map(|r| match r {
                        Ok(result) => view! { <MarkdownContent html=result.html /> }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 text-sm">"Failed to load content: " {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </article>

            <NoteToc headings=headings toc_search=toc_search />
        </div>
    }
}

#[component]
fn NoteToc(
    headings: impl Fn() -> Vec<HeadingItem> + Copy + Send + Sync + 'static,
    toc_search: RwSignal<String>,
) -> impl IntoView {
    view! {
        <aside class="hidden xl:flex xl:flex-col w-56 shrink-0 sticky top-4 py-8 max-h-[calc(100vh-2rem)]">
            <a href="/notes"
                class="inline-flex items-center gap-1.5 text-[0.8rem] text-muted hover:text-teal-400 transition-colors no-underline mb-5">
                <i class="bi bi-arrow-left text-[0.75rem]"></i>
                "Back to Notes"
            </a>
            <p class="text-[0.7rem] font-semibold text-muted uppercase tracking-widest mb-3">
                "On this page"
            </p>
            <div class="relative mb-3">
                <i class="bi bi-search absolute left-2.5 top-1/2 -translate-y-1/2 text-muted text-[0.7rem] pointer-events-none"></i>
                <input
                    type="text"
                    placeholder="Search headings..."
                    class="w-full text-[0.75rem] bg-surface border border-line rounded pl-7 pr-2.5 py-1.5 text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors"
                    prop:value=toc_search
                    on:input=move |e| toc_search.set(event_target_value(&e))
                />
            </div>
            <nav class="overflow-y-auto flex-1 min-h-0">
                <ul class="space-y-1 pr-1">
                    {move || {
                        let q = toc_search.get().to_lowercase();
                        headings()
                            .into_iter()
                            .filter(|h| q.is_empty() || h.text.to_lowercase().contains(&q))
                            .map(|h| {
                                let indent = format!(
                                    "padding-left:{:.2}rem",
                                    h.level.saturating_sub(2) as f32 * 0.75
                                );
                                view! {
                                    <li style=indent>
                                        <a href=format!("#{}", h.id)
                                            class="block text-[0.8rem] text-muted hover:text-teal-400 transition-colors py-0.5 leading-snug no-underline">
                                            {h.text}
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
            </nav>
        </aside>
    }
}
