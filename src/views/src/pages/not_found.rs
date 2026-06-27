use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="py-24 text-center">
            <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
            <p class="text-muted my-4 mb-8">"Page not found."</p>
            <a href="/"
                class="inline-flex items-center gap-1.5 px-[1.4rem] py-[0.6rem] border border-line text-muted hover:border-indigo-500 hover:text-fg rounded text-[0.9rem] font-medium transition-colors">
                "Back to home"
            </a>
        </div>
    }
}
