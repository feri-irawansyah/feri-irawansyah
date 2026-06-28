use leptos::prelude::*;
use modules::skills::SkillView;

#[cfg(feature = "ssr")]
async fn skill_svc() -> Result<std::sync::Arc<dyn modules::skills::SkillService>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Arc;
    let svc = extract::<Data<Arc<dyn modules::skills::SkillService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(Arc::clone(&svc))
}

#[server]
pub async fn get_all_skills() -> Result<Vec<SkillView>, ServerFnError> {
    skill_svc().await?.list().await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn SkillsPage() -> impl IntoView {
    let skills = Resource::new(|| (), |_| get_all_skills());

    view! {
        <div class="py-4">
            <div class="max-w-6xl mx-auto px-12">
                <header class="py-16 pb-10">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">"Tech Stack"</h1>
                    <p class="text-muted text-[1.05rem]">
                        "Technologies and tools I use to build robust applications."
                    </p>
                </header>

                <Suspense fallback=|| view! {
                    <div class="text-center text-muted py-16">"Loading skills..."</div>
                }>
                    {move || skills.get().map(|r| match r {
                        Ok(items) if items.is_empty() => view! {
                            <p class="text-center text-muted py-16">"No skills added yet."</p>
                        }.into_any(),
                        Ok(items) => view! {
                            <div class="grid grid-cols-[repeat(auto-fill,minmax(160px,1fr))] gap-5 pb-16">
                                {items.into_iter().map(|s| {
                                    let pct = s.progress.min(100);
                                    let circ = 188.5_f32;
                                    let filled = circ * pct as f32 / 100.0;
                                    view! {
                                        <div class="bg-surface border border-line rounded-xl p-5 flex flex-col items-center gap-3 hover:border-teal-500 transition-colors">
                                            // Circular progress SVG
                                            <div class="relative w-20 h-20">
                                                <svg class="w-full h-full -rotate-90" viewBox="0 0 80 80">
                                                    <circle cx="40" cy="40" r="30"
                                                        fill="none" stroke="#2a2d35" stroke-width="6"/>
                                                    <circle cx="40" cy="40" r="30"
                                                        fill="none" stroke="#7c3aed" stroke-width="6"
                                                        stroke-linecap="round"
                                                        stroke-dasharray=format!("{:.1} {:.1}", filled, circ)
                                                        stroke-dashoffset="0"/>
                                                </svg>
                                                <div class="absolute inset-0 flex items-center justify-center">
                                                    {(!s.image_src.is_empty()).then(|| view! {
                                                        <img src=s.image_src.clone() alt=s.title.clone()
                                                            class="w-8 h-8 object-contain"/>
                                                    })}
                                                </div>
                                            </div>
                                            <p class="text-lg font-extrabold text-teal-500">
                                                {format!("{}%", pct)}
                                            </p>
                                            <p class="text-[0.85rem] text-muted font-medium text-center">{s.title}</p>
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
            </div>
        </div>
    }
}
