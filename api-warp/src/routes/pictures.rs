use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::pictures;
use crate::options::SharedOptions;

use super::with_share_dir;

pub fn init_routes(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    get_pictures_vacation(options).boxed()
}

fn get_pictures_vacation(options: SharedOptions) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "pictures" / "vacation" / String))
        .and(with_share_dir(options))
        .and_then(pictures::get_pictures_vacation_handler)
        .boxed()
}
