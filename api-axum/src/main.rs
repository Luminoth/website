//#![deny(warnings)]

mod http_tracing;
mod options;
mod state;
mod util;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    debug_handler,
    http::{HeaderValue, Method, StatusCode, Uri},
    middleware,
    response::IntoResponse,
    Router,
};
use clap::Parser;
use parking_lot::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

use http_tracing::tracing_wrapper;
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
        console_subscriber::init();
    } else {
        init_logging()?
    };

    // TODO: make region configurable?
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;
    let app_state = AppState::new(aws_config);

    let addr = options
        .address()
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| panic!("Invalid address: {}", options.address()));

    let options = Arc::new(RwLock::new(options));

    let app = Router::new()
        // TODO: routes
        .fallback(handler_404)
        .layer(init_cors_layer(!options.read().prod)?)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(tracing_wrapper))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(app_state);

    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

#[debug_handler]
async fn handler_404(uri: Uri) -> impl IntoResponse {
    debug!("invalid resource: {}", uri);
    (StatusCode::NOT_FOUND, "Resource not found")
}
