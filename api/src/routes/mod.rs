mod downloads;
mod news;
mod pictures;
mod wow;

use log::info;
use warp::filters::BoxedFilter;
use warp::http::Method;
use warp::{Filter, Reply};

pub fn init_cors() -> warp::cors::Builder {
    warp::cors()
        .allow_methods(&[
            Method::HEAD,
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_any_origin()
        .allow_credentials(true)
}

fn get_root() -> BoxedFilter<(impl Reply,)> {
    warp::get().and(warp::path::end()).map(warp::reply).boxed()
}

pub fn init_routes() -> BoxedFilter<(impl Reply,)> {
    info!("Initializing routes...");

    get_root()
        .or(downloads::init_routes())
        .or(news::init_routes())
        .or(pictures::init_routes())
        .or(wow::init_routes())
        .boxed()
}
