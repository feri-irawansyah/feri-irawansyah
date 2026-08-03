#![recursion_limit = "512"]

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod features;
pub mod markdown;
pub mod pages;
pub mod seo;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    leptos::mount::hydrate_body(App);
}
