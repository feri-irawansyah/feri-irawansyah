use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="page-home">
            <section class="hero">
                <h1>"Feri Irawansyah"</h1>
                <p class="hero-title">"Full Stack Developer"</p>
                <p class="hero-bio">"Building robust web applications with Rust, TypeScript, and modern tooling."</p>
            </section>
        </div>
    }
}
