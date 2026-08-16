//! Centralizes the Supabase Storage bucket base URL for public assets, so a
//! bucket/provider migration is a one-line change here instead of a
//! grep-replace across every page rendering an `<img>`.
//!
//! Read-side counterpart to `connectors::supabase::StorageStore`, which only
//! covers the *write* (upload) side. Pages never depend on `connectors`
//! directly — see CLAUDE.md: pages reach DB/cache/storage only through a
//! service trait object — but a public asset *URL* isn't a DB/cache/storage
//! read, it's a string a `<img src>` needs, so a plain constant here (rather
//! than round-tripping through a service for a value that never changes at
//! runtime) is the right layer for it.

/// Overridable at compile time via `SUPABASE_ASSET_BASE_URL` (e.g. if the
/// bucket or project ever changes) — falls back to the current bucket so
/// nothing needs to be set for a normal build.
pub const ASSET_BASE: &str = match option_env!("SUPABASE_ASSET_BASE_URL") {
    Some(url) => url,
    None => {
        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img"
    }
};

/// Builds a full asset URL from a path relative to `ASSET_BASE`
/// (e.g. `"notes/some-slug.webp"` or `"fullstack.webp"`).
pub fn asset_url(path: &str) -> String {
    format!("{ASSET_BASE}/{path}").to_string()
}

/// The hero/profile image reused across SEO tags, structured data, and the about/home pages.
pub fn hero_image_url() -> String {
    asset_url("hero-bg.webp")
}

/// Cover image for a note/journey entry, keyed by slug.
pub fn note_cover_url(slug: &str) -> String {
    asset_url(&format!("notes/{slug}.webp"))
}

/// Fallback shown when a note's cover image 404s.
pub fn note_default_cover_url() -> String {
    asset_url("notes/default.webp")
}
