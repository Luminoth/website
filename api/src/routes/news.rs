use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::news;

use super::with_region;

pub fn init_routes(region: impl Into<String>) -> BoxedFilter<(impl Reply,)> {
    let region = region.into();

    get_news_authors(region.clone())
        .or(get_news(region))
        .boxed()
}

fn get_news_authors(region: impl Into<String>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "news" / "authors"))
        .and(with_region(region.into()))
        .and_then(news::get_news_authors_handler)
        .boxed()
}

fn get_news(region: impl Into<String>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "news"))
        .and(with_region(region.into()))
        .and_then(news::get_news_handler)
        .boxed()
}
