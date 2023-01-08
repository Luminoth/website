use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::static_files;
use crate::options::SharedOptions;

use super::{with_is_local, with_region};

pub fn init_routes(
    region: impl Into<String>,
    options: SharedOptions,
) -> BoxedFilter<(impl Reply,)> {
    let region = region.into();

    get_static_files(region, options).boxed()
}

fn get_static_files(
    region: impl Into<String>,
    options: SharedOptions,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("static"))
        .and(with_region(region.into()))
        .and(with_is_local(options))
        .and_then(static_files::get_static_files_handler)
        .boxed()
}
