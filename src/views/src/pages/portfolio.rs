use crate::i18n::*;
use crate::seo::Seo;
use leptos::prelude::*;
use modules::portfolio::PortfolioView;

#[cfg(feature = "ssr")]
async fn portfolio_svc()
-> Result<std::sync::Arc<dyn modules::portfolio::PortfolioService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::portfolio::PortfolioService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn get_all_portfolio() -> Result<Vec<PortfolioView>, ServerFnError> {
    portfolio_svc()
        .await?
        .list()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[allow(non_snake_case)]
#[component]
pub fn PortfolioPage() -> impl IntoView {
    let i18n = use_i18n();
    let portfolio = Resource::new(|| (), |_| get_all_portfolio());

    view! {
        <Seo
            title="Portfolio — Feri Irawansyah"
            description="Projects built by Feri Irawansyah across web, backend, and infrastructure — Rust, Leptos, and production-grade systems."
            path="/portfolio"
        />
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">{t!(i18n, portfolio.title)}</h1>
                    <p class="text-muted text-[1.05rem]">
                        {t!(i18n, portfolio.subtitle)}
                    </p>
                </header>

                <Suspense fallback=move || view! {
                    <div class="text-center text-muted py-8">{t!(i18n, portfolio.loading)}</div>
                }>
                    {move || portfolio.get().map(|r| match r {
                        Ok(items) if items.is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>{t!(i18n, portfolio.empty)}</p>
                            </div>
                        }.into_any(),
                        Ok(items) => view! {
                            <div class="grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] gap-6">
                                {items.into_iter().map(|p| view! {
                                    <article class="group relative bg-surface border border-line rounded-2xl overflow-hidden transition-all duration-300 hover:border-teal-500/50 hover:shadow-xl hover:shadow-teal-500/5 hover:-translate-y-1">
                                        <div class="relative w-full h-47.5 overflow-hidden bg-line">
                                            {if !p.image_src.is_empty() {
                                                view! {
                                                    <img class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                                                        src=p.image_src.clone() alt=p.title.clone()/>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="w-full h-full flex items-center justify-center bg-linear-to-br from-teal-500/15 to-transparent">
                                                        <i class="bi bi-code-slash text-4xl text-teal-500/40"></i>
                                                    </div>
                                                }.into_any()
                                            }}
                                            <div class="absolute inset-0 bg-linear-to-t from-base/80 via-transparent to-transparent pointer-events-none"></div>
                                            {p.pined.then(|| view! {
                                                <span class="absolute top-3 right-3 inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-semibold bg-teal-500 text-white shadow-md whitespace-nowrap">
                                                    <i class="bi bi-star-fill text-[0.65rem]"></i>
                                                    {t!(i18n, portfolio.featured)}
                                                </span>
                                            })}
                                        </div>
                                        <div class="p-6">
                                            <div class="flex items-center justify-between gap-3 mb-2">
                                                <h3 class="text-[1.1rem] font-bold text-fg group-hover:text-teal-500 transition-colors">{p.title}</h3>
                                                <span class="text-xs text-muted shrink-0">{p.last_update.format("%b %Y").to_string()}</span>
                                            </div>
                                            <p class="text-[0.9rem] text-muted mb-5 leading-relaxed line-clamp-2">{p.description}</p>
                                            {(!p.url_docs.is_empty()).then(|| view! {
                                                <a href=p.url_docs.clone() target="_blank"
                                                    class="inline-flex items-center gap-1.5 text-[0.85rem] font-semibold text-teal-500 hover:gap-2.5 transition-all no-underline whitespace-nowrap">
                                                    {t!(i18n, portfolio.view_project)}
                                                    <i class="bi bi-arrow-right text-[0.8rem]"></i>
                                                </a>
                                            })}
                                        </div>
                                    </article>
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">{t!(i18n, portfolio.load_error)} {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
