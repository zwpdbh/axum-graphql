use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub mod graphql;

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub(crate) async fn health() -> impl IntoResponse {
    let health = Health { healthy: true };

    (StatusCode::OK, Json(health))
}
