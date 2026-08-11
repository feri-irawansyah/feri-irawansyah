use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

pub const SITE_URL: &str = "https://feri-irawansyah.my.id";
pub const SITE_NAME: &str = "Feri Irawansyah";

#[allow(non_snake_case)]
#[component]
pub fn Seo(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    /// Path only, e.g. "/about" — combined with `SITE_URL` for canonical/og:url.
    #[prop(into)]
    path: String,
    #[prop(optional, into)] image: Option<String>,
    #[prop(optional, into)] og_type: Option<String>,
) -> impl IntoView {
    let image = image.unwrap_or_else(crate::assets::hero_image_url);
    let og_type = og_type.unwrap_or_else(|| "website".to_string());
    let url = format!("{SITE_URL}{path}");

    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=url.clone()/>

        <Meta property="og:type" content=og_type/>
        <Meta property="og:site_name" content=SITE_NAME/>
        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:url" content=url/>
        <Meta property="og:image" content=image.clone()/>

        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=image/>
    }
}
