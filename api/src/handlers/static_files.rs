use std::path::Path as StdPath;

use axum::{
    Json,
    body::Body,
    debug_handler,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tokio::fs;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct GetStaticFilesResponse {}

#[debug_handler]
pub async fn get_static_files_handler(
    State(_app_state): State<AppState>,
) -> Result<Json<GetStaticFilesResponse>, AppError> {
    // TODO: read the file / dir or whatever list from S3 and return that
    // if not prod tho, do that but using the local static files

    // NOTE: it's unsafe to hard-code the bucket name here
    // (or anywhere else in source)

    Ok(Json(GetStaticFilesResponse {}))
}

#[debug_handler]
pub async fn get_static_file_handler(
    Path(path): Path<String>,
    State(app_state): State<AppState>,
) -> Response {
    if app_state.options.prod {
        return StatusCode::NOT_FOUND.into_response();
    }

    let file_path = app_state.options.static_dir().join(&path);

    let Ok(content) = fs::read(&file_path).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let content_type = content_type_from_path(&path);

    Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(content))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

fn content_type_from_path(path: &str) -> &'static str {
    let ext = StdPath::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "application/octet-stream",
    }
}
