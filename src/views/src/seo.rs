use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

pub const SITE_URL: &str = "https://feri-irawansyah.my.id";
pub const SITE_NAME: &str = "Feri Irawansyah";
pub const DEFAULT_OG_IMAGE: &str = "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/hero-bg.webp";

/// Renders the full per-page SEO tag set: title, description, canonical link,
/// Open Graph, and Twitter Card tags. Nested inside a route, it overrides the
/// site-wide defaults set in `<App>` for that route only.
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
    let image = image.unwrap_or_else(|| DEFAULT_OG_IMAGE.to_string());
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

/// Static routes worth listing in the sitemap (dynamic note slugs are appended separately).
#[cfg(feature = "ssr")]
const STATIC_ROUTES: &[&str] = &[
    "/",
    "/about",
    "/portfolio",
    "/experience",
    "/skills",
    "/notes",
    "/contact",
    "/about/journey/background",
    "/about/journey/meditate",
    "/about/journey/educational",
    "/about/journey/snakesystem",
];

#[cfg(feature = "ssr")]
pub async fn robots_txt() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!(
            "User-agent: *\nAllow: /\n\nSitemap: {SITE_URL}/sitemap.xml\n"
        ))
}

#[cfg(feature = "ssr")]
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(feature = "ssr")]
pub async fn rss_xml(
    note_svc: actix_web::web::Data<std::sync::Arc<dyn modules::notes::NoteService>>,
) -> impl actix_web::Responder {
    let notes = note_svc.list().await.unwrap_or_default();

    let items: String = notes
        .iter()
        .map(|n| {
            format!(
                "  <item>\n    <title>{}</title>\n    <link>{SITE_URL}/notes/{}</link>\n    <description>{}</description>\n    <pubDate>{}</pubDate>\n    <guid>{SITE_URL}/notes/{}</guid>\n  </item>\n",
                xml_escape(&n.title),
                n.slug,
                xml_escape(&n.description),
                n.last_update.to_rfc2822(),
                n.slug,
            )
        })
        .collect();

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{SITE_NAME} — Notes</title>
    <link>{SITE_URL}/notes</link>
    <description>Notes tentang Rust, engineering, dan teknologi oleh {SITE_NAME}.</description>
    <language>id</language>
    <atom:link href="{SITE_URL}/rss.xml" rel="self" type="application/rss+xml"/>
{items}  </channel>
</rss>"#
    );

    actix_web::HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(xml)
}

#[cfg(feature = "ssr")]
pub async fn sitemap_xml(
    note_svc: actix_web::web::Data<std::sync::Arc<dyn modules::notes::NoteService>>,
) -> impl actix_web::Responder {
    let notes = note_svc.list().await.unwrap_or_default();

    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for path in STATIC_ROUTES {
        xml.push_str(&format!("  <url><loc>{SITE_URL}{path}</loc></url>\n"));
    }
    for n in notes {
        xml.push_str(&format!(
            "  <url><loc>{SITE_URL}/notes/{}</loc><lastmod>{}</lastmod></url>\n",
            n.slug,
            n.last_update.format("%Y-%m-%d")
        ));
    }
    xml.push_str("</urlset>");

    actix_web::HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml)
}
