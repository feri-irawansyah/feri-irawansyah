use crate::pages::admin::layout::{
    ErrorCard, ErrorCardProps, TableErrorRow, TableErrorRowProps, TableSkeleton,
    TableSkeletonProps,
};
use leptos::prelude::*;

// ── ErrorCard ────────────────────────────────────────────────────────────────
#[test]
fn error_card_renders_message() {
    let view = ErrorCard(
        ErrorCardProps::builder()
            .message("Gagal memuat data".to_string())
            .build(),
    );
    let html = view.to_html();
    assert!(html.contains("Gagal memuat data"));
    assert!(html.contains("Coba Lagi"));
}

// ── TableErrorRow ────────────────────────────────────────────────────────────
#[test]
fn table_error_row_uses_default_colspan() {
    let view = TableErrorRow(
        TableErrorRowProps::builder()
            .message("Error".to_string())
            .build(),
    );
    let html = view.to_html();
    assert!(html.contains(r#"colspan="5""#));
    assert!(html.contains("Error"));
}

#[test]
fn table_error_row_respects_custom_colspan() {
    let view = TableErrorRow(
        TableErrorRowProps::builder()
            .cols(3)
            .message("Oops".to_string())
            .build(),
    );
    let html = view.to_html();
    assert!(html.contains(r#"colspan="3""#));
}

// ── TableSkeleton ────────────────────────────────────────────────────────────
#[test]
fn table_skeleton_renders_default_rows_and_cols() {
    let view = TableSkeleton(TableSkeletonProps::builder().build());
    let html = view.to_html();
    assert_eq!(html.matches("<tr").count(), 6);
    assert_eq!(html.matches("<td").count(), 6 * 5);
}

#[test]
fn table_skeleton_respects_custom_rows_and_cols() {
    let view = TableSkeleton(TableSkeletonProps::builder().rows(2).cols(3).build());
    let html = view.to_html();
    assert_eq!(html.matches("<tr").count(), 2);
    assert_eq!(html.matches("<td").count(), 2 * 3);
}
