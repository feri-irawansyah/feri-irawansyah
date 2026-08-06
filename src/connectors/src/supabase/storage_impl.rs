use anyhow::{Result, anyhow};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;

use super::client::SupabaseClient;
use super::storage::StorageStore;

#[async_trait]
impl StorageStore for SupabaseClient {
    async fn upload(&self, path: &str, bytes: Vec<u8>, content_type: &str) -> Result<String> {
        let bucket = &self.bucket;
        let base_url = &self.base_url;
        let upload_url = format!("{base_url}/storage/v1/object/{bucket}/{path}");

        let resp = self
            .http
            .post(&upload_url)
            .header("Authorization", format!("Bearer {}", self.service_key))
            .header("apikey", &self.service_key)
            .header("Content-Type", content_type)
            .header("x-upsert", "true")
            .body(bytes)
            .send()
            .await
            .map_err(|e| anyhow!("Gagal menghubungi Supabase: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Supabase menolak upload ({status}): {body}"));
        }

        Ok(format!(
            "{base_url}/storage/v1/object/public/{bucket}/{path}"
        ))
    }
}

/// In-memory [`StorageStore`] for unit tests — no network, no real Supabase
/// bucket required. Returns a fake `mock://<path>` URL.
#[derive(Default)]
pub struct MockStorageClient {
    files: Mutex<HashMap<String, Vec<u8>>>,
}

impl MockStorageClient {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl StorageStore for MockStorageClient {
    async fn upload(&self, path: &str, bytes: Vec<u8>, _content_type: &str) -> Result<String> {
        self.files.lock().unwrap().insert(path.to_string(), bytes);
        Ok(format!("mock://{path}"))
    }
}
