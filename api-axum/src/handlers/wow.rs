use std::fs::File;
use std::io::BufReader;

use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::models::wow;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
struct AddonsFile {
    wow_version: String,
    addons: Vec<wow::Addon>,
}

#[derive(Debug, Serialize)]
pub struct GetAddonsResponse {
    wow_version: String,
    addons: Vec<wow::Addon>,
}

#[debug_handler]
pub async fn get_addons_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetAddonsResponse>, AppError> {
    let addons_file_path = app_state
        .options
        .share_dir()
        .join("wow")
        .join("addons.json");

    let file = match File::open(&addons_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to read wow addons file {:?}: {}",
                addons_file_path,
                e
            )
            .into());
        }
    };

    let reader = BufReader::new(file);

    let addons: AddonsFile = match serde_json::from_reader(reader) {
        Ok(addons) => addons,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to parse wow addons file: {}", e).into());
        }
    };

    Ok(Json(GetAddonsResponse {
        wow_version: addons.wow_version,
        addons: addons.addons,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetMacrosResponse {
    macro_classes: Vec<wow::MacroClass>,
}

#[debug_handler]
pub async fn get_macros_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetMacrosResponse>, AppError> {
    let macros_file_path = app_state
        .options
        .share_dir()
        .join("wow")
        .join("macros.json");

    let file = match File::open(&macros_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to read wow macros file {:?}: {}",
                macros_file_path,
                e
            )
            .into());
        }
    };

    let reader = BufReader::new(file);

    let macro_classes = match serde_json::from_reader(reader) {
        Ok(macro_classes) => macro_classes,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to parse wow macros file: {}", e).into());
        }
    };

    Ok(Json(GetMacrosResponse { macro_classes }))
}

#[derive(Serialize)]
pub struct GetScreenshotsResponse {
    screenshots: Vec<wow::Screenshots>,
}

#[debug_handler]
pub async fn get_screenshots_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetScreenshotsResponse>, AppError> {
    get_screenshot_handler(Path("screenshots".to_owned()), State(app_state)).await
}

#[debug_handler]
pub async fn get_screenshot_handler(
    Path(id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<GetScreenshotsResponse>, AppError> {
    let screenshots_file_path = app_state
        .options
        .share_dir()
        .join("wow")
        .join(format!("{}.json", id));

    let file = match File::open(&screenshots_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Failed to read wow screenshots file {:?}: {}",
                screenshots_file_path,
                e
            )
            .into());
        }
    };

    let reader = BufReader::new(file);

    let screenshots = match serde_json::from_reader(reader) {
        Ok(screenshots) => screenshots,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to parse wow screenshots file: {}", e).into());
        }
    };

    Ok(Json(GetScreenshotsResponse { screenshots }))
}
