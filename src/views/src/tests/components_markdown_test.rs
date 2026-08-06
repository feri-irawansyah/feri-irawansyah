use crate::components::markdown::{MarkdownContent, MarkdownContentProps};
use leptos::prelude::*;

#[test]
fn renders_html_prop_into_article_content_wrapper() {
    let view = MarkdownContent(
        MarkdownContentProps::builder()
            .html("<p>hello world</p>".to_string())
            .build(),
    );
    let html = view.to_html();
    assert!(html.contains(r#"class="article-content""#));
    assert!(html.contains("hello world"));
}
