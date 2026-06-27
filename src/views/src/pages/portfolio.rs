use leptos::prelude::*;
use modules::portfolio::PortfolioView;

#[server]
pub async fn get_all_portfolio() -> Result<Vec<PortfolioView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::portfolio::PortfolioRepository;
    use repositories::portfolio::PortfolioRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    PortfolioRepositoryImpl::new(pool.get_ref().clone())
        .find_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn PortfolioPage() -> impl IntoView {
    let portfolio = Resource::new(|| (), |_| get_all_portfolio());

    view! {
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">"Portfolio"</h1>
                    <p class="text-muted text-[1.05rem]">
                        "Projects I've built across web, backend, and infrastructure."
                    </p>
                </header>

                <Suspense fallback=|| view! {
                    <div class="text-center text-muted py-8">"Loading projects..."</div>
                }>
                    {move || portfolio.get().map(|r| match r {
                        Ok(items) if items.is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>"No projects added yet. Check back soon."</p>
                            </div>
                        }.into_any(),
                        Ok(items) => view! {
                            <div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-5">
                                {items.into_iter().map(|p| view! {
                                    <article class="bg-surface border border-line rounded-xl overflow-hidden transition-all duration-200 hover:border-indigo-500 hover:-translate-y-0.5">
                                        {(!p.image_src.is_empty()).then(|| view! {
                                            <img class="w-full h-[180px] object-cover"
                                                src=p.image_src.clone() alt=p.title.clone()/>
                                        })}
                                        <div class="p-5">
                                            <div class="flex items-center gap-3 mb-1">
                                                <h3 class="text-[1.1rem] font-semibold text-fg">{p.title}</h3>
                                                {p.pined.then(|| view! {
                                                    <span class="inline-block px-2 py-0.5 rounded-full text-xs font-semibold bg-indigo-500/15 text-indigo-400">
                                                        "Featured"
                                                    </span>
                                                })}
                                            </div>
                                            <p class="text-[0.9rem] text-muted mb-4 leading-relaxed">{p.description}</p>
                                            {(!p.url_docs.is_empty()).then(|| view! {
                                                <a href=p.url_docs.clone() target="_blank"
                                                    class="text-[0.875rem] font-medium text-indigo-500 hover:text-indigo-400">
                                                    "View Project →"
                                                </a>
                                            })}
                                        </div>
                                    </article>
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
