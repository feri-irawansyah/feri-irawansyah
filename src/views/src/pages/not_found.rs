use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="page-not-found">
            <h1>"404"</h1>
            <p>"Page not found."</p>
            <a href="/">"Back to home"</a>
        </div>
    }
}
