#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use modules::certifications::CertService;
    use modules::notes::NoteService;
    use modules::portfolio::PortfolioService;
    use modules::positions::PositionService;
    use modules::skills::SkillService;
    use repositories::certifications::CertRepositoryImpl;
    use repositories::database::create_pool;
    use repositories::notes::NoteRepositoryImpl;
    use repositories::portfolio::PortfolioRepositoryImpl;
    use repositories::positions::PositionRepositoryImpl;
    use repositories::skills::SkillRepositoryImpl;
    use services::certifications::{CertServiceDeps, CertServiceImpl};
    use services::notes::{NoteServiceDeps, NoteServiceImpl};
    use services::portfolio::{PortfolioServiceDeps, PortfolioServiceImpl};
    use services::positions::{PositionServiceDeps, PositionServiceImpl};
    use services::skills::{SkillServiceDeps, SkillServiceImpl};
    use std::sync::Arc;
    use views::app::{shell, App};

    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool = create_pool().await.expect("database connection failed");
    schemas::run(&pool).await.expect("migration failed");
    tracing::info!("Database connected and migrations applied");

    let note_svc: Arc<dyn NoteService> = Arc::new(NoteServiceImpl::new(NoteServiceDeps {
        note_repo: Arc::new(NoteRepositoryImpl::new(pool.clone())),
    }));
    let skill_svc: Arc<dyn SkillService> = Arc::new(SkillServiceImpl::new(SkillServiceDeps {
        skill_repo: Arc::new(SkillRepositoryImpl::new(pool.clone())),
    }));
    let portfolio_svc: Arc<dyn PortfolioService> =
        Arc::new(PortfolioServiceImpl::new(PortfolioServiceDeps {
            portfolio_repo: Arc::new(PortfolioRepositoryImpl::new(pool.clone())),
        }));
    let position_svc: Arc<dyn PositionService> =
        Arc::new(PositionServiceImpl::new(PositionServiceDeps {
            position_repo: Arc::new(PositionRepositoryImpl::new(pool.clone())),
        }));
    let cert_svc: Arc<dyn CertService> = Arc::new(CertServiceImpl::new(CertServiceDeps {
        cert_repo: Arc::new(CertRepositoryImpl::new(pool.clone())),
    }));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    tracing::info!("Listening on http://{addr}");

    HttpServer::new(move || {
        let leptos_options = conf.leptos_options.clone();
        let site_root = leptos_options.site_root.clone();
        let pool = pool.clone();

        App::new()
            .app_data(web::Data::new(leptos_options.clone()))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(note_svc.clone()))
            .app_data(web::Data::new(skill_svc.clone()))
            .app_data(web::Data::new(portfolio_svc.clone()))
            .app_data(web::Data::new(position_svc.clone()))
            .app_data(web::Data::new(cert_svc.clone()))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root.to_string()))
            .service(Files::new("/public", "./public"))
            .service(Files::new("/uploads", "./uploads"))
            .leptos_routes(routes.clone(), {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
