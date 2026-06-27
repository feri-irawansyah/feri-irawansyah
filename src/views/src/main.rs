#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use repositories::database::create_pool;
    use views::app::{shell, App};

    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool = create_pool().await.expect("database connection failed");
    schemas::run(&pool).await.expect("migration failed");
    tracing::info!("Database connected and migrations applied");

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
