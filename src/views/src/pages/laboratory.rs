use crate::components::{ArticleHeaderSkeleton, ContentLinesSkeleton, ListRowSkeleton};
use crate::i18n::*;
use crate::markdown::MarkdownResult;
use crate::seo::Seo;
use leptos::prelude::*;
use leptos_i18n::I18nContext;
use leptos_router::hooks::use_params_map;
use modules::laboratory::LaboratoryView;

const CATEGORIES: &[(&str, &str)] = &[
    ("performance", "fullstack.webp"),
    ("security", "devops.webp"),
    ("architecture", "backend.webp"),
    ("rendering", "frontend.webp"),
    ("restapi", "random.webp"),
];

fn category_image_url(slug: &str) -> String {
    let file = CATEGORIES
        .iter()
        .find(|(s, _)| *s == slug)
        .map(|(_, f)| *f)
        .unwrap_or("random.webp");
    crate::assets::asset_url(file)
}

fn category_title(i18n: I18nContext<Locale>, slug: String) -> impl IntoView {
    match slug.as_str() {
        "performance" => t!(i18n, laboratory.categories.performance).into_any(),
        "security" => t!(i18n, laboratory.categories.security).into_any(),
        "architecture" => t!(i18n, laboratory.categories.architecture).into_any(),
        "rendering" => t!(i18n, laboratory.categories.rendering).into_any(),
        "restapi" => t!(i18n, laboratory.categories.restapi).into_any(),
        _ => slug.to_string().into_any(),
    }
}

#[cfg(feature = "ssr")]
async fn laboratory_svc()
-> Result<std::sync::Arc<dyn modules::laboratory::LaboratoryService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::laboratory::LaboratoryService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn get_lab_category_page(
    category: String,
    page: i64,
) -> Result<(Vec<LaboratoryView>, i64), ServerFnError> {
    laboratory_svc()
        .await?
        .find_by_category_page_async(&category, page, 8)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_lab_by_slug(slug: String) -> Result<Option<LaboratoryView>, ServerFnError> {
    laboratory_svc()
        .await?
        .find_by_slug_async(&slug)  
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
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

/// See `notes::CONTENT_CACHE_TTL_SECS` / `process_localized_cached` doc —
/// same TTL-only rationale (content lives on GitHub, not Postgres).
#[cfg(feature = "ssr")]
const CONTENT_CACHE_TTL_SECS: u64 = 3600;

#[server]
pub async fn fetch_lab_markdown_html(
    url: String,
    locale: String,
) -> Result<MarkdownResult, ServerFnError> {
    let cache = cache_svc().await?;
    let key = format!("lab-content:v1:{locale}:{url}");
    crate::markdown::process_localized_cached(cache.as_ref(), &key, &url, &locale, CONTENT_CACHE_TTL_SECS)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

// ── Pages ────────────────────────────────────────────────────────────────────
#[allow(non_snake_case)]
#[component]
pub fn LaboratoryPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <Seo
            title="Laboratorium — Feri Irawansyah"
            description="Technical experiments and write-ups on performance, security, architecture, and rendering by Feri Irawansyah."
            path="/laboratory"
        />
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-3 block">
                        {t!(i18n, laboratory.eyebrow)}
                    </span>
                    <h1 class="text-[2.25rem] font-extrabold mb-2">{t!(i18n, laboratory.title)}</h1>
                    <p class="text-muted text-[1.05rem]">
                        {t!(i18n, laboratory.subtitle)}
                    </p>
                </header>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-5">
                    {CATEGORIES.iter().map(|(slug, _)| {
                        let href = format!("/laboratory/{slug}");
                        let img_url = category_image_url(slug);
                        view! {
                            <a href=href
                                class="group relative h-52 rounded-2xl overflow-hidden border border-line hover:border-teal-500/50 transition-colors no-underline">
                                <img src=img_url alt="" loading="lazy"
                                    class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"/>
                                <div class="absolute inset-0 bg-linear-to-t from-base/90 via-base/40 to-transparent"></div>
                                <div class="absolute bottom-0 left-0 right-0 p-6">
                                    <h2 class="text-xl font-bold text-white">{category_title(i18n, slug.to_string())}</h2>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn LaboratoryCategoryPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let category = move || params.with(|p| p.get("category").unwrap_or_default());

    let current_page = RwSignal::new(1i64);
    let items = Resource::new(
        move || (category(), current_page.get()),
        |(cat, page)| get_lab_category_page(cat, page),
    );

    let total_pages = Memo::new(move |_| {
        items
            .get()
            .and_then(|r| r.ok())
            .map(|(_, total)| ((total + 7) / 8).max(1))
            .unwrap_or(1)
    });

    let is_first_page = move || current_page.get() <= 1;
    let is_last_page = move || current_page.get() >= total_pages.get();

    view! {
        {move || {
            let cat = category();
            view! {
                <Seo
                    title=format!("{cat} — Laboratorium — Feri Irawansyah")
                    description="Technical experiments by Feri Irawansyah."
                    path=format!("/laboratory/{cat}")
                />
            }
        }}
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <div class="pt-8 pb-4">
                    <a href="/laboratory"
                        class="inline-flex items-center gap-1.5 text-sm text-muted hover:text-fg transition-colors no-underline whitespace-nowrap">
                        <i class="bi bi-arrow-left text-[0.9rem]"></i>
                        {t!(i18n, laboratory.back_to_lab)}
                    </a>
                </div>

                <header class="pb-8">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">{move || category_title(i18n, category())}</h1>
                </header>

                <Suspense fallback=|| view! { <ListRowSkeleton count=4 /> }>
                    {move || items.get().map(|r| match r {
                        Ok((rows, _)) if rows.is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>{t!(i18n, laboratory.empty)}</p>
                            </div>
                        }.into_any(),
                        Ok((rows, _)) => {
                            let cat = category();
                            view! {
                                <div class="flex flex-col gap-4">
                                    {rows.into_iter().map(|lab| {
                                        let href = format!("/laboratory/{cat}/{}", lab.slug);
                                        view! {
                                            <a href=href
                                                class="group flex flex-col gap-2 bg-surface border border-line rounded-2xl p-5 hover:border-teal-500/50 transition-colors no-underline">
                                                <h2 class="text-[1.05rem] font-bold text-fg group-hover:text-teal-500 transition-colors">
                                                    {lab.title.clone()}
                                                </h2>
                                                {(!lab.description.is_empty()).then(|| view! {
                                                    <p class="text-[0.875rem] text-muted line-clamp-2 leading-relaxed">
                                                        {lab.description.clone()}
                                                    </p>
                                                })}
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        },
                        Err(e) => view! {
                            <p class="text-red-400 py-4">{t!(i18n, laboratory.load_error)} {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>

                {move || (total_pages.get() > 1).then(|| view! {
                    <div class="flex items-center justify-center gap-1.5 mt-8">
                        <button
                            disabled=is_first_page
                            on:click=move |_| current_page.update(|p| *p = (*p - 1).max(1))
                            class="w-8 h-8 rounded-full border border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer">
                            <i class="bi bi-arrow-left text-xs"></i>
                        </button>
                        <span class="text-xs text-muted px-2">
                            {move || format!("{} / {}", current_page.get(), total_pages.get())}
                        </span>
                        <button
                            disabled=is_last_page
                            on:click=move |_| {
                                let tp = total_pages.get();
                                current_page.update(|p| if *p < tp { *p += 1; });
                            }
                            class="w-8 h-8 rounded-full border border-line flex items-center justify-center text-muted hover:border-teal-500 hover:text-teal-500 transition-colors disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer">
                            <i class="bi bi-arrow-right text-xs"></i>
                        </button>
                    </div>
                })}
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn LaboratoryDetailPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let category = move || params.with(|p| p.get("category").unwrap_or_default());
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

    let lab = Resource::new_blocking(slug, get_lab_by_slug);

    let content_url = move || {
        lab.get()
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
                fetch_lab_markdown_html(url, locale.to_string()).await
            }
        },
    );

    view! {
        <div class="py-4">
            <div class="max-w-3xl mx-auto px-6">
                <div class="pt-8 pb-4">
                    <a href=move || format!("/laboratory/{}", category())
                        class="inline-flex items-center gap-1.5 text-sm text-muted hover:text-fg transition-colors no-underline whitespace-nowrap">
                        <i class="bi bi-arrow-left text-[0.9rem]"></i>
                        {t!(i18n, laboratory.back_to_category)} " " {move || category_title(i18n, category())}
                    </a>
                </div>

                <Suspense fallback=|| view! {
                    <ArticleHeaderSkeleton with_icon=false />
                    <ContentLinesSkeleton />
                }>
                    {move || lab.get().map(|r| match r {
                        Ok(Some(n)) => view! {
                            <Seo
                                title=format!("{} — Laboratorium — Feri Irawansyah", n.title)
                                description=n.description.clone()
                                path=format!("/laboratory/{}/{}", category(), n.slug)
                            />
                            <header class="pb-8 border-b border-line mb-8">
                                <h1 class="text-[2rem] font-extrabold text-fg">{n.title.clone()}</h1>
                            </header>
                            <Suspense fallback=|| view! { <ContentLinesSkeleton /> }>
                                {move || content_html.get().map(|r| match r {
                                    Ok(result) => view! { <crate::components::MarkdownContent html=result.html /> }.into_any(),
                                    Err(e) => view! {
                                        <p class="text-red-400 text-sm">{t!(i18n, laboratory.content_load_error)} {e.to_string()}</p>
                                    }.into_any(),
                                })}
                            </Suspense>
                        }.into_any(),
                        Ok(None) => view! {
                            <div class="py-24 text-center">
                                <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
                                <p class="text-muted my-4 mb-8">{t!(i18n, laboratory.not_found_body)}</p>
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">{t!(i18n, laboratory.load_error)} {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
