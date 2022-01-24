use axum::{extract::Form, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: Form<FormData>) -> impl IntoResponse {
    println!("{}, {}", form.name, form.email);
    StatusCode::OK
}
