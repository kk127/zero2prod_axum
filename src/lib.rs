use axum::routing::IntoMakeService;
use axum::Server;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use log::{debug, info};

// use serde::{Deserialize, Serialize};
use env_logger;
use std::env;
use std::net::{SocketAddr, TcpListener};

async fn health_check() -> impl IntoResponse {
    info!("health check");
    StatusCode::OK
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    info!("Greetings from {}", name);
    format!("Hello {}", name)
}

pub fn run(listener: TcpListener) -> Result<String, std::io::Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    let socket = listener.local_addr().unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    println!("socket: {socket}");
    let server = axum::Server::bind(&socket).serve(app.into_make_service());
    let _ = tokio::spawn(server);

    Ok(format!("127.0.0.1:{port}"))
}
