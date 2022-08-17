use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use std::sync::Once;

static INIT: Once = Once::new();

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn setup_logger() {
    INIT.call_once(|| {
        let env = env_logger::Env::new().default_filter_or("info");
        env_logger::init_from_env(env);
    });
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let address = listener.local_addr().unwrap().to_string();
    log::info!("Starting HTTP at {address}");

    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/health_check",
                web::get().to(health_check).wrap(Logger::default()),
            )
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
