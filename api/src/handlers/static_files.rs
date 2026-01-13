use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::Serialize;

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
    Path(_path): Path<String>,
    State(_app_state): State<AppState>,
) -> Result<(), AppError> {
    // TODO: this should only be available when not prod
    // and should just return the static file

    Ok(())
}
