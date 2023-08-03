use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::handlers::news;
use crate::state::AppState;

use super::with_app_state;

pub fn init_routes(app_state: AppState) -> BoxedFilter<(impl Reply,)> {
    get_news_authors(app_state.clone())
        .or(get_news(app_state))
        .boxed()
}

fn get_news_authors(app_state: AppState) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "news" / "authors"))
        .and(with_app_state(app_state))
        .and_then(news::get_news_authors_handler)
        .boxed()
}

fn get_news(app_state: AppState) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path!("v1" / "news"))
        .and(with_app_state(app_state))
        .and_then(news::get_news_handler)
        .boxed()
}
