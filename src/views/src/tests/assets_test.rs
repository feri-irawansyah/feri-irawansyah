use crate::assets::{
    ASSET_BASE, asset_url, hero_image_url, note_cover_url, note_default_cover_url,
};

#[test]
fn asset_url_joins_base_and_path() {
    assert_eq!(
        asset_url("fullstack.webp"),
        format!("{ASSET_BASE}/fullstack.webp")
    );
}

#[test]
fn hero_image_url_points_at_hero_bg() {
    assert_eq!(hero_image_url(), format!("{ASSET_BASE}/hero-bg.webp"));
}

#[test]
fn note_cover_url_is_keyed_by_slug() {
    assert_eq!(
        note_cover_url("rust-async"),
        format!("{ASSET_BASE}/notes/rust-async.webp")
    );
}

#[test]
fn note_default_cover_url_is_a_fixed_fallback() {
    assert_eq!(
        note_default_cover_url(),
        format!("{ASSET_BASE}/notes/default.webp")
    );
}
