use axum::{debug_handler, extract::State, http::StatusCode, routing::get, Json, Router};
use tracing::info;

use crate::error::AppError;
use crate::state::AppState;

pub fn init_routes(app: Router<AppState>) -> Router<AppState> {
    app.route("/news", get(news))
}

#[debug_handler]
pub async fn news(State(app_state): State<AppState>) -> Result<(StatusCode, Json<()>), AppError> {
    info!("news");

    Ok((StatusCode::OK, Json(())))
}
