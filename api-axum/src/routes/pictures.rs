use axum::{debug_handler, extract::State, http::StatusCode, routing::get, Json, Router};
use tracing::info;

use crate::error::AppError;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/pictures", get(pictures))
}

#[debug_handler]
pub async fn pictures(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<()>), AppError> {
    info!("pictures");

    Ok((StatusCode::OK, Json(())))
}
