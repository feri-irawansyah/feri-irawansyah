mod google_auth;

/// POSTs a GA4 Data API report body to `endpoint` (`runReport` or
/// `runRealtimeReport`) and returns the raw JSON response. Shared by every
/// `fetch_*` function below so each only has to describe its own
/// dimensions/metrics, not the auth + HTTP plumbing.
async fn run_ga(endpoint: &str, body: serde_json::Value) -> Result<serde_json::Value, String> {
    let property_id = std::env::var("GA_PROPERTY_ID")
        .map_err(|_| "GA_PROPERTY_ID belum di-set di .env".to_string())?;
    let token = google_auth::get_access_token().await?;

    let client = reqwest::Client::new();
    let url =
        format!("https://analyticsdata.googleapis.com/v1beta/properties/{property_id}:{endpoint}");
    let resp = client
        .post(&url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi Google Analytics: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Google Analytics API menolak ({status}): {text}"));
    }

    resp.json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Gagal membaca response Google Analytics: {e}"))
}

fn extract_rows(json: &serde_json::Value) -> Vec<serde_json::Value> {
    json.get("rows").and_then(|r| r.as_array()).cloned().unwrap_or_default()
}

fn dim(row: &serde_json::Value, i: usize) -> Option<String> {
    row.get("dimensionValues")?.as_array()?.get(i)?.get("value")?.as_str().map(String::from)
}

fn metric(row: &serde_json::Value, i: usize) -> Option<String> {
    row.get("metricValues")?.as_array()?.get(i)?.get("value")?.as_str().map(String::from)
}

/// Fetches per-day Active Users + Page Views for the last `days` days from the
/// GA4 Data API. Returns `(date_label, active_users, page_views)` tuples —
/// deliberately plain tuples, not a struct, since this whole module is
/// ssr-only (behind `#[cfg(feature = "ssr")]` on `views::features`) and any
/// type crossing the `#[server]` fn boundary needs to exist in the wasm build
/// too. The admin dashboard page owns the public DTOs instead.
pub async fn fetch_daily_stats(days: i64) -> Result<Vec<(String, i64, i64)>, String> {
    let body = serde_json::json!({
        "dateRanges": [{ "startDate": format!("{days}daysAgo"), "endDate": "today" }],
        "dimensions": [{ "name": "date" }],
        "metrics": [{ "name": "activeUsers" }, { "name": "screenPageViews" }],
        "orderBys": [{ "dimension": { "dimensionName": "date" } }]
    });
    let json = run_ga("runReport", body).await?;

    let stats = extract_rows(&json)
        .into_iter()
        .filter_map(|row| {
            let raw_date = dim(&row, 0)?;
            let active_users = metric(&row, 0)?.parse::<i64>().ok()?;
            let page_views = metric(&row, 1)?.parse::<i64>().ok()?;
            Some((format_ga_date(&raw_date), active_users, page_views))
        })
        .collect();

    Ok(stats)
}

/// Most-viewed pages in the last `days` days. Returns `(path, title, views)`,
/// highest views first, capped at `limit` rows.
pub async fn fetch_top_pages(days: i64, limit: i64) -> Result<Vec<(String, String, i64)>, String> {
    let body = serde_json::json!({
        "dateRanges": [{ "startDate": format!("{days}daysAgo"), "endDate": "today" }],
        "dimensions": [{ "name": "pagePath" }, { "name": "pageTitle" }],
        "metrics": [{ "name": "screenPageViews" }],
        "orderBys": [{ "metric": { "metricName": "screenPageViews" }, "desc": true }],
        "limit": limit,
    });
    let json = run_ga("runReport", body).await?;

    Ok(extract_rows(&json)
        .into_iter()
        .filter_map(|row| {
            let path = dim(&row, 0)?;
            let title = dim(&row, 1)?;
            let views = metric(&row, 0)?.parse::<i64>().ok()?;
            Some((path, title, views))
        })
        .collect())
}

/// Where sessions came from in the last `days` days. Returns
/// `(source, medium, sessions)`, highest sessions first, capped at `limit`.
pub async fn fetch_traffic_sources(
    days: i64,
    limit: i64,
) -> Result<Vec<(String, String, i64)>, String> {
    let body = serde_json::json!({
        "dateRanges": [{ "startDate": format!("{days}daysAgo"), "endDate": "today" }],
        "dimensions": [{ "name": "sessionSource" }, { "name": "sessionMedium" }],
        "metrics": [{ "name": "sessions" }],
        "orderBys": [{ "metric": { "metricName": "sessions" }, "desc": true }],
        "limit": limit,
    });
    let json = run_ga("runReport", body).await?;

    Ok(extract_rows(&json)
        .into_iter()
        .filter_map(|row| {
            let source = dim(&row, 0)?;
            let medium = dim(&row, 1)?;
            let sessions = metric(&row, 0)?.parse::<i64>().ok()?;
            Some((source, medium, sessions))
        })
        .collect())
}

/// Active users by device category (desktop/mobile/tablet) in the last
/// `days` days, highest first.
pub async fn fetch_device_breakdown(days: i64) -> Result<Vec<(String, i64)>, String> {
    let body = serde_json::json!({
        "dateRanges": [{ "startDate": format!("{days}daysAgo"), "endDate": "today" }],
        "dimensions": [{ "name": "deviceCategory" }],
        "metrics": [{ "name": "activeUsers" }],
        "orderBys": [{ "metric": { "metricName": "activeUsers" }, "desc": true }],
    });
    let json = run_ga("runReport", body).await?;

    Ok(extract_rows(&json)
        .into_iter()
        .filter_map(|row| {
            let device = dim(&row, 0)?;
            let users = metric(&row, 0)?.parse::<i64>().ok()?;
            Some((device, users))
        })
        .collect())
}

/// Active users by country in the last `days` days, highest first, capped at
/// `limit`.
pub async fn fetch_geo_breakdown(days: i64, limit: i64) -> Result<Vec<(String, i64)>, String> {
    let body = serde_json::json!({
        "dateRanges": [{ "startDate": format!("{days}daysAgo"), "endDate": "today" }],
        "dimensions": [{ "name": "country" }],
        "metrics": [{ "name": "activeUsers" }],
        "orderBys": [{ "metric": { "metricName": "activeUsers" }, "desc": true }],
        "limit": limit,
    });
    let json = run_ga("runReport", body).await?;

    Ok(extract_rows(&json)
        .into_iter()
        .filter_map(|row| {
            let country = dim(&row, 0)?;
            let users = metric(&row, 0)?.parse::<i64>().ok()?;
            Some((country, users))
        })
        .collect())
}

/// Site-wide engagement totals for the last `days` days: `(bounce_rate,
/// avg_session_duration_secs, engagement_rate, new_users)`. `bounce_rate` and
/// `engagement_rate` are fractions in `[0, 1]`.
pub async fn fetch_engagement_summary(days: i64) -> Result<(f64, f64, f64, i64), String> {
    let body = serde_json::json!({
        "dateRanges": [{ "startDate": format!("{days}daysAgo"), "endDate": "today" }],
        "metrics": [
            { "name": "bounceRate" },
            { "name": "averageSessionDuration" },
            { "name": "engagementRate" },
            { "name": "newUsers" }
        ],
    });
    let json = run_ga("runReport", body).await?;

    let row = extract_rows(&json).into_iter().next();
    let bounce_rate = row.as_ref().and_then(|r| metric(r, 0)).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let avg_session_duration =
        row.as_ref().and_then(|r| metric(r, 1)).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let engagement_rate =
        row.as_ref().and_then(|r| metric(r, 2)).and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let new_users = row.as_ref().and_then(|r| metric(r, 3)).and_then(|v| v.parse().ok()).unwrap_or(0);

    Ok((bounce_rate, avg_session_duration, engagement_rate, new_users))
}

/// Users on the site right now (last ~30 minutes), via the separate GA4
/// Realtime API — a different snapshot than the historical `runReport` data.
pub async fn fetch_realtime_active_users() -> Result<i64, String> {
    let body = serde_json::json!({ "metrics": [{ "name": "activeUsers" }] });
    let json = run_ga("runRealtimeReport", body).await?;

    Ok(extract_rows(&json)
        .into_iter()
        .next()
        .and_then(|row| metric(&row, 0))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0))
}

/// GA4 returns dates as `YYYYMMDD` — turn `20260801` into `1 Agu`.
fn format_ga_date(raw: &str) -> String {
    const MONTHS: [&str; 13] = [
        "", "Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des",
    ];
    if raw.len() != 8 {
        return raw.to_string();
    }
    let month_num: usize = raw[4..6].parse().unwrap_or(0);
    let day = raw[6..8].trim_start_matches('0');
    match MONTHS.get(month_num) {
        Some(&name) if !name.is_empty() => format!("{day} {name}"),
        _ => raw.to_string(),
    }
}
