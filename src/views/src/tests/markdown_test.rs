use crate::markdown::{HeadingItem, render};

// ── headings ─────────────────────────────────────────────────────────────────

#[test]
fn no_headings_when_none_present() {
    let result = render("just a paragraph, no headings here").unwrap();
    assert!(result.headings.is_empty());
    assert!(result.html.contains("<p>"));
}

#[test]
fn extracts_single_heading_with_slugified_id() {
    let result = render("## Hello World!").unwrap();
    assert_eq!(
        result.headings,
        vec![HeadingItem {
            level: 2,
            text: "Hello World!".to_string(),
            id: "hello-world".to_string(),
        }]
    );
    assert!(result.html.contains(r#"<h2 id="hello-world">Hello World!</h2>"#));
}

#[test]
fn extracts_multiple_headings_in_order() {
    let md = "# Title\n\nintro\n\n## Section One\n\nbody\n\n### Sub Section";
    let result = render(md).unwrap();
    let levels: Vec<u8> = result.headings.iter().map(|h| h.level).collect();
    let ids: Vec<&str> = result.headings.iter().map(|h| h.id.as_str()).collect();
    assert_eq!(levels, vec![1, 2, 3]);
    assert_eq!(ids, vec!["title", "section-one", "sub-section"]);
}

#[test]
fn slugify_strips_punctuation_and_collapses_separators() {
    let result = render("## Rust & Leptos: A Love Story?!").unwrap();
    assert_eq!(result.headings[0].id, "rust-leptos-a-love-story");
}

// ── fenced code blocks ───────────────────────────────────────────────────────

#[test]
fn highlights_fenced_code_block_with_language_label() {
    let md = "```rust\nfn main() {}\n```";
    let result = render(md).unwrap();
    assert!(result.html.contains(r#"class="code-wrapper""#));
    assert!(result.html.contains(r#"<span class="code-lang">rust</span>"#));
    assert!(result.html.contains(r#"<pre class="code-block"><code>"#));
    // Syntect splits the source into one highlighting `<span>` per token
    // (`fn `, `main`, `() {}`), so the code itself won't appear as one
    // contiguous run — assert on the tokens and that highlighting actually
    // happened instead.
    assert!(result.html.contains("fn "));
    assert!(result.html.contains("main"));
    assert!(result.html.contains("() {}"));
    assert!(result.html.contains("style=\"color:"));
}

#[test]
fn fenced_code_block_without_language_labeled_plain() {
    let md = "```\nno lang here\n```";
    let result = render(md).unwrap();
    assert!(result.html.contains(r#"<span class="code-lang">plain</span>"#));
}

// ── <details><summary> handling ─────────────────────────────────────────────

#[test]
fn extracts_summary_as_heading_and_injects_id_into_html() {
    let md = "<details><summary>Click to expand</summary>\n\nhidden content\n\n</details>";
    let result = render(md).unwrap();

    let summary_heading = result
        .headings
        .iter()
        .find(|h| h.text == "Click to expand")
        .expect("summary should be collected as a heading");
    assert_eq!(summary_heading.level, 2);
    assert_eq!(summary_heading.id, "click-to-expand");

    assert!(
        result
            .html
            .contains(&format!(r#"<summary id="{}""#, summary_heading.id))
    );
}

#[test]
fn strips_nested_tags_from_summary_text() {
    let md = "<details><summary><strong>Bold Title</strong></summary>\n\nbody\n\n</details>";
    let result = render(md).unwrap();
    let summary_heading = result
        .headings
        .iter()
        .find(|h| h.id == "bold-title")
        .expect("nested tags should be stripped before slugifying");
    assert_eq!(summary_heading.text, "Bold Title");
}

#[test]
fn empty_summary_is_not_collected_as_heading() {
    let md = "<details><summary></summary>\n\nbody\n\n</details>";
    let result = render(md).unwrap();
    assert!(result.headings.is_empty());
}
