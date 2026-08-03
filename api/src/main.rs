#![deny(warnings)]

mod constants;
mod error;
mod handlers;
mod http_tracing;
mod models;
mod options;
mod otel;
mod routes;
mod state;
mod util;

use std::net::SocketAddr;

use axum::{
    Router,
    http::{HeaderValue, Method},
    middleware,
};
use clap::Parser;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit,
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, TraceLayer},
};
use tracing::{Level, info, warn};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use options::Options;
use otel::{OtelProviders, SERVICE_NAME};
use state::AppState;

fn init_logging() -> anyhow::Result<Option<OtelProviders>> {
    let providers = otel::init_otel_providers()?;

    let otel_layer = providers.as_ref().map(|providers| {
        tracing_opentelemetry::layer().with_tracer(providers.tracer_provider.tracer(SERVICE_NAME))
    });
    let log_layer = providers
        .as_ref()
        .map(|providers| OpenTelemetryTracingBridge::new(&providers.logger_provider));

    // The AWS SDK crates log credential-chain resolution (including the access key ID)
    // at INFO by default, which is noisier than we want in normal operation.
    let target_filter = Targets::new()
        .with_default(Level::INFO)
        .with_target("aws", Level::WARN);

    tracing_subscriber::registry()
        .with(target_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .with(log_layer)
        .try_init()?;

    if providers.is_some() {
        info!(
            endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap_or_default(),
            "OTel exporters configured"
        );
        if std::env::var("NEW_RELIC_LICENSE_KEY").is_err() {
            warn!("NEW_RELIC_LICENSE_KEY not set, exporting OTLP without an api-key header");
        }
    } else {
        info!("OTEL_EXPORTER_OTLP_ENDPOINT not set, skipping OTel exporter setup");
    }

    Ok(providers)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

pub fn init_cors_layer(options: &Options) -> anyhow::Result<CorsLayer> {
    info!("Initializing CORS layer...");

    let mut layer = CorsLayer::new()
        .allow_methods([Method::OPTIONS, Method::HEAD, Method::GET])
        .allow_origin(options.cors_origin.parse::<HeaderValue>()?)
        .allow_credentials(true);

    if !options.prod {
        warn!("Allowing localhost...");
        layer = layer.allow_origin("http://localhost:4200".parse::<HeaderValue>()?);
    }

    Ok(layer)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let options = Options::parse();

    // TODO: make this not mutually exclusive
    // we should probably use `tracing_subscriber::registry().with(console_layer).with(fmt_layer).init()` ?
    let otel_providers = if options.tracing {
        println!("Enabling tracing ...");
        console_subscriber::init();
        None
    } else {
        init_logging()?
    };

    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new(options.aws_region.clone()))
        .load()
        .await;

    let app_state = AppState::new(options, aws_config);

    let addr = app_state
        .options
        .address()
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| panic!("Invalid address: {}", app_state.options.address()));

    let app = routes::init_routes(Router::new())
        .layer(init_cors_layer(&app_state.options)?)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(http_tracing::tracing_wrapper))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        //.on_request(http_tracing::on_request)
                        //.on_response(http_tracing::on_response),
                        .on_failure(DefaultOnFailure::new().latency_unit(LatencyUnit::Micros)),
                )
                .into_inner(),
        )
        .with_state(app_state.clone());

    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    if let Some(providers) = otel_providers {
        providers.shutdown()?;
    }

    Ok(())
}
