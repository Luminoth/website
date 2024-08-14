use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::downloads;

use super::with_app_state;
use crate::state::AppState;

pub fn init_routes(app_state: AppState) -> BoxedFilter<(impl Reply,)> {
    get_download_categories(app_state.clone())
        .or(get_downloads(app_state))
        .boxed()
}

fn get_download_categories(app_state: AppState) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "downloads" / "categories"))
        .and(with_app_state(app_state))
        .and_then(downloads::get_download_categories_handler)
        .boxed()
}

fn get_downloads(app_state: AppState) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "downloads"))
        .and(with_app_state(app_state))
        .and_then(downloads::get_downloads_handler)
        .boxed()
}
