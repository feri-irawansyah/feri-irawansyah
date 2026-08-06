use anyhow::Result;
use async_trait::async_trait;

/// Contract for uploading a file and getting back its public URL. Real
/// callers get this via [`SupabaseClient`](super::client::SupabaseClient)
/// (implemented in [`storage_impl`](super::storage_impl)); tests can
/// substitute [`MockStorageClient`](super::storage_impl::MockStorageClient)
/// instead of hitting the real Supabase API.
#[async_trait]
pub trait StorageStore: Send + Sync {
    async fn upload(&self, path: &str, bytes: Vec<u8>, content_type: &str) -> Result<String>;
}
