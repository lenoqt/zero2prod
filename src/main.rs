use zero2prod::configuration::get_configuration;
use zero2prod::startup::build;
use zero2prod::telemetry::{get_subscriber, setup_logger};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    setup_logger(subscriber);

    // Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let server = build(configuration).await?;
    server.await?;
    Ok(())
}
