use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::wow;

pub fn init_routes() -> BoxedFilter<(impl Reply,)> {
    get_addons()
        .or(get_macros())
        .or(get_screenshots())
        .or(get_screenshots_type())
        .boxed()
}

fn get_addons() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "addons"))
        .and_then(wow::get_addons_handler)
        .boxed()
}

fn get_macros() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "macros"))
        .and_then(wow::get_macros_handler)
        .boxed()
}

fn get_screenshots() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "screenshots"))
        .and_then(|| wow::get_screenshots_handler("screenshots"))
        .boxed()
}

fn get_screenshots_type() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "screenshots" / String))
        .and_then(|id| wow::get_screenshots_handler(id))
        .boxed()
}
