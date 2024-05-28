use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new();

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
