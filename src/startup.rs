use std::net::TcpListener;
use std::sync::Arc;

use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<String, std::io::Error> {
    // let db_pool = Arc::new(db_pool);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .layer(AddExtensionLayer::new(db_pool));

    let socket = listener.local_addr().unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let server = axum::Server::bind(&socket).serve(app.into_make_service());
    let _ = tokio::spawn(server);

    Ok(format!("127.0.0.1:{port}"))
}
