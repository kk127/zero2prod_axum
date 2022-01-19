use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use log::info;
use serde::Deserialize;

use std::net::TcpListener;

async fn health_check() -> impl IntoResponse {
    info!("health check");
    StatusCode::OK
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    info!("Greetings from {}", name);
    format!("Hello {}", name)
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(form: Form<FormData>) -> impl IntoResponse {
    println!("{}, {}", form.name, form.email);
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> Result<String, std::io::Error> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe));

    let socket = listener.local_addr().unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let server = axum::Server::bind(&socket).serve(app.into_make_service());
    let _ = tokio::spawn(server);

    Ok(format!("127.0.0.1:{port}"))
}
