use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::pictures;

pub fn init_routes() -> BoxedFilter<(impl Reply,)> {
    get_pictures().or(get_picture()).boxed()
}

fn get_pictures() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "pictures"))
        .and_then(pictures::get_pictures_handler)
        .boxed()
}

fn get_picture() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "pictures" / String))
        .and_then(pictures::get_picture_handler)
        .boxed()
}
