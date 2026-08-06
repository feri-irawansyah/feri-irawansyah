use connectors::cache::CacheConn;

pub mod cache_service;
pub use cache_service::CacheServiceImpl;

pub struct CacheServiceDeps {
    pub conn: CacheConn,
}
