use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeadingItem {
    pub level: u8,
    pub text: String,
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkdownResult {
    pub html: String,
    pub headings: Vec<HeadingItem>,
}

#[cfg(feature = "ssr")]
pub async fn process(url: &str) -> anyhow::Result<MarkdownResult> {
    use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
    use syntect::highlighting::ThemeSet;
    use syntect::html::highlighted_html_for_string;
    use syntect::parsing::SyntaxSet;

    fn level_num(l: HeadingLevel) -> u8 {
        match l {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }

    fn slugify(s: &str) -> String {
        s.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn strip_html_tags(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut in_tag = false;
        for c in s.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => out.push(c),
                _ => {}
            }
        }
        out
    }

    fn extract_summary_heading(html: &str) -> Option<HeadingItem> {
        let lower = html.to_lowercase();
        let sum_start = lower.find("<summary")?;
        let rel_close = html[sum_start..].find('>')?;
        let content_start = sum_start + rel_close + 1;
        let rel_end = lower[content_start..].find("</summary>")?;
        let text = strip_html_tags(&html[content_start..content_start + rel_end])
            .trim()
            .to_string();
        if text.is_empty() {
            return None;
        }
        Some(HeadingItem {
            level: 2,
            id: slugify(&text),
            text,
        })
    }

    fn inject_summary_id(html: &str, id: &str) -> String {
        let lower = html.to_lowercase();
        if let Some(pos) = lower.find("<summary")
            && let Some(rel_end) = html[pos..].find('>')
        {
            let tag_end = pos + rel_end;
            return format!("{} id=\"{}\"{}", &html[..tag_end], id, &html[tag_end..]);
        }
        html.to_string()
    }

    let md = reqwest::get(url)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    // Pass 1: collect headings + summary headings
    let mut headings: Vec<HeadingItem> = Vec::new();
    let mut sum_headings: Vec<HeadingItem> = Vec::new();
    {
        let mut in_heading: Option<(u8, String)> = None;
        for event in Parser::new_ext(&md, Options::all()) {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = Some((level_num(level), String::new()));
                }
                Event::Text(ref text) | Event::Code(ref text) => {
                    if let Some((_, ref mut buf)) = in_heading {
                        buf.push_str(text);
                    }
                }
                Event::End(TagEnd::Heading(_)) => {
                    if let Some((lvl, text)) = in_heading.take() {
                        headings.push(HeadingItem {
                            level: lvl,
                            id: slugify(&text),
                            text,
                        });
                    }
                }
                Event::Html(ref html) => {
                    if let Some(item) = extract_summary_heading(html) {
                        sum_headings.push(item.clone());
                        headings.push(item);
                    }
                }
                _ => {}
            }
        }
    }

    // Pass 2: generate HTML with heading ids, summary ids, and syntax highlighting
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut events: Vec<Event> = Vec::new();
    let mut code_buf: Option<(String, String)> = None;
    let mut h_idx = 0usize;
    let mut s_idx = 0usize;

    for event in Parser::new_ext(&md, Options::all()) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let n = level_num(level);
                let id = headings.get(h_idx).map(|h| h.id.as_str()).unwrap_or("");
                events.push(Event::Html(format!("<h{n} id=\"{id}\">").into()));
            }
            Event::End(TagEnd::Heading(level)) => {
                let n = level_num(level);
                events.push(Event::Html(format!("</h{n}>").into()));
                h_idx += 1;
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                code_buf = Some((lang.into_string(), String::new()));
            }
            Event::Text(text) => {
                if let Some((_, ref mut buf)) = code_buf {
                    buf.push_str(&text);
                } else {
                    events.push(Event::Text(text));
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((lang, code)) = code_buf.take() {
                    let syntax = ps
                        .find_syntax_by_token(&lang)
                        .unwrap_or_else(|| ps.find_syntax_plain_text());
                    let full_html = highlighted_html_for_string(&code, &ps, syntax, theme)
                        .unwrap_or_else(|_| {
                            let escaped = code
                                .replace('&', "&amp;")
                                .replace('<', "&lt;")
                                .replace('>', "&gt;");
                            format!("<pre>{}</pre>", escaped)
                        });
                    let inner = if let (Some(s), Some(e)) =
                        (full_html.find('>'), full_html.rfind("</pre>"))
                    {
                        full_html[s + 1..e].to_string()
                    } else {
                        full_html
                    };
                    let display_lang = if lang.is_empty() {
                        "plain".to_string()
                    } else {
                        lang
                    };
                    let block = format!(
                        r#"<div class="code-wrapper"><div class="code-header"><span class="code-lang">{display_lang}</span><button class="copy-btn" onclick="var c=this.closest('.code-wrapper').querySelector('code');navigator.clipboard.writeText(c.innerText).then(()=>{{this.textContent='Copied!';setTimeout(()=>this.textContent='Copy',1500)}})">Copy</button></div><pre class="code-block"><code>{inner}</code></pre></div>"#
                    );
                    events.push(Event::Html(block.into()));
                }
            }
            Event::Html(html) => {
                let lower = html.to_lowercase();
                if lower.contains("<summary") && s_idx < sum_headings.len() {
                    let id = sum_headings[s_idx].id.clone();
                    s_idx += 1;
                    events.push(Event::Html(inject_summary_id(&html, &id).into()));
                } else {
                    events.push(Event::Html(html));
                }
            }
            _ => events.push(event),
        }
    }

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    Ok(MarkdownResult { html, headings })
}
