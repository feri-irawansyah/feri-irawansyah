use leptos::prelude::*;
use modules::portfolio::PortfolioView;
use modules::skills::SkillView;

#[server]
pub async fn get_featured_portfolio() -> Result<Vec<PortfolioView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::portfolio::PortfolioRepository;
    use repositories::portfolio::PortfolioRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    PortfolioRepositoryImpl::new(pool.get_ref().clone())
        .find_featured()
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
    let portfolio = Resource::new(|| (), |_| get_featured_portfolio());
    let skills = Resource::new(|| (), |_| get_skills());

    view! {
        <div class="page-home">
            <section class="hero">
                <div class="hero-inner">
                    <p class="hero-eyebrow">"Hi, I'm"</p>
                    <h1 class="hero-name">"Feri Irawansyah"</h1>
                    <p class="hero-role">"Principal Engineer"</p>
                    <p class="hero-bio">
                        "I build robust web apps with Rust, TypeScript, and modern tooling. "
                        "Passionate about clean architecture and performance."
                    </p>
                    <div class="hero-cta">
                        <a href="/portfolio" class="btn btn-primary">"See Projects"</a>
                        <a href="/experience" class="btn btn-outline">"Experience"</a>
                    </div>
                </div>
            </section>

            <section class="section" id="featured">
                <div class="container">
                    <h2 class="section-title">"Featured Projects"</h2>
                    <Suspense fallback=|| view! { <div class="loading">"Loading projects..."</div> }>
                        {move || portfolio.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="empty-state">"No featured projects yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="card-grid">
                                    {items.into_iter().map(|p| view! {
                                        <article class="card">
                                            {(!p.image_src.is_empty()).then(|| view! {
                                                <img class="card-img" src=p.image_src.clone() alt=p.title.clone()/>
                                            })}
                                            <div class="card-body">
                                                <h3 class="card-title">{p.title}</h3>
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
                    <div class="section-footer">
                        <a href="/portfolio" class="btn btn-outline">"All Projects →"</a>
                    </div>
                </div>
            </section>

            <section class="section section-alt" id="skills">
                <div class="container">
                    <h2 class="section-title">"Tech Stack"</h2>
                    <Suspense fallback=|| view! { <div class="loading">"Loading skills..."</div> }>
                        {move || skills.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="empty-state">"No skills added yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="skills-grid">
                                    {items.into_iter().map(|s| view! {
                                        <div class="skill-chip">
                                            {(!s.image_src.is_empty()).then(|| view! {
                                                <img src=s.image_src.clone() alt=s.title.clone() class="skill-icon"/>
                                            })}
                                            <span class="skill-name">{s.title}</span>
                                            <div class="skill-bar">
                                                <div class="skill-fill" style=format!("width:{}%", s.progress)></div>
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="error-state">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </div>
            </section>
        </div>
    }
}
