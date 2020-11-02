use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::pictures;

pub fn init_routes() -> BoxedFilter<(impl Reply,)> {
    get_pictures_vacation().boxed()
}

fn get_pictures_vacation() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "pictures" / "vacation" / String))
        .and_then(pictures::get_pictures_vacation_handler)
        .boxed()
}
