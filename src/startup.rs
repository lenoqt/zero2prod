use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Once;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

static INIT: Once = Once::new();

pub fn setup_logger() {
    INIT.call_once(|| {
        // let env = env_logger::Env::new().default_filter_or("info");
        // env_logger::init_from_env(env);
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let formatting_layer = BunyanFormattingLayer::new(
            "zero2prod".into(),
            // Output the formatted spans to stout.
            std::io::stdout,
        );

        // The `with` method is provided by `SubscriberExt`, an extension
        // trait for `Subscriber` exposed by `tracing_subscriber`
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);
        // `set_global_default` can be used by applications to specify
        // what subscriber should be used to process spans.
        set_global_default(subscriber).expect("Failed to set subscriber");
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
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
