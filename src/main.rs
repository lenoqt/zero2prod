use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::{run, setup_logger};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    // Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
