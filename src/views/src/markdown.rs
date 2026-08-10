use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeadingItem {
    pub level: u8,
    pub text: String,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkdownResult {
    pub html: String,
    pub headings: Vec<HeadingItem>,
}

#[cfg(feature = "ssr")]
fn http_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(8))
            .build()
            .expect("failed to build markdown http client")
    })
}

#[cfg(feature = "ssr")]
pub(crate) struct GithubCircuit {
    consecutive_failures: std::sync::atomic::AtomicU32,
    open_until_epoch_ms: std::sync::atomic::AtomicI64,
}

#[cfg(feature = "ssr")]
pub(crate) const CIRCUIT_FAILURE_THRESHOLD: u32 = 5;
#[cfg(feature = "ssr")]
const CIRCUIT_OPEN_COOLDOWN_MS: i64 = 30_000;

#[cfg(feature = "ssr")]
impl GithubCircuit {
    pub(crate) const fn new() -> Self {
        Self {
            consecutive_failures: std::sync::atomic::AtomicU32::new(0),
            open_until_epoch_ms: std::sync::atomic::AtomicI64::new(0),
        }
    }

    fn now_ms() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    pub(crate) fn allow_request(&self) -> bool {
        let open_until = self
            .open_until_epoch_ms
            .load(std::sync::atomic::Ordering::Relaxed);
        open_until == 0 || Self::now_ms() >= open_until
    }

    pub(crate) fn record_success(&self) {
        self.consecutive_failures
            .store(0, std::sync::atomic::Ordering::Relaxed);
        self.open_until_epoch_ms
            .store(0, std::sync::atomic::Ordering::Relaxed);
    }

    /// Only call for failures that indicate GitHub/network trouble — a 404
    /// (note has no translation, etc.) is not that, and must not trip this.
    pub(crate) fn record_transient_failure(&self) {
        let failures = self
            .consecutive_failures
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1;
        if failures >= CIRCUIT_FAILURE_THRESHOLD {
            self.open_until_epoch_ms.store(
                Self::now_ms() + CIRCUIT_OPEN_COOLDOWN_MS,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
    }
}

#[cfg(feature = "ssr")]
static GITHUB_CIRCUIT: GithubCircuit = GithubCircuit::new();

#[cfg(feature = "ssr")]
const FETCH_MAX_ATTEMPTS: u32 = 3;
#[cfg(feature = "ssr")]
const FETCH_BASE_BACKOFF_MS: u64 = 200;

#[cfg(feature = "ssr")]
fn is_retryable(err: &reqwest::Error) -> bool {
    if err.is_timeout() || err.is_connect() {
        return true;
    }
    match err.status() {
        Some(status) => status.is_server_error(),
        None => true,
    }
}

#[cfg(feature = "ssr")]
async fn fetch_once(url: &str) -> Result<String, reqwest::Error> {
    http_client()
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await
}

#[cfg(feature = "ssr")]
async fn fetch_markdown_source(url: &str) -> anyhow::Result<String> {
    if !GITHUB_CIRCUIT.allow_request() {
        anyhow::bail!(
            "GitHub fetch circuit open — too many recent failures, skipping network call"
        );
    }

    let mut last_err: Option<reqwest::Error> = None;
    for attempt in 0..FETCH_MAX_ATTEMPTS {
        if attempt > 0 {
            let backoff_ms = FETCH_BASE_BACKOFF_MS * 2u64.pow(attempt - 1);
            tokio::time::sleep(std::time::Duration::from_millis(backoff_ms)).await;
        }

        match fetch_once(url).await {
            Ok(text) => {
                GITHUB_CIRCUIT.record_success();
                return Ok(text);
            }
            Err(err) => {
                let retryable = is_retryable(&err);
                tracing::warn!(%err, url, attempt, retryable, "markdown fetch attempt failed");
                if !retryable {
                    // Permanent (e.g. 404) — retrying can't help, and this
                    // isn't a GitHub-health signal either.
                    return Err(anyhow::anyhow!(err.to_string()));
                }
                last_err = Some(err);
            }
        }
    }

    // Exhausted retries on transient errors — this *is* a GitHub-health signal.
    GITHUB_CIRCUIT.record_transient_failure();
    Err(anyhow::anyhow!(
        last_err.map(|e| e.to_string()).unwrap_or_else(|| format!(
            "markdown fetch failed after {FETCH_MAX_ATTEMPTS} attempts"
        ))
    ))
}

#[cfg(feature = "ssr")]
pub async fn process(url: &str) -> anyhow::Result<MarkdownResult> {
    let md = fetch_markdown_source(url).await?;
    render(&md)
}

#[cfg(feature = "ssr")]
pub async fn process_localized(url: &str, locale: &str) -> anyhow::Result<MarkdownResult> {
    if locale == "en"
        && let Some(base) = url.strip_suffix(".md")
    {
        let en_url = format!("{base}.en.md");
        if let Ok(result) = process(&en_url).await {
            return Ok(result);
        }
    }
    process(url).await
}

#[cfg(feature = "ssr")]
pub async fn process_localized_cached(
    cache: &dyn modules::cache::CacheService,
    cache_key: &str,
    url: &str,
    locale: &str,
    ttl_secs: u64,
) -> anyhow::Result<MarkdownResult> {
    if let Some(raw) = cache.get_raw(cache_key).await
        && let Ok(cached) = serde_json::from_str::<MarkdownResult>(&raw)
    {
        return Ok(cached);
    }

    let result = process_localized(url, locale).await?;

    if let Ok(raw) = serde_json::to_string(&result) {
        cache.set_raw(cache_key, raw, ttl_secs).await;
    }

    Ok(result)
}

#[cfg(feature = "ssr")]
pub(crate) fn render(md: &str) -> anyhow::Result<MarkdownResult> {
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

    // Pass 1: collect headings + summary headings
    let mut headings: Vec<HeadingItem> = Vec::new();
    let mut sum_headings: Vec<HeadingItem> = Vec::new();
    {
        let mut in_heading: Option<(u8, String)> = None;
        for event in Parser::new_ext(md, Options::all()) {
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

    for event in Parser::new_ext(md, Options::all()) {
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
                            format!("<pre>{}</pre>", escaped).to_string()
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
