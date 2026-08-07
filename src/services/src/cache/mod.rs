use connectors::cache::CacheStore;
use std::sync::Arc;

pub mod cache_service;
pub use cache_service::CacheServiceImpl;

pub struct CacheServiceDeps {
    pub conn: Arc<dyn CacheStore>,
}
