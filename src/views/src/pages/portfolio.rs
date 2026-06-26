use leptos::prelude::*;
use modules::portfolio::PortfolioView;

#[server]
pub async fn get_all_portfolio() -> Result<Vec<PortfolioView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::portfolio::PortfolioRepository;
    use repositories::portfolio::PortfolioRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>().await
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
        <div class="page-portfolio">
            <div class="container">
                <header class="page-header">
                    <h1>"Portfolio"</h1>
                    <p>"Projects I've built across web, backend, and infrastructure."</p>
                </header>

                <Suspense fallback=|| view! { <div class="loading">"Loading projects..."</div> }>
                    {move || portfolio.get().map(|r| match r {
                        Ok(items) if items.is_empty() => view! {
                            <div class="empty-state">
                                <p>"No projects added yet. Check back soon."</p>
                            </div>
                        }.into_any(),
                        Ok(items) => view! {
                            <div class="card-grid">
                                {items.into_iter().map(|p| view! {
                                    <article class="card">
                                        {(!p.image_src.is_empty()).then(|| view! {
                                            <img class="card-img" src=p.image_src.clone() alt=p.title.clone()/>
                                        })}
                                        <div class="card-body">
                                            <div class="card-head">
                                                <h3 class="card-title">{p.title}</h3>
                                                {p.featured.then(|| view! {
                                                    <span class="badge">"Featured"</span>
                                                })}
                                            </div>
                                            <p class="card-desc">{p.description}</p>
                                            {(!p.url_docs.is_empty()).then(|| view! {
                                                <a href=p.url_docs.clone() target="_blank" class="card-link">
                                                    "View Project →"
                                                </a>
                                            })}
                                        </div>
                                    </article>
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="error-state">"Failed to load: " {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
