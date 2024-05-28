use axum::routing::get;
use axum::{Router, Server};
use routes::health;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
