use axum::{debug_handler, extract::State, http::StatusCode, routing::get, Json, Router};
use tracing::info;

use crate::error::AppError;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/static_files", get(static_files))
}

#[debug_handler]
async fn static_files(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<()>), AppError> {
    info!("static files");

    Ok((StatusCode::OK, Json(())))
}
