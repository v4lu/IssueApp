use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api::{
    api::{
        middlewares::{logger::RequestLogger, token_extractor::TokenExtractor},
        routes,
    },
    app_state::AppState,
    config,
    utils::logger::setup_logger,
};

use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
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
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(state.clone())
            .wrap(RequestLogger)
            .wrap(TokenExtractor)
            .configure(routes::configure_routes)
    })
    .bind(format!("0.0.0.0:{}", cfg.port))?
    .run()
    .await
}
