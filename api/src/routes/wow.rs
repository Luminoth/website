use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::wow;
use crate::options::SharedOptions;

use super::with_share_dir;

pub fn init_routes(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    get_addons(options.clone())
        .or(get_macros(options.clone()))
        .or(get_screenshots(options.clone()))
        .or(get_screenshots_type(options))
        .boxed()
}

fn get_addons(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "addons"))
        .and(with_share_dir(options))
        .and_then(wow::get_addons_handler)
        .boxed()
}

fn get_macros(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "macros"))
        .and(with_share_dir(options))
        .and_then(wow::get_macros_handler)
        .boxed()
}

fn get_screenshots(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "screenshots"))
        .and(with_share_dir(options))
        .and_then(|share_dir| wow::get_screenshots_handler("screenshots", share_dir))
        .boxed()
}

fn get_screenshots_type(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "wow" / "screenshots" / String))
        .and(with_share_dir(options))
        .and_then(wow::get_screenshots_handler)
        .boxed()
}
