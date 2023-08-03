use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::static_files;
use crate::options::SharedOptions;

use super::{with_app_state, with_is_local};
use crate::state::AppState;

pub fn init_routes(app_state: AppState, options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    get_static_files(app_state, options).boxed()
}

fn get_static_files(app_state: AppState, options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("static"))
        .and(with_app_state(app_state.clone()))
        .and(with_is_local(options))
        .and_then(static_files::get_static_files_handler)
        .boxed()
}
