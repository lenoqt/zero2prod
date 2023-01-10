use sqlx::postgres::PgPoolOptions;

use std::net::TcpListener;
use std::time::Duration;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, setup_logger};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    setup_logger(subscriber);

    // Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::new(5, 0))
        .connect_lazy_with(configuration.database.with_db());

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
