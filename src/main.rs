use crate::command_line::Arguments;
use crate::command_line::SubCommand;
use crate::model::QueryRoot;
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::observability::tracing::create_tracer_from_env;
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::middleware;
use axum::{extract::Extension, routing::get, Router, Server};
use clap::Parser;
use dotenv::dotenv;
use std::future::ready;

use tokio::signal;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

mod command_line;
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
    // dotenv().ok();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::StartServer { port } => {
            let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
            let prometheus_recorder = create_prometheus_recorder();
            let registry = Registry::default().with(tracing_subscriber::fmt::layer().pretty());

            match create_tracer_from_env() {
                Some(tracer) => registry
                    .with(tracing_opentelemetry::layer().with_tracer(tracer))
                    .try_init()
                    .expect("Failed to register tracer with registry"),
                None => registry
                    .try_init()
                    .expect("Failed to register tracer with registry"),
            }
            info!("Service starting");

            let app = Router::new()
                .route("/", get(graphql_playground).post(graphql_handler))
                .route("/health", get(health))
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics))
                .layer(Extension(schema));

            Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
                .serve(app.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap();
        }
        _ => {
            unreachable!("not implemented")
        }
    }
}
