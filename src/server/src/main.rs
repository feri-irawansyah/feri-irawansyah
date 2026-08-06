#![recursion_limit = "512"]

mod auth;
mod seo;
mod uploads;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::seo::{robots_txt, rss_xml, sitemap_xml};
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use modules::auth::AuthService;
    use modules::cache::CacheService;
    use modules::certifications::CertService;
    use modules::experience::ExperienceService;
    use modules::laboratory::LaboratoryService;
    use modules::notes::NoteService;
    use modules::portfolio::PortfolioService;
    use modules::positions::PositionService;
    use modules::skills::SkillService;
    use modules::users::UserService;
    use repositories::auth::AuthRepositoryImpl;
    use repositories::certifications::CertRepositoryImpl;
    use repositories::database::create_pool;
    use repositories::experience::ExperienceRepositoryImpl;
    use repositories::laboratory::LaboratoryRepositoryImpl;
    use repositories::notes::NoteRepositoryImpl;
    use repositories::portfolio::PortfolioRepositoryImpl;
    use repositories::positions::PositionRepositoryImpl;
    use repositories::skills::SkillRepositoryImpl;
    use repositories::users::UserRepositoryImpl;
    use services::auth::{AuthServiceDeps, AuthServiceImpl};
    use services::cache::{CacheServiceDeps, CacheServiceImpl};
    use services::certifications::{CertServiceDeps, CertServiceImpl};
    use services::experience::{ExperienceServiceDeps, ExperienceServiceImpl};
    use services::laboratory::{LaboratoryServiceDeps, LaboratoryServiceImpl};
    use services::notes::{NoteServiceDeps, NoteServiceImpl};
    use services::portfolio::{PortfolioServiceDeps, PortfolioServiceImpl};
    use services::positions::{PositionServiceDeps, PositionServiceImpl};
    use services::skills::{SkillServiceDeps, SkillServiceImpl};
    use services::users::{UserServiceDeps, UserServiceImpl};
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

    let cache = connectors::cache::create_cache_client()
        .await
        .expect("valkey connection failed");
    tracing::info!("Valkey cache connected");

    let auth_svc: Arc<dyn AuthService> = Arc::new(AuthServiceImpl::new(AuthServiceDeps {
        auth_repo: Arc::new(AuthRepositoryImpl::new(pool.clone())),
        jwt_secret,
    }));
    let user_svc: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(UserServiceDeps {
        user_repo: Arc::new(UserRepositoryImpl::new(pool.clone())),
    }));
    let note_svc: Arc<dyn NoteService> = Arc::new(NoteServiceImpl::new(NoteServiceDeps {
        note_repo: Arc::new(NoteRepositoryImpl::new(pool.clone())),
        cache: cache.clone(),
    }));
    let laboratory_svc: Arc<dyn LaboratoryService> =
        Arc::new(LaboratoryServiceImpl::new(LaboratoryServiceDeps {
            laboratory_repo: Arc::new(LaboratoryRepositoryImpl::new(pool.clone())),
        }));
    let skill_svc: Arc<dyn SkillService> = Arc::new(SkillServiceImpl::new(SkillServiceDeps {
        skill_repo: Arc::new(SkillRepositoryImpl::new(pool.clone())),
        cache: cache.clone(),
    }));
    let portfolio_svc: Arc<dyn PortfolioService> =
        Arc::new(PortfolioServiceImpl::new(PortfolioServiceDeps {
            portfolio_repo: Arc::new(PortfolioRepositoryImpl::new(pool.clone())),
        }));
    let experience_repo = Arc::new(ExperienceRepositoryImpl::new(pool.clone()));
    let experience_svc: Arc<dyn ExperienceService> =
        Arc::new(ExperienceServiceImpl::new(ExperienceServiceDeps {
            experience_repo: experience_repo.clone(),
        }));
    let position_svc: Arc<dyn PositionService> =
        Arc::new(PositionServiceImpl::new(PositionServiceDeps {
            position_repo: Arc::new(PositionRepositoryImpl::new(pool.clone())),
            experience_repo,
        }));
    let cert_svc: Arc<dyn CertService> = Arc::new(CertServiceImpl::new(CertServiceDeps {
        cert_repo: Arc::new(CertRepositoryImpl::new(pool.clone())),
    }));
    let cache_svc: Arc<dyn CacheService> =
        Arc::new(CacheServiceImpl::new(CacheServiceDeps { conn: cache }));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    tracing::info!("Listening on http://{addr}");

    HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        let site_root = leptos_options.site_root.clone();
        let pool = pool.clone();

        App::new()
            .wrap(auth::AuthMiddleware)
            // Transparently gzip/brotli/zstd-compresses every response (SSR
            // HTML, server-fn JSON, and the static pkg/*.wasm|js|css assets)
            // based on the client's Accept-Encoding — no precompressed files
            // or reverse proxy needed.
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(leptos_options.clone()))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(auth_svc.clone()))
            .app_data(web::Data::new(user_svc.clone()))
            .app_data(web::Data::new(note_svc.clone()))
            .app_data(web::Data::new(laboratory_svc.clone()))
            .app_data(web::Data::new(skill_svc.clone()))
            .app_data(web::Data::new(portfolio_svc.clone()))
            .app_data(web::Data::new(position_svc.clone()))
            .app_data(web::Data::new(experience_svc.clone()))
            .app_data(web::Data::new(cert_svc.clone()))
            .app_data(web::Data::new(cache_svc.clone()))
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
