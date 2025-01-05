use axum::{routing::get, Router};

use crate::handlers::news::*;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/v1/news", get(get_news_handler))
        .route("/v1/news/authors", get(get_news_authors_handler))
}
