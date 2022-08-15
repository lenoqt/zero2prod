use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let address = listener.local_addr().unwrap().to_string();
    log::info!("Starting HTTP at {address}");

    let server = HttpServer::new(|| {
        App::new().route(
            "/health_check",
            web::get().to(health_check).wrap(Logger::default()),
        )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
