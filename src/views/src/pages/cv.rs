use leptos::prelude::*;
use crate::seo::Seo;

#[allow(non_snake_case)]
#[component]
pub fn CvPage() -> impl IntoView {
    let page_count = Resource::new(|| (), |_| async move {
        #[cfg(feature = "ssr")]
        { typests::cv::cv_page_count().unwrap_or(1) }
        #[cfg(not(feature = "ssr"))]
        { 1usize }
    });

    view! {
        <Seo
            title="CV — Feri Irawansyah"
            description="Curriculum Vitae of Feri Irawansyah — Backend Engineer, Web Developer, AI Engineer."
            path="/cv"
        />
        <div class="sticky top-0 z-40 bg-base/80 backdrop-blur-sm border-b border-line">
            <div class="max-w-4xl mx-auto px-6 py-3 flex items-center justify-between gap-4">
                <a href="/about"
                    class="inline-flex items-center gap-1.5 text-sm text-muted hover:text-teal-400 transition-colors no-underline whitespace-nowrap shrink-0">
                    <i class="bi bi-arrow-left text-[0.9rem]"></i>
                    "Back"
                </a>
                <h1 class="text-lg font-bold text-fg truncate">"Curriculum Vitae"</h1>
                <a
                    href="/cv.pdf?dl=1"
                    download="feri-irawansyah-cv.pdf"
                    class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-teal-600 hover:bg-teal-500 text-white text-sm font-medium transition-colors duration-200 no-underline whitespace-nowrap shrink-0"
                >
                    <i class="bi bi-download"></i>
                    "Download PDF"
                </a>
            </div>
        </div>

        <div class="max-w-4xl mx-auto px-6 py-10">
            <Suspense fallback=move || view! {
                <div class="w-full rounded-xl border border-line bg-surface animate-pulse" style="height: 80vh;"/>
            }>
                {move || page_count.get().map(|count| {
                    (0..count).map(|i| view! {
                        <div class="mb-4 rounded-xl overflow-hidden border border-line shadow-lg bg-white">
                            <img
                                src=format!("/cv/preview/{i}")
                                alt=format!("CV page {}", i + 1)
                                class="w-full h-auto block"
                                loading=if i == 0 { "eager" } else { "lazy" }
                            />
                        </div>
                    }).collect_view()
                })}
            </Suspense>
        </div>
    }
}
