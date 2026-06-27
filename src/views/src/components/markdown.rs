use leptos::prelude::*;

#[component]
pub fn MarkdownContent(html: String) -> impl IntoView {
    view! {
        <div class="article-content" inner_html=html/>
    }
}
