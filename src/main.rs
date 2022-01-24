use std::net::TcpListener;
use zero2prod_axum::configuration::get_configuration;
use zero2prod_axum::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration file");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let _ = run(listener).unwrap();
    Ok(())
}
