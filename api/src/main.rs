#![deny(warnings)]
//#![deny(missing_docs)]

mod error;
mod handlers;
mod http_tracing;
mod models;
mod options;
mod routes;
mod state;
mod util;

use std::net::SocketAddr;

use axum::{
    http::{HeaderValue, Method},
    middleware, Router,
};
use clap::Parser;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, TraceLayer},
    LatencyUnit,
};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

use options::Options;
use state::AppState;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

pub fn init_cors_layer(local: bool) -> anyhow::Result<CorsLayer> {
    info!("Initializing CORS layer...");

    let mut layer = CorsLayer::new()
        .allow_methods([Method::OPTIONS, Method::HEAD, Method::GET])
        // TODO: make this configurable
        .allow_origin("https://www.energonsoftware.org".parse::<HeaderValue>()?)
        .allow_credentials(true);

    if local {
        warn!("Allowing localhost...");
        layer = layer.allow_origin("http://localhost:4200".parse::<HeaderValue>()?);
    }

    Ok(layer)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    // TODO: make this not mutually exclusive
    if options.tracing {
        println!("Enabling tracing ...");
        console_subscriber::init();
    } else {
        init_logging()?
    };

    // TODO: make region configurable?
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;

    let app_state = AppState::new(options, aws_config);

    let addr = app_state
        .options
        .address()
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| panic!("Invalid address: {}", app_state.options.address()));

    let app = routes::init_routes(Router::new())
        .layer(init_cors_layer(!app_state.options.prod)?)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(http_tracing::tracing_wrapper))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                //.level(Level::INFO)
                                .include_headers(true),
                        )
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
    .await?;

    Ok(())
}
