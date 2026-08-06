/// Uploads raw bytes to a Supabase Storage bucket via the REST API and
/// returns the public URL. Keeping this outside `repositories` since it talks
/// to an external HTTP API, not Postgres.
const BUCKET: &str = "feri-irawansyah.my.id";

pub async fn upload(path: &str, bytes: Vec<u8>, content_type: &str) -> Result<String, String> {
    let base_url = std::env::var("SUPABASE_URL")
        .map_err(|_| "SUPABASE_URL belum di-set di .env".to_string())?;
    let service_key = std::env::var("SUPABASE_SERVICE_KEY")
        .map_err(|_| "SUPABASE_SERVICE_KEY belum di-set di .env".to_string())?;

    let upload_url = format!("{base_url}/storage/v1/object/{BUCKET}/{path}");

    let client = reqwest::Client::new();
    let resp = client
        .post(&upload_url)
        .header("Authorization", format!("Bearer {service_key}"))
        .header("apikey", &service_key)
        .header("Content-Type", content_type)
        .header("x-upsert", "true")
        .body(bytes)
        .send()
        .await
        .map_err(|e| format!("Gagal menghubungi Supabase: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Supabase menolak upload ({status}): {body}"));
    }

    Ok(format!("{base_url}/storage/v1/object/public/{BUCKET}/{path}"))
}
