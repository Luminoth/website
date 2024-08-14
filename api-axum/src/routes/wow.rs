use axum::{debug_handler, extract::State, http::StatusCode, routing::get, Json, Router};
use tracing::info;

use crate::error::AppError;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/wow", get(wow))
}

#[debug_handler]
async fn wow(State(app_state): State<AppState>) -> Result<(StatusCode, Json<()>), AppError> {
    info!("wow");

    Ok((StatusCode::OK, Json(())))
}
