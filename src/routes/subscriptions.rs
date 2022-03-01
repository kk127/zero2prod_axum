use axum::{
    extract::{Extension, Form},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: Form<FormData>,
    Extension(db_pool): Extension<PgPool>,
) -> impl IntoResponse {
    info!(
        "Adding '{}' '{}' as a new subscriber",
        form.email, form.name
    );
    info!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            info!("New subscribers details have been saved");
            StatusCode::OK
        }
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
