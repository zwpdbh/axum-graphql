use crate::command_line::Arguments;
use crate::command_line::SubCommand;
use crate::model::QueryRoot;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::middleware;
use axum::{extract::Extension, routing::get, Router, Server};
use clap::Parser;
use std::future::ready;

mod command_line;
mod model;
mod observability;
mod routes;

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::StartServer { port } => {
            let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
            let prometheus_recorder = create_prometheus_recorder();

            let app = Router::new()
                .route("/", get(graphql_playground).post(graphql_handler))
                .route("/health", get(health))
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics))
                .layer(Extension(schema));

            Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        _ => {
            unreachable!("not implemented")
        }
    }
}
