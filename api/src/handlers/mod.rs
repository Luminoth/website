pub mod downloads;
pub mod news;
pub mod pictures;
pub mod static_files;
pub mod wow;

use axum::{debug_handler, http::StatusCode, http::Uri, response::IntoResponse, Json};
use serde::Serialize;
use tracing::debug;

use crate::models;

#[derive(Debug, Serialize)]
pub struct GetStatusResponse {
    status: models::Status,
}

#[debug_handler]
pub async fn get_status() -> Json<GetStatusResponse> {
    Json(GetStatusResponse {
        status: models::Status::default(),
    })
}

#[debug_handler]
pub async fn handler_404(uri: Uri) -> impl IntoResponse {
    debug!("invalid resource: {}", uri);

    (StatusCode::NOT_FOUND, "Resource not found")
}
