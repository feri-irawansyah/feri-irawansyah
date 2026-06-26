use leptos::prelude::*;
use modules::certifications::CertView;
use modules::positions::PositionView;

#[server]
pub async fn get_all_positions() -> Result<Vec<PositionView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::positions::PositionRepository;
    use repositories::positions::PositionRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    PositionRepositoryImpl::new(pool.get_ref().clone())
        .find_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_all_certifications() -> Result<Vec<CertView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::certifications::CertRepository;
    use repositories::certifications::CertRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    CertRepositoryImpl::new(pool.get_ref().clone())
        .find_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn ExperiencePage() -> impl IntoView {
    let positions = Resource::new(|| (), |_| get_all_positions());
    let certs     = Resource::new(|| (), |_| get_all_certifications());

    view! {
        <div class="page-experience">
            <div class="container">
                <header class="page-header">
                    <h1>"Experience"</h1>
                    <p>"Work history and certifications."</p>
                </header>

                <section class="exp-section">
                    <h2 class="section-title">"Work History"</h2>
                    <Suspense fallback=|| view! { <div class="loading">"Loading..."</div> }>
                        {move || positions.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="empty-state">"No work history added yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="timeline">
                                    {items.into_iter().map(|p| {
                                        let period = match p.ended_at {
                                            Some(end) => format!(
                                                "{} – {}",
                                                p.started_at.format("%b %Y"),
                                                end.format("%b %Y")
                                            ),
                                            None => format!("{} – Present", p.started_at.format("%b %Y")),
                                        };
                                        view! {
                                            <div class="timeline-item">
                                                <div class="timeline-dot"></div>
                                                <div class="timeline-body">
                                                    <div class="timeline-head">
                                                        <h3 class="timeline-role">{p.role}</h3>
                                                        {p.is_current.then(|| view! {
                                                            <span class="badge badge-green">"Current"</span>
                                                        })}
                                                    </div>
                                                    <p class="timeline-company">
                                                        {p.company}
                                                        {(!p.location.is_empty()).then(|| view! {
                                                            <span class="timeline-location">" · " {p.location}</span>
                                                        })}
                                                    </p>
                                                    <p class="timeline-period">{period}</p>
                                                    {(!p.description.is_empty()).then(|| view! {
                                                        <p class="timeline-desc">{p.description}</p>
                                                    })}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="error-state">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </section>

                <section class="exp-section">
                    <h2 class="section-title">"Certifications"</h2>
                    <Suspense fallback=|| view! { <div class="loading">"Loading..."</div> }>
                        {move || certs.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="empty-state">"No certifications added yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="cert-grid">
                                    {items.into_iter().map(|c| view! {
                                        <div class="cert-card">
                                            {(!c.image_src.is_empty()).then(|| view! {
                                                <img src=c.image_src.clone() alt=c.title.clone() class="cert-img"/>
                                            })}
                                            <div class="cert-body">
                                                <h3 class="cert-title">{c.title}</h3>
                                                <p class="cert-issuer">{c.issuer}</p>
                                                <p class="cert-date">
                                                    {c.issued_at.format("%b %Y").to_string()}
                                                    {c.expired_at.map(|e| format!(" – {}", e.format("%b %Y")))}
                                                </p>
                                                {(!c.credential_url.is_empty()).then(|| view! {
                                                    <a href=c.credential_url.clone() target="_blank" class="card-link">
                                                        "View Certificate →"
                                                    </a>
                                                })}
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
                </section>
            </div>
        </div>
    }
}
