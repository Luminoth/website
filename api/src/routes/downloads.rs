use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::downloads;

pub fn init_routes() -> BoxedFilter<(impl Reply,)> {
    get_download_categories().or(get_downloads()).boxed()
}

fn get_download_categories() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "downloads" / "categories"))
        .and_then(downloads::get_download_categories_handler)
        .boxed()
}

fn get_downloads() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "downloads"))
        .and_then(downloads::get_downloads_handler)
        .boxed()
}
