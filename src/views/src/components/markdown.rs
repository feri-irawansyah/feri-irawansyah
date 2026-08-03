use leptos::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn MarkdownContent(html: String) -> impl IntoView {
    view! {
        <div class="article-content" inner_html=html/>
    }
}
