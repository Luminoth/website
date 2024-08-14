mod downloads;
mod news;
mod pictures;
mod static_files;
mod wow;

use std::path::PathBuf;

use tracing::{info, warn};
use warp::filters::BoxedFilter;
use warp::http::Method;
use warp::{Filter, Reply};

use crate::options::SharedOptions;
use crate::state::AppState;

pub fn init_cors(local: bool) -> warp::cors::Builder {
    info!("Initializing CORS...");

    let mut builder = warp::cors()
        .allow_methods(&[
            Method::HEAD,
            Method::GET,
            //Method::PUT,
            //Method::POST,
            //Method::DELETE,
            Method::OPTIONS,
        ])
        // TODO: make this configurable
        .allow_origin("https://www.energonsoftware.org")
        .allow_credentials(true);

    if local {
        warn!("Allowing localhost...");
        builder = builder.allow_origin("http://localhost:4200");
    }

    builder
}

fn get_root() -> BoxedFilter<(impl Reply,)> {
    warp::get().and(warp::path::end()).map(warp::reply).boxed()
}

pub fn init_routes(app_state: AppState, options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    info!("Initializing routes...");

    get_root()
        .or(downloads::init_routes(app_state.clone()))
        .or(news::init_routes(app_state.clone()))
        .or(pictures::init_routes(options.clone()))
        .or(wow::init_routes(options.clone()))
        .or(static_files::init_routes(app_state.clone(), options))
        .boxed()
}

fn with_app_state(
    app_state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || app_state.clone())
}

fn with_is_local(
    options: SharedOptions,
) -> impl Filter<Extract = (bool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || !options.prod)
}

fn with_share_dir(
    options: SharedOptions,
) -> impl Filter<Extract = (PathBuf,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || options.share_dir())
}
