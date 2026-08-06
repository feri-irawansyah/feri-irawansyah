/// HTTP client for Supabase Storage — talks to the REST API, not Postgres,
/// so it lives here rather than in `repositories`.
#[derive(Clone)]
pub struct SupabaseClient {
    pub(crate) base_url: String,
    pub(crate) service_key: String,
    pub(crate) bucket: String,
    pub(crate) http: reqwest::Client,
}

impl SupabaseClient {
    pub fn new(base_url: String, service_key: String, bucket: String) -> Self {
        Self {
            base_url,
            service_key,
            bucket,
            http: reqwest::Client::new(),
        }
    }

    /// Reads `SUPABASE_URL` / `SUPABASE_SERVICE_KEY` from the environment.
    /// Bucket defaults to `feri-irawansyah.my.id`, overridable via
    /// `SUPABASE_BUCKET`.
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let base_url = std::env::var("SUPABASE_URL")
            .map_err(|_| anyhow::anyhow!("SUPABASE_URL belum di-set di .env"))?;
        let service_key = std::env::var("SUPABASE_SERVICE_KEY")
            .map_err(|_| anyhow::anyhow!("SUPABASE_SERVICE_KEY belum di-set di .env"))?;
        let bucket = std::env::var("SUPABASE_BUCKET")
            .unwrap_or_else(|_| "feri-irawansyah.my.id".to_string());

        Ok(Self::new(base_url, service_key, bucket))
    }
}
