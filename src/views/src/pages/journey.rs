use crate::components::{ArticleHeaderSkeleton, ContentLinesSkeleton, MarkdownContent};
use crate::i18n::*;
use crate::pages::notes::{fetch_markdown_html, get_note_by_slug};
use crate::seo::Seo;
use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_router::hooks::use_params_map;

pub fn icon_for_slug(slug: &str) -> &'static str {
    match slug {
        "background" => "bi-house-heart-fill",
        "meditate" => "bi-shield-check",
        "educational" => "bi-terminal-fill",
        "snakesystem" => "bi-rocket-takeoff-fill",
        _ => "bi-journal-text",
    }
}

#[allow(non_snake_case)]
#[component]
pub fn JourneyPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

    let note = Resource::new(slug, |slug| async move {
        match get_note_by_slug(slug).await {
            Ok(Some(n)) if n.category == "journey" => Ok(Some(n)),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    });

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
                Ok(crate::markdown::MarkdownResult {
                    html: String::new(),
                    headings: vec![],
                })
            } else {
                fetch_markdown_html(url, locale.to_string()).await
            }
        },
    );

    view! {
        <div class="py-4">
            <div class="max-w-3xl mx-auto px-6">
                <div class="pt-8 pb-4">
                    <a href="/about#journey"
                        class="inline-flex items-center gap-1.5 text-sm text-muted hover:text-teal-400 transition-colors no-underline">
                        <i class="bi bi-arrow-left text-[0.9rem]"></i>
                        "Back to About"
                    </a>
                </div>

                <Suspense fallback=|| view! {
                    <ArticleHeaderSkeleton with_icon=true />
                    <ContentLinesSkeleton />
                }>
                    {move || note.get().map(|r| match r {
                        Ok(Some(n)) => {
                            let icon = icon_for_slug(&n.slug);
                            view! {
                                <Seo
                                    title=format!("{} — Journey — Feri Irawansyah", n.title)
                                    description=n.description.clone()
                                    path=format!("/about/journey/{}", n.slug)
                                />
                                <header class="pb-8 border-b border-line mb-8">
                                    <div class="w-10 h-10 rounded-xl bg-teal-500/15 flex items-center justify-center mb-5">
                                        <i class=format!("bi {} text-teal-500 text-lg", icon)></i>
                                    </div>
                                    <h1 class="text-[2rem] font-extrabold text-fg">{n.title.clone()}</h1>
                                </header>
                                <Suspense fallback=|| view! { <ContentLinesSkeleton /> }>
                                    {move || content_html.get().map(|r| match r {
                                        Ok(result) if result.html.trim().is_empty() => view! {
                                            <p class="text-muted italic">"This story hasn't been written yet."</p>
                                        }.into_any(),
                                        Ok(result) => view! { <MarkdownContent html=result.html /> }.into_any(),
                                        Err(e) => view! {
                                            <p class="text-red-400 text-sm">"Failed to load content: " {e.to_string()}</p>
                                        }.into_any(),
                                    })}
                                </Suspense>
                            }.into_any()
                        },
                        Ok(None) => {
                            #[cfg(feature = "ssr")]
                            {
                                if let Some(response) = use_context::<leptos_actix::ResponseOptions>() {
                                    response.set_status(actix_web::http::StatusCode::NOT_FOUND);
                                }
                            }
                            view! {
                                <Seo
                                    title="Story Not Found — Feri Irawansyah"
                                    description="The journey story you're looking for doesn't exist."
                                    path=format!("/about/journey/{}", slug())
                                />
                                <Meta name="robots" content="noindex, nofollow"/>
                                <div class="py-24 text-center">
                                    <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
                                    <p class="text-muted my-4 mb-8">"Story not found."</p>
                                    <a href="/about#journey"
                                        class="inline-flex items-center gap-1.5 px-[1.4rem] py-[0.6rem] border border-line text-muted hover:border-teal-500 hover:text-fg rounded text-[0.9rem] font-medium transition-colors">
                                        "← Back to About"
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
