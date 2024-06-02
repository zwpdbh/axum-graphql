use opentelemetry::sdk::trace::Tracer;
use std::env;
use tracing_subscriber::Layer;

struct JaegerConfig {
    jaeger_agent_host: String,
    jaeger_agent_port: String,
    jaeger_tracing_service_name: String,
}

pub fn create_tracer_from_env() -> Option<Tracer> {
    let jaeger_enabled: bool = env::var("JAEGER_ENABLED")
        .unwrap_or_else(|_| "false".into())
        .parse()
        .unwrap();

    if jaeger_enabled {
        let config = get_jaeger_config_from_env();
        Some(init_tracer_with_jaeger(config))
    } else {
        None
    }
}

fn init_tracer_with_jaeger(config: JaegerConfig) -> Tracer {
    use opentelemetry::sdk::trace::{self, Sampler};
    use opentelemetry::{global, runtime::Tokio, sdk::propagation::TraceContextPropagator};
    // ensures that tracing is propagated by the traceparent header
    global::set_text_map_propagator(TraceContextPropagator::new());
    opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(format!(
            "{}:{}",
            config.jaeger_agent_host, config.jaeger_agent_port
        ))
        .with_auto_split_batch(true)
        .with_service_name(config.jaeger_tracing_service_name)
        .with_trace_config(trace::config().with_sampler(Sampler::AlwaysOn))
        .install_batch(Tokio)
        .expect("pipeline install error")
}

fn get_jaeger_config_from_env() -> JaegerConfig {
    JaegerConfig {
        // The JAEGER_AGENT_HOST and JAEGER_AGENT_PORT need to match the setting in docker-compose file.
        jaeger_agent_host: env::var("JAEGER_AGENT_HOST").unwrap_or_else(|_| "localhost".into()),
        jaeger_agent_port: env::var("JAEGER_AGENT_PORT").unwrap_or_else(|_| "6831".into()),
        jaeger_tracing_service_name: env::var("TRACING_SERVICE_NAME")
            .unwrap_or_else(|_| "axum-graphql".into()),
    }
}

pub fn setup_tracer() {
    use tracing_subscriber::filter::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::Registry;

    // set trace level
    let filter = EnvFilter::new("debug");

    let registry = Registry::default().with(
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_filter(filter),
    );

    match create_tracer_from_env() {
        Some(tracer) => registry
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()
            .expect("Failed to register tracer with registry"),
        None => registry
            .try_init()
            .expect("Failed to register tracer with registry"),
    }
}
