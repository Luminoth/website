use std::fs::File;
use std::io::BufReader;

use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::error::AppError;
use crate::models::pictures;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct GetPicturesResponse {
    pictures: Vec<pictures::Pictures>,
}

#[debug_handler]
pub async fn get_pictures_vacation_handler(
    Path(id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<GetPicturesResponse>, AppError> {
    let pics_file_path = app_state
        .options
        .share_dir()
        .join("vacation")
        .join(format!("{}.json", id));

    let file = match File::open(&pics_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(
                anyhow::anyhow!("Failed to read picture file {:?}: {}", pics_file_path, e).into(),
            );
        }
    };

    let reader = BufReader::new(file);

    let pictures = match serde_json::from_reader(reader) {
        Ok(pictures) => pictures,
        Err(e) => return Err(anyhow::anyhow!("Failed to parse picture file: {}", e).into()),
    };

    Ok(Json(GetPicturesResponse { pictures }))
}
