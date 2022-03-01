use sqlx::PgPool;
use std::net::TcpListener;
use tracing::info;
use tracing_subscriber;
use zero2prod_axum::configuration::get_configuration;
use zero2prod_axum::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let configuration = get_configuration().expect("Failed to read configuration file");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("{}", address);
    tracing::info!("Server listening on {}", address);
    let listener = TcpListener::bind(address)?;

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let _ = run(listener, connection_pool).unwrap();
    loop {}
    Ok(())
}
