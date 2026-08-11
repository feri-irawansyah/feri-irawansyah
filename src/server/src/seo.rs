use views::seo::{SITE_NAME, SITE_URL};

/// Static routes worth listing in the sitemap (dynamic note slugs are appended separately).
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

pub async fn robots_txt() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!(
            "User-agent: *\nAllow: /\n\nSitemap: {SITE_URL}/sitemap.xml\n"
        ))
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub async fn rss_xml(
    note_svc: actix_web::web::Data<std::sync::Arc<dyn modules::notes::NoteService>>,
) -> impl actix_web::Responder {
    let notes = note_svc.find_all_async().await.unwrap_or_default();

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

pub async fn sitemap_xml(
    note_svc: actix_web::web::Data<std::sync::Arc<dyn modules::notes::NoteService>>,
) -> impl actix_web::Responder {
    let notes = note_svc.find_all_async().await.unwrap_or_default();

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
