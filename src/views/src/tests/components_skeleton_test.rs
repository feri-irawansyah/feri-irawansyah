use crate::components::skeleton::{
    NoteCardSkeleton, NoteCardSkeletonProps, SkillCardSkeleton, SkillCardSkeletonProps,
};
use leptos::prelude::*;

#[test]
fn note_card_skeleton_renders_requested_count() {
    let view = NoteCardSkeleton(NoteCardSkeletonProps::builder().count(4).build());
    let html = view.to_html();
    let card_open = r#"class="flex flex-col sm:flex-row gap-4 sm:gap-5 items-start bg-surface"#;
    assert_eq!(html.matches(card_open).count(), 4);
}

#[test]
fn note_card_skeleton_default_count_matches_documented_default() {
    let view = NoteCardSkeleton(NoteCardSkeletonProps::builder().build());
    let html = view.to_html();
    let card_open = r#"class="flex flex-col sm:flex-row gap-4 sm:gap-5 items-start bg-surface"#;
    // 3 is the documented `#[prop(default = 3)]`.
    assert_eq!(html.matches(card_open).count(), 3);
}

#[test]
fn skill_card_skeleton_renders_requested_count() {
    let view = SkillCardSkeleton(SkillCardSkeletonProps::builder().count(2).build());
    let html = view.to_html();
    let card_open = r#"class="bg-surface border border-line rounded-xl p-6 flex flex-col items-center gap-4""#;
    assert_eq!(html.matches(card_open).count(), 2);
}
