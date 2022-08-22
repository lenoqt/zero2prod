use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup_logger() {
    INIT.call_once(|| {
        let env = env_logger::Env::new().default_filter_or("info");
        env_logger::init_from_env(env);
    });
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    log::info!("Starting HTTP at {address}");

    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route(
                "/health_check",
                web::get().to(health_check).wrap(Logger::default()),
            )
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
