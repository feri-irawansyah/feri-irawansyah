use leptos::prelude::*;

use crate::seo::Seo;

const PDF_PATH: &str = "/public/cv/feri-irawansyah.pdf";

#[allow(non_snake_case)]
#[component]
pub fn CvPage() -> impl IntoView {
    view! {
        <Seo
            title="CV — Feri Irawansyah"
            description="Curriculum Vitae Feri Irawansyah — Rust programmer dan Principal Engineer. Preview dan download PDF."
            path="/cv"
        />
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6 py-12">

                <div class="flex items-center justify-between mb-6">
                    <div>
                        <span class="text-xs font-semibold text-teal-500 uppercase tracking-widest mb-2 block">
                            "Curriculum Vitae"
                        </span>
                        <h1 class="text-2xl font-extrabold text-fg">"Feri Irawansyah"</h1>
                    </div>
                    <a
                        href=PDF_PATH
                        download="feri-irawansyah-cv.pdf"
                        class="flex items-center gap-2 px-5 py-2.5 bg-teal-600 hover:bg-teal-500 text-white rounded-lg text-sm font-semibold transition-colors no-underline"
                    >
                        <i class="bi bi-download"></i>
                        "Download PDF"
                    </a>
                </div>

                <div class="bg-surface border border-line rounded-2xl overflow-hidden">
                    <object
                        data=PDF_PATH
                        type="application/pdf"
                        class="w-full"
                        style="height: 85vh; min-height: 600px;"
                    >
                        <div class="flex flex-col items-center justify-center gap-4 py-20 text-center px-6">
                            <div class="w-16 h-16 rounded-2xl bg-teal-500/10 flex items-center justify-center">
                                <i class="bi bi-file-pdf text-teal-500 text-3xl"></i>
                            </div>
                            <p class="text-muted text-sm max-w-xs">
                                "Browser kamu tidak mendukung preview PDF. Silakan download untuk melihatnya."
                            </p>
                            <a
                                href=PDF_PATH
                                download="feri-irawansyah-cv.pdf"
                                class="flex items-center gap-2 px-5 py-2.5 bg-teal-600 hover:bg-teal-500 text-white rounded-lg text-sm font-semibold transition-colors no-underline"
                            >
                                <i class="bi bi-download"></i>
                                "Download PDF"
                            </a>
                        </div>
                    </object>
                </div>

            </div>
        </div>
    }
}
