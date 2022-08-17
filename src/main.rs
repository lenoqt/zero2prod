use std::net::TcpListener;
use zero2prod::{run, setup_logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
