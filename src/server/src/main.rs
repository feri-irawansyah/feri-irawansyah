#![recursion_limit = "512"]

mod app;
mod auth;
mod seo;
mod uploads;

use app::AppServices;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::seo::{robots_txt, rss_xml, sitemap_xml};
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use repositories::database::create_pool;
    use std::sync::Arc;
    use views::app::{App, shell};

    dotenvy::dotenv().ok();

    // ── Logging setup ────────────────────────────────────────────────────
    // Dual output: human-readable stdout + daily-rotated file in LOG_DIR.
    // Set LOG_FORMAT=json in production to enable structured JSON logs
    // (useful when shipping to Loki or a log management page).
    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "text".to_string());

    std::fs::create_dir_all(&log_dir).expect("failed to create log directory");

    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("app")
        .filename_suffix("log")
        .build(&log_dir)
        .expect("failed to create log file appender");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    if log_format == "json" {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(non_blocking_file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking_file)
                    .with_ansi(false),
            )
            .init();
    }
    // _guard must live until process exit to flush buffered file writes

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = create_pool().await.expect("database connection failed");
    schemas::run(&pool).await.expect("migration failed");
    tracing::info!("Database connected and migrations applied");

    // Never panics — a cache is an optimization, not a hard boot dependency.
    // Logs `error!` and runs degraded (every read falls through to the DB)
    // if Valkey isn't reachable; see `connect_or_degraded`.
    let cache = connectors::cache::connect_or_degraded().await;

    let storage: Arc<dyn connectors::supabase::StorageStore> = Arc::new(
        connectors::supabase::SupabaseClient::from_env().expect("supabase config invalid"),
    );

    let services = AppServices::build(pool.clone(), cache, storage, jwt_secret);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    tracing::info!("Listening on http://{addr}");

    HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        let site_root = leptos_options.site_root.clone();
        let pool = pool.clone();
        let services = services.clone();

        App::new()
            .wrap(auth::AuthMiddleware)
            // Transparently gzip/brotli/zstd-compresses every response (SSR
            // HTML, server-fn JSON, and the static pkg/*.wasm|js|css assets)
            // based on the client's Accept-Encoding — no precompressed files
            // or reverse proxy needed.
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(leptos_options.clone()))
            .app_data(web::Data::new(pool))
            .configure(move |cfg| services.configure(cfg))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root.to_string()))
            .service(Files::new("/public", "./public"))
            .service(Files::new("/uploads", "./uploads"))
            .route("/health", web::get().to(health))
            .route("/robots.txt", web::get().to(robots_txt))
            .route("/sitemap.xml", web::get().to(sitemap_xml))
            .route("/rss.xml", web::get().to(rss_xml))
            .configure(uploads::configure)
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
    })
    .bind(&addr)?
    .run()
    .await
}

async fn health(
    pool: actix_web::web::Data<repositories::database::PgPool>,
) -> actix_web::HttpResponse {
    match repositories::database::health_check(pool.get_ref()).await {
        Ok(_) => actix_web::HttpResponse::Ok().json(serde_json::json!({ "status": "ok" })),
        Err(e) => actix_web::HttpResponse::ServiceUnavailable()
            .json(serde_json::json!({ "status": "error", "detail": e.to_string() })),
    }
}
