use crate::command_line::Arguments;
use crate::command_line::MigrationFolder;
use crate::command_line::SubCommand;
use crate::model::QueryRoot;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::observability::tracing::setup_tracer;
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::middleware;
use axum::{extract::Extension, routing::get, Router, Server};
use clap::Parser;
use command_line::SqlCase;
use dotenv::dotenv;
use std::future::ready;
use tokio::signal;
use tracing::info;

mod command_line;
mod db;
mod model;
mod observability;
mod routes;

async fn shutdown_signal() {
    // (1)
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    opentelemetry::global::shutdown_tracer_provider();
}

#[tokio::main]
async fn main() {
    let _ = dotenv().ok();
    let _ = setup_tracer();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::StartServer { port } => {
            let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
            let prometheus_recorder = create_prometheus_recorder();

            let address = format!("0.0.0.0:{}", port);
            info!("Service starting at address: {}", address);

            let app = Router::new()
                .route("/", get(graphql_playground).post(graphql_handler))
                .route("/health", get(health))
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics))
                .layer(Extension(schema));

            Server::bind(&address.parse().unwrap())
                .serve(app.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
        }
        SubCommand::Sqlx { case } => match case {
            SqlCase::Test => {
                db::test().await.unwrap();
            }
            SqlCase::Migrate { folder } => match folder {
                MigrationFolder::Bookstore => {
                    db::migrate_bookstore().await.unwrap();
                }
            },
            _ => {
                todo!("not implemented")
            }
        },
        _ => todo!("not implemented"),
    }
}
