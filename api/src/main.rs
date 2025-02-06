use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::{
    api::{middlewares::token_extractor::TokenExtractor, routes},
    app_state::AppState,
    config,
    utils::logger::setup_custom_logger,
};

use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_custom_logger().unwrap();

    let cfg =
        config::Config::new().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let pool = PgPool::connect(&cfg.database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let state = web::Data::new(
        AppState::new(pool.clone(), &cfg)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?,
    );

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
            .wrap(cors)
            .wrap(TokenExtractor)
            .app_data(web::Data::new(pool.clone()))
            .app_data(state.clone())
            .configure(routes::configure_routes)
    })
    .bind(format!("0.0.0.0:{}", cfg.port))?
    .run()
    .await
}
