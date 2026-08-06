use crate::components::MarkdownContent;
use crate::i18n::*;
use crate::markdown::{HeadingItem, MarkdownResult};
use crate::seo::{DEFAULT_OG_IMAGE, SITE_URL, Seo};
use leptos::prelude::*;
use leptos_meta::Meta;
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
pub async fn fetch_markdown_html(
    url: String,
    locale: String,
) -> Result<MarkdownResult, ServerFnError> {
    crate::markdown::process_localized(&url, &locale)
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

#[server]
pub async fn get_notes_by_category(category: String) -> Result<Vec<NoteView>, ServerFnError> {
    note_svc()
        .await?
        .by_category(&category)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Pages ────────────────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
pub fn NotesPage() -> impl IntoView {
    let i18n = use_i18n();
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
            {
                let _ = leptos::web_sys::window().map(|w| w.scroll_to_with_x_and_y(0.0, 0.0));
            }
        }
    };
    let on_next = move |_: leptos::ev::MouseEvent| {
        let p = current_page.get();
        if p < total_pages.get() {
            current_page.set(p + 1);
            #[cfg(target_arch = "wasm32")]
            {
                let _ = leptos::web_sys::window().map(|w| w.scroll_to_with_x_and_y(0.0, 0.0));
            }
        }
    };
    let go_to_page = move |p: i64| {
        current_page.set(p);
        #[cfg(target_arch = "wasm32")]
        {
            let _ = leptos::web_sys::window().map(|w| w.scroll_to_with_x_and_y(0.0, 0.0));
        }
    };

    // Compact page list: 1, 2, … , last (no windowing around the current page).
    let page_items = move || -> Vec<Option<i64>> {
        let total = total_pages.get();
        if total <= 3 {
            (1..=total).map(Some).collect()
        } else {
            vec![Some(1), Some(2), None, Some(total)]
        }
    };

    view! {
        <Seo
            title="Notes — Feri Irawansyah"
            description="Articles, learnings, and thoughts on software development by Feri Irawansyah."
            path="/notes"
            image=DEFAULT_OG_IMAGE
        />
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8 flex flex-wrap items-end justify-between gap-4">
                    <div>
                        <h1 class="text-[2.25rem] font-extrabold mb-2">{t!(i18n, notes.title)}</h1>
                        <p class="text-muted text-[1.05rem]">
                            {t!(i18n, notes.subtitle)}
                        </p>
                    </div>

                    <Suspense fallback=|| ()>
                        {move || notes.get().is_some_and(|_| total_pages.get() > 1).then(|| view! {
                            <div class="flex items-center gap-1.5 shrink-0">
                                <button
                                    disabled={move || current_page.get() <= 1}
                                    on:click=on_prev
                                    class="w-8 h-8 rounded-full border border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer">
                                    <i class="bi bi-arrow-left text-xs"></i>
                                </button>

                                {move || page_items().into_iter().map(|item| match item {
                                    Some(p) => view! {
                                        <button
                                            on:click=move |_| go_to_page(p)
                                            class="w-8 h-8 rounded-full border text-sm font-semibold transition-colors cursor-pointer hover:border-teal-500 hover:text-teal-500"
                                            class=("border-teal-500", move || current_page.get() == p)
                                            class=("text-teal-500", move || current_page.get() == p)
                                            class=("border-line", move || current_page.get() != p)
                                            class=("text-muted", move || current_page.get() != p)>
                                            {p.to_string()}
                                        </button>
                                    }.into_any(),
                                    None => view! {
                                        <span class="w-8 h-8 flex items-center justify-center text-muted text-sm select-none">"…"</span>
                                    }.into_any(),
                                }).collect_view()}

                                <button
                                    disabled={move || current_page.get() >= total_pages.get()}
                                    on:click=on_next
                                    class="w-8 h-8 rounded-full border border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer">
                                    <i class="bi bi-arrow-right text-xs"></i>
                                </button>
                            </div>
                        })}
                    </Suspense>
                </header>

                <Suspense fallback=move || view! {
                    <div class="text-center text-muted py-8">{t!(i18n, notes.loading)}</div>
                }>
                    {move || notes.get().map(|r| match r {
                        Ok((items, _)) if items.is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>{t!(i18n, notes.empty)}</p>
                            </div>
                        }.into_any(),
                        Ok((items, _)) => view! {
                            <div class="flex flex-col gap-4">
                                {items.into_iter().map(|n| view! {
                                    <NoteCard note=n />
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">{t!(i18n, notes.load_error)} {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn NoteCard(note: NoteView) -> impl IntoView {
    let href = format!("/notes/{}", note.slug);
    let img_url = format!(
        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
        note.slug
    );
    view! {
        <a href=href
            class="group flex flex-col sm:flex-row gap-4 sm:gap-5 items-start bg-surface border border-line rounded-2xl p-4 sm:p-5 hover:border-teal-500/50 transition-colors no-underline">
            <div class="w-full h-44 sm:w-50 sm:h-32.5 rounded-lg overflow-hidden shrink-0 bg-base border border-line">
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
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-[0.06em]">
                        {note.category.clone()}
                    </span>
                    <span class="text-xs text-muted">
                        {note.last_update.format("%d %b %Y").to_string()}
                    </span>
                </div>
                <h2 class="text-[1.05rem] font-bold mb-1.5 text-fg group-hover:text-teal-500 transition-colors leading-snug">
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
    }
}

#[allow(non_snake_case)]
#[component]
pub fn NotePage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());
    // Blocking so the SSR response waits for the note (title/description/image)
    // before flushing <head> — otherwise the per-note <Seo/> tags stream in too
    // late to be part of the initial response that crawlers see.
    let note = Resource::new_blocking(slug, get_note_by_slug);

    let content_url = move || {
        note.get()
            .and_then(|r| r.ok())
            .flatten()
            .map(|n| n.content)
            .unwrap_or_default()
    };
    let locale_code = move || match i18n.get_locale() {
        Locale::id => "id",
        Locale::en => "en",
    };
    let content_html = Resource::new(
        move || (content_url(), locale_code()),
        |(url, locale)| async move {
            if url.is_empty() {
                Ok(MarkdownResult {
                    html: String::new(),
                    headings: vec![],
                })
            } else {
                fetch_markdown_html(url, locale.to_string()).await
            }
        },
    );

    let headings = move || {
        content_html
            .get()
            .and_then(|r| r.ok())
            .map(|r| r.headings)
            .unwrap_or_default()
    };

    let toc_search = RwSignal::new(String::new());

    // `note` is a blocking resource, so by the time this renders (even outside
    // any <Suspense>) its value is already resolved — required for <Seo/>'s
    // <Title>/<Meta> tags to land in the head instead of streaming in too late.
    let note_seo = move || match note.get() {
        Some(Ok(Some(n))) => {
            let img_url = format!(
                "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
                n.slug
            );
            let note_path = format!("/notes/{}", n.slug);
            let date_iso = n.last_update.to_rfc3339();
            let article_ld = format!(
                r#"{{"@context":"https://schema.org","@type":"Article","headline":"{}","description":"{}","image":"{}","author":{{"@type":"Person","name":"Feri Irawansyah","url":"{SITE_URL}"}},"datePublished":"{date_iso}","dateModified":"{date_iso}","mainEntityOfPage":"{SITE_URL}{note_path}"}}"#,
                json_escape(&n.title),
                json_escape(&n.description),
                img_url,
            );
            view! {
                <Seo
                    title=format!("{} — Feri Irawansyah", n.title)
                    description=n.description.clone()
                    path=note_path
                    image=img_url
                    og_type="article"
                />
                <leptos_meta::Script type_="application/ld+json">{article_ld}</leptos_meta::Script>
            }
            .into_any()
        }
        Some(Ok(None)) => {
            #[cfg(feature = "ssr")]
            {
                if let Some(response) = use_context::<leptos_actix::ResponseOptions>() {
                    response.set_status(actix_web::http::StatusCode::NOT_FOUND);
                }
            }
            view! {
                <Seo
                    title="Note Not Found — Feri Irawansyah"
                    description="The note you're looking for doesn't exist."
                    path=format!("/notes/{}", slug())
                />
                <Meta name="robots" content="noindex, nofollow"/>
            }
            .into_any()
        }
        _ => ().into_any(),
    };

    view! {
        <Suspense fallback=|| ()>
            {note_seo}
        </Suspense>
        <div class="py-4">
            <div class="sticky top-0 z-40 bg-base/80 backdrop-blur-sm border-b border-line xl:hidden">
                <div class="max-w-6xl mx-auto px-6 py-3">
                    <a href="/notes"
                        class="inline-flex items-center gap-1.5 text-sm text-muted hover:text-fg transition-colors no-underline whitespace-nowrap">
                        <i class="bi bi-arrow-left text-[0.9rem]"></i>
                        {t!(i18n, notes.back_to_notes)}
                    </a>
                </div>
            </div>
            <div class="max-w-6xl mx-auto px-6">
                <Suspense fallback=move || view! {
                    <div class="text-center text-muted py-8">{t!(i18n, notes.detail_loading)}</div>
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
                        Ok(None) => {
                            view! {
                                <div class="py-24 text-center">
                                    <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
                                    <p class="text-muted my-4 mb-8">{t!(i18n, notes.not_found_body)}</p>
                                    <a href="/notes"
                                        class="inline-flex items-center gap-1.5 px-[1.4rem] py-[0.6rem] border border-line text-muted hover:border-teal-500 hover:text-fg rounded text-[0.9rem] font-medium transition-colors whitespace-nowrap">
                                        "← " {t!(i18n, notes.back_to_notes)}
                                    </a>
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
    }
}

#[allow(non_snake_case)]
#[component]
fn NoteDetail(
    note: NoteView,
    content_html: Resource<Result<MarkdownResult, ServerFnError>>,
    headings: impl Fn() -> Vec<HeadingItem> + Copy + Send + Sync + 'static,
    toc_search: RwSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let img_url = format!(
        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
        note.slug
    );
    let reading_time = move || {
        content_html
            .get()
            .and_then(|r| r.ok())
            .map(|r| estimate_reading_time(&r.html))
            .unwrap_or(0)
    };
    view! {
        <div class="flex gap-10 items-start">
            <article class="flex-1 min-w-0 pb-12">
                <header class="relative w-full h-95 overflow-hidden mb-10">
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
                            <span class="text-xs font-semibold text-teal-600 dark:text-teal-400 uppercase tracking-[0.06em]">
                                {note.category.clone()}
                            </span>
                            <span class="text-xs text-muted">
                                {note.last_update.format("%d %B %Y").to_string()}
                            </span>
                            {move || (reading_time() > 0).then(|| view! {
                                <span class="flex items-center gap-1 text-xs text-muted/80">
                                    <i class="bi bi-clock"></i>
                                    {format!("{} min read", reading_time())}
                                </span>
                            })}
                        </div>
                        <h1 class="text-3xl font-extrabold mb-2 leading-tight text-gray-600 dark:text-white">{note.title}</h1>
                        <p class="text-gray-500 dark:text-white/70 text-[1.05rem] mb-4">{note.description}</p>
                        <div class="flex flex-wrap gap-1.5">
                            {note.hashtag.into_iter().map(|tag| view! {
                                <span class="text-xs px-2.5 py-0.5 rounded-full bg-white/80 dark:bg-white/10 text-gray-600 dark:text-white/70 backdrop-blur-sm">
                                    "#" {tag}
                                </span>
                            }).collect_view()}
                        </div>
                    </div>
                </header>
                <div class="mb-8 border-b border-line"></div>
                <Suspense fallback=move || view! {
                    <div class="text-muted text-sm py-4">{t!(i18n, notes.content_loading)}</div>
                }>
                    {move || content_html.get().map(|r| match r {
                        Ok(result) => view! { <MarkdownContent html=result.html /> }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 text-sm">{t!(i18n, notes.content_load_error)} {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </article>

            <NoteToc headings=headings toc_search=toc_search />
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn NoteToc(
    headings: impl Fn() -> Vec<HeadingItem> + Copy + Send + Sync + 'static,
    toc_search: RwSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let search_placeholder = move || t_string!(i18n, notes.toc_search_placeholder);
    view! {
        <aside class="hidden xl:flex xl:flex-col w-56 shrink-0 sticky top-4 py-8 max-h-[calc(100vh-2rem)]">
            <a href="/notes"
                class="inline-flex items-center gap-1.5 text-[0.8rem] text-muted hover:text-teal-400 transition-colors no-underline mb-5 whitespace-nowrap">
                <i class="bi bi-arrow-left text-[0.75rem]"></i>
                {t!(i18n, notes.back_to_notes)}
            </a>
            <p class="text-[0.7rem] font-semibold text-muted uppercase tracking-widest mb-3">
                {t!(i18n, notes.toc_title)}
            </p>
            <div class="relative mb-3">
                <i class="bi bi-search absolute left-2.5 top-1/2 -translate-y-1/2 text-muted text-[0.7rem] pointer-events-none"></i>
                <input
                    type="text"
                    placeholder=search_placeholder
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

fn estimate_reading_time(html: &str) -> u32 {
    let mut in_tag = false;
    let text: String = html
        .chars()
        .filter(|&c| {
            if c == '<' { in_tag = true; false }
            else if c == '>' { in_tag = false; false }
            else { !in_tag }
        })
        .collect();
    let words = text.split_whitespace().count();
    ((words as f32 / 200.0).ceil() as u32).max(1)
}

fn json_escape(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            c => c.to_string(),
        })
        .collect()
}
