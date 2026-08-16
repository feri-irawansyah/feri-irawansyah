use crate::pages::admin::layout::{ErrorCard, TableErrorRow, TableSkeleton};
use leptos::prelude::*;

// ── ErrorCard ────────────────────────────────────────────────────────────────
#[test]
fn error_card_renders_message() {
    let view = view! {
        <ErrorCard
            message="Gagal memuat data".to_string()
        />
    };
    let html = view.to_html();
    assert!(html.contains("Gagal memuat data"));
    assert!(html.contains("Coba Lagi"));
}

// ── TableErrorRow ────────────────────────────────────────────────────────────
#[test]
fn table_error_row_uses_default_colspan() {
    let view = view! {
        <TableErrorRow
            message="Error".to_string()
        />
    };
    let html = view.to_html();
    assert!(html.contains(r#"colspan="5""#));
    assert!(html.contains("Error"));
}

#[test]
fn table_error_row_respects_custom_colspan() {
    let view = view! {
        <TableErrorRow
            cols=3
            message="Oops".to_string()
        />
    };
    let html = view.to_html();
    assert!(html.contains(r#"colspan="3""#));
}

// ── TableSkeleton ────────────────────────────────────────────────────────────
#[test]
fn table_skeleton_renders_default_rows_and_cols() {
    let view = view! {
        <TableSkeleton />
    };
    let html = view.to_html();
    assert_eq!(html.matches("<tr").count(), 6);
    assert_eq!(html.matches("<td").count(), 6 * 5);
}

#[test]
fn table_skeleton_respects_custom_rows_and_cols() {
    let view = view! {
        <TableSkeleton rows=2 cols=3 />
    };
    let html = view.to_html();
    assert_eq!(html.matches("<tr").count(), 2);
    assert_eq!(html.matches("<td").count(), 2 * 3);
}
