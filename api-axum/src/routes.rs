use axum::{debug_handler, extract::State, http::StatusCode, Json};
use tracing::info;

use crate::error::AppError;
use crate::state::AppState;

#[debug_handler]
pub async fn downloads(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<()>), AppError> {
    info!("downloads");

    Ok((StatusCode::OK, Json(())))
}

#[debug_handler]
pub async fn news(State(app_state): State<AppState>) -> Result<(StatusCode, Json<()>), AppError> {
    info!("news");

    Ok((StatusCode::OK, Json(())))
}

#[debug_handler]
pub async fn pictures(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<()>), AppError> {
    info!("pictures");

    Ok((StatusCode::OK, Json(())))
}

#[debug_handler]
pub async fn wow(State(app_state): State<AppState>) -> Result<(StatusCode, Json<()>), AppError> {
    info!("wow");

    Ok((StatusCode::OK, Json(())))
}

#[debug_handler]
pub async fn static_files(
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<()>), AppError> {
    info!("static files");

    Ok((StatusCode::OK, Json(())))
}
