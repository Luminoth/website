use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::downloads;

use super::with_region;

pub fn init_routes(region: impl Into<String>) -> BoxedFilter<(impl Reply,)> {
    let region = region.into();

    get_download_categories(region.clone())
        .or(get_downloads(region))
        .boxed()
}

fn get_download_categories(region: impl Into<String>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "downloads" / "categories"))
        .and(with_region(region.into()))
        .and_then(downloads::get_download_categories_handler)
        .boxed()
}

fn get_downloads(region: impl Into<String>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "downloads"))
        .and(with_region(region.into()))
        .and_then(downloads::get_downloads_handler)
        .boxed()
}
