use axum::{routing::get, Router};

use crate::handlers::static_files::*;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/static", get(get_static_files_handler))
        .route("/static/:path", get(get_static_file_handler))
}
