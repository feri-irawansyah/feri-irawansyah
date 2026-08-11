use crate::assets::note_cover_url;
use crate::components::{ContentLinesSkeleton, MarkdownContent, NoteCardSkeleton, NoteHeroSkeleton};
use crate::i18n::*;
use crate::markdown::{HeadingItem, MarkdownResult};
use crate::seo::{SITE_URL, Seo};
use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_router::NavigateOptions;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};
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

#[cfg(feature = "ssr")]
async fn cache_svc() -> Result<std::sync::Arc<dyn modules::cache::CacheService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::cache::CacheService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

/// TTL for cached rendered content (`views::markdown::process_localized_cached`)
/// — see that fn's doc for why this is TTL-only, no active invalidation.
#[cfg(feature = "ssr")]
const CONTENT_CACHE_TTL_SECS: u64 = 3600;

#[server]
pub async fn fetch_markdown_html(
    url: String,
    locale: String,
) -> Result<MarkdownResult, ServerFnError> {
    let cache = cache_svc().await?;
    let key = format!("notes-content:v1:{locale}:{url}");
    crate::markdown::process_localized_cached(cache.as_ref(), &key, &url, &locale, CONTENT_CACHE_TTL_SECS)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_notes_page(page: i64) -> Result<(Vec<NoteView>, i64), ServerFnError> {
    note_svc()
        .await?
        .find_page_async(page, 8)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_note_by_slug(slug: String) -> Result<Option<NoteView>, ServerFnError> {
    note_svc()
        .await?
        .find_by_slug_async(&slug)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_notes_by_category(category: String) -> Result<Vec<NoteView>, ServerFnError> {
    note_svc()
        .await?
        .find_by_category_async(&category)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn search_notes(query: String, page: i64) -> Result<(Vec<NoteView>, i64), ServerFnError> {
    note_svc()
        .await?
        .search_async(&query, page, 8)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Pages ────────────────────────────────────────────────────────────────────

#[allow(non_snake_case)]
#[component]
pub fn NotesPage() -> impl IntoView {
    let i18n = use_i18n();
    let query_map = use_query_map();
    let navigate = use_navigate();

    let initial_q = query_map.with_untracked(|q| q.get("q").unwrap_or_default());
    let search_input = RwSignal::new(initial_q.clone());
    let active_query = RwSignal::new(initial_q);
    let current_page = RwSignal::new(1i64);

    // Browser back/forward (or a plain `/notes?q=...` link) changes the URL
    // without going through `on_search_submit` below — keep the signals that
    // actually drive the resource in sync with it.
    Effect::new(move |_| {
        let q = query_map.with(|q| q.get("q").unwrap_or_default());
        if q != active_query.get_untracked() {
            search_input.set(q.clone());
            active_query.set(q);
            current_page.set(1);
        }
    });

    let notes = Resource::new(
        move || (current_page.get(), active_query.get()),
        |(page, q)| async move {
            if q.trim().is_empty() {
                get_notes_page(page).await
            } else {
                search_notes(q, page).await
            }
        },
    );

    let total_pages = Memo::new(move |_| {
        notes
            .get()
            .and_then(|r| r.ok())
            .map(|(_, total)| ((total + 7) / 8).max(1))
            .unwrap_or(1)
    });

    let navigate_submit = navigate.clone();
    let on_search_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let q = search_input.get();
        active_query.set(q.clone());
        current_page.set(1);
        let target = if q.trim().is_empty() {
            "/notes".to_string()
        } else {
            format!("/notes?q={}", encode_query_param(q.trim()))
        };
        navigate_submit(
            &target,
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
    };

    let on_clear_search = move |_: leptos::ev::MouseEvent| {
        search_input.set(String::new());
        active_query.set(String::new());
        current_page.set(1);
        navigate(
            "/notes",
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
    };

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

    let search_placeholder = move || t_string!(i18n, notes.search_placeholder);
    let search_clear_label = move || t_string!(i18n, notes.search_clear);

    view! {
        <Seo
            title="Notes — Feri Irawansyah"
            description="Articles, learnings, and thoughts on software development by Feri Irawansyah."
            path="/notes"
            image=crate::assets::hero_image_url()
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

                <form on:submit=on_search_submit class="flex items-center gap-2 mb-8">
                    <div class="relative flex-1">
                        <i class="bi bi-search absolute left-4 top-1/2 -translate-y-1/2 text-muted text-sm pointer-events-none"></i>
                        <input
                            type="text"
                            aria-label=search_placeholder
                            placeholder=search_placeholder
                            class="w-full text-sm bg-surface border border-line rounded-xl pl-11 pr-10 py-3 text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors"
                            prop:value=search_input
                            on:input=move |e| search_input.set(event_target_value(&e))
                        />
                        {move || {
                            let on_clear_search = on_clear_search.clone();
                            (!search_input.get().is_empty()).then(move || view! {
                                <button
                                    type="button"
                                    aria-label=search_clear_label
                                    on:click=on_clear_search
                                    class="absolute right-3 top-1/2 -translate-y-1/2 text-muted hover:text-fg transition-colors cursor-pointer">
                                    <i class="bi bi-x-lg text-sm"></i>
                                </button>
                            })
                        }}
                    </div>
                    <button
                        type="submit"
                        class="shrink-0 flex items-center gap-1.5 px-4 py-3 bg-teal-600 hover:bg-teal-500 text-white rounded-xl text-sm font-semibold transition-colors cursor-pointer whitespace-nowrap">
                        <i class="bi bi-search text-xs"></i>
                        {t!(i18n, notes.search_submit)}
                    </button>
                </form>

                {move || (!active_query.get().is_empty()).then(|| view! {
                    <Meta name="robots" content="noindex, follow"/>
                    <p class="text-sm text-muted -mt-4 mb-6">
                        {t!(i18n, notes.search_results_for)} " “" {active_query.get()} "”"
                    </p>
                })}

                <Suspense fallback=|| view! { <NoteCardSkeleton count=3 /> }>
                    {move || notes.get().map(|r| match r {
                        Ok((items, _)) if items.is_empty() && !active_query.get().is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>{t!(i18n, notes.search_empty)} " “" {active_query.get()} "”"</p>
                            </div>
                        }.into_any(),
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
    let img_url = note_cover_url(&note.slug);
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
                            use crate::assets::note_default_cover_url;
                            use leptos::web_sys;
                            use wasm_bindgen::JsCast;
                            if let Some(img) = _e.target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                            {
                                img.set_src(&note_default_cover_url());
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
            let img_url = note_cover_url(&n.slug);
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
                <Suspense fallback=|| view! {
                    <div class="pb-12">
                        <NoteHeroSkeleton />
                        <ContentLinesSkeleton />
                    </div>
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
    let img_url = note_cover_url(&note.slug);
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
                                use crate::assets::note_default_cover_url;
                                use leptos::web_sys;
                                use wasm_bindgen::JsCast;
                                if let Some(img) = _e.target()
                                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                {
                                    img.set_src(&note_default_cover_url());
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
                <Suspense fallback=|| view! { <ContentLinesSkeleton /> }>
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
            if c == '<' {
                in_tag = true;
                false
            } else if c == '>' {
                in_tag = false;
                false
            } else {
                !in_tag
            }
        })
        .collect();
    let words = text.split_whitespace().count();
    ((words as f32 / 200.0).ceil() as u32).max(1)
}

/// Minimal RFC 3986 percent-encoder for a URL query value — enough to keep
/// `?q=...` shareable/refreshable without pulling in a dedicated crate.
/// Operates byte-by-byte, which stays correct for multi-byte UTF-8 too.
pub(crate) fn encode_query_param(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(*b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
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
