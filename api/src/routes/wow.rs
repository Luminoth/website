use axum::{routing::get, Router};

use crate::handlers::wow::*;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/v1/wow/addons", get(get_addons_handler))
        .route("/v1/wow/macros", get(get_macros_handler))
        .route("/v1/wow/screenshots", get(get_screenshots_handler))
        .route("/v1/wow/screenshots/{path}", get(get_screenshot_handler))
}
