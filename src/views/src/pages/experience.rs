use leptos::prelude::*;
use modules::certifications::CertView;
use modules::positions::PositionView;

#[server]
pub async fn get_all_positions() -> Result<Vec<PositionView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::positions::PositionRepository;
    use repositories::positions::PositionRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
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
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    CertRepositoryImpl::new(pool.get_ref().clone())
        .find_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn ExperiencePage() -> impl IntoView {
    let positions = Resource::new(|| (), |_| get_all_positions());
    let certs = Resource::new(|| (), |_| get_all_certifications());

    view! {
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">"Experience"</h1>
                    <p class="text-muted text-[1.05rem]">"Work history and certifications."</p>
                </header>

                // ── Work History ────────────────────────────────────────────────
                <section class="mb-16">
                    <h2 class="text-2xl font-bold mb-8 text-fg">"Work History"</h2>
                    <Suspense fallback=|| view! {
                        <div class="text-center text-muted py-8">"Loading..."</div>
                    }>
                        {move || positions.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="text-center text-muted py-12">"No work history added yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="flex flex-col">
                                    {items.into_iter().map(|p| {
                                        let is_current = p.end_date.is_none();
                                        let period = match p.end_date {
                                            Some(end) => format!(
                                                "{} – {}",
                                                p.start_date.format("%b %Y"),
                                                end.format("%b %Y")
                                            ),
                                            None => format!("{} – Present", p.start_date.format("%b %Y")),
                                        };
                                        let desc = p.description.clone();
                                        view! {
                                            <div class="relative flex gap-6 pb-8 before:content-[''] before:absolute before:left-[7px] before:top-5 before:bottom-0 before:w-0.5 before:bg-line last:before:hidden">
                                                <div class="w-4 h-4 rounded-full bg-indigo-500 border-[3px] border-base shrink-0 mt-1 z-10"></div>
                                                <div class="flex-1">
                                                    <div class="flex items-center gap-3 mb-1">
                                                        <h3 class="text-[1.05rem] font-semibold text-fg">{p.title}</h3>
                                                        {is_current.then(|| view! {
                                                            <span class="inline-block px-2 py-0.5 rounded-full text-xs font-semibold bg-green-500/15 text-green-500">
                                                                "Current"
                                                            </span>
                                                        })}
                                                        <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium bg-line text-muted">
                                                            {p.job_position.clone()}
                                                        </span>
                                                        <span class="inline-block px-2 py-0.5 rounded-full text-xs font-medium bg-line text-muted">
                                                            {p.job_type.clone()}
                                                        </span>
                                                    </div>
                                                    <div class="flex items-center gap-2 mb-1">
                                                        {(!p.image_src.is_empty()).then(|| view! {
                                                            <img src=p.image_src.clone() alt=p.company.clone()
                                                                class="w-4 h-4 object-contain rounded-sm"/>
                                                        })}
                                                        <p class="text-[0.9rem] text-muted">
                                                            {p.company.clone()}
                                                            {(!p.address.is_empty()).then(|| view! {
                                                                <span class="text-muted">" · " {p.address.clone()}</span>
                                                            })}
                                                        </p>
                                                    </div>
                                                    <p class="text-xs text-muted mb-2">{period}</p>
                                                    {(!desc.is_empty()).then(|| view! {
                                                        <ul class="list-disc list-inside space-y-0.5 mt-1">
                                                            {desc.into_iter().map(|item| view! {
                                                                <li class="text-[0.875rem] text-muted">{item}</li>
                                                            }).collect_view()}
                                                        </ul>
                                                    })}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="text-red-400 py-4">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </section>

                // ── Certifications ──────────────────────────────────────────────
                <section class="mb-16">
                    <h2 class="text-2xl font-bold mb-8 text-fg">"Certifications"</h2>
                    <Suspense fallback=|| view! {
                        <div class="text-center text-muted py-8">"Loading..."</div>
                    }>
                        {move || certs.get().map(|r| match r {
                            Ok(items) if items.is_empty() => view! {
                                <p class="text-center text-muted py-12">"No certifications added yet."</p>
                            }.into_any(),
                            Ok(items) => view! {
                                <div class="grid grid-cols-[repeat(auto-fill,minmax(260px,1fr))] gap-4">
                                    {items.into_iter().map(|c| view! {
                                        <div class="bg-surface border border-line rounded-xl p-5 flex gap-4 items-start">
                                            {(!c.image_src.is_empty()).then(|| view! {
                                                <img src=c.image_src.clone() alt=c.title.clone()
                                                    class="w-12 h-12 object-contain rounded shrink-0"/>
                                            })}
                                            <div class="flex-1 min-w-0">
                                                <h3 class="text-[0.95rem] font-semibold mb-0.5">{c.title}</h3>
                                                <p class="text-sm text-muted">{c.description.clone()}</p>
                                                <p class="text-xs text-muted mt-0.5 mb-3">
                                                    {c.start_date.format("%b %Y").to_string()}
                                                </p>
                                                {(!c.url_docs.is_empty()).then(|| view! {
                                                    <a href=c.url_docs.clone() target="_blank"
                                                        class="text-[0.875rem] font-medium text-indigo-500 hover:text-indigo-400">
                                                        "View Certificate →"
                                                    </a>
                                                })}
                                            </div>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p class="text-red-400 py-4">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </section>
            </div>
        </div>
    }
}
