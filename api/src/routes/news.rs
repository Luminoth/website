use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::news;

pub fn init_routes() -> BoxedFilter<(impl Reply,)> {
    get_news_authors().or(get_news()).boxed()
}

fn get_news_authors() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "news" / "authors"))
        .and_then(news::get_news_authors_handler)
        .boxed()
}

fn get_news() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "news"))
        .and_then(news::get_news_handler)
        .boxed()
}
