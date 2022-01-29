mod downloads;
mod news;
mod pictures;
mod wow;

use std::path::PathBuf;

use tracing::info;
use warp::filters::BoxedFilter;
use warp::http::Method;
use warp::{Filter, Reply};

use crate::options::SharedOptions;

pub fn init_cors(local: bool) -> warp::cors::Builder {
    info!("Initializing CORS...");

    let mut builder = warp::cors()
        .allow_methods(&[
            Method::HEAD,
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // TODO: make this configurable
        .allow_origin("https://www.energonsoftware.org/")
        .allow_credentials(true);

    if local {
        info!("Allowing localhost...");
        builder = builder.allow_origin("http://localhost:4200");
    }

    builder
}

fn get_root() -> BoxedFilter<(impl Reply,)> {
    warp::get().and(warp::path::end()).map(warp::reply).boxed()
}

pub fn init_routes(
    region: impl Into<String>,
    options: SharedOptions,
) -> BoxedFilter<(impl Reply,)> {
    info!("Initializing routes...");

    let region = region.into();

    get_root()
        .or(downloads::init_routes(region.clone()))
        .or(news::init_routes(region))
        .or(pictures::init_routes(options.clone()))
        .or(wow::init_routes(options))
        .boxed()
}

fn with_region(
    region: impl Into<String>,
) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    let region = region.into();
    warp::any().map(move || region.clone())
}

fn with_share_dir(
    options: SharedOptions,
) -> impl Filter<Extract = (PathBuf,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || options.read().share_dir())
}
