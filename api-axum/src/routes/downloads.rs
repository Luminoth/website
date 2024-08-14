use axum::{routing::get, Router};

use crate::handlers::downloads::*;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/v1/downloads", get(get_downloads_handler))
        .route(
            "/v1/downloads/categories",
            get(get_download_categories_handler),
        )
}
