pub mod downloads;
pub mod news;
pub mod pictures;
pub mod static_files;
pub mod wow;

use axum::{debug_handler, http::StatusCode, http::Uri, response::IntoResponse};
use tracing::debug;

#[debug_handler]
pub async fn handler_404(uri: Uri) -> impl IntoResponse {
    debug!("invalid resource: {}", uri);

    (StatusCode::NOT_FOUND, "Resource not found")
}
