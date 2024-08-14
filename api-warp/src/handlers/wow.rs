use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::internal_error;
use crate::models::wow;

#[derive(Deserialize)]
struct AddonsFile {
    wow_version: String,
    addons: Vec<wow::Addon>,
}

#[derive(Serialize)]
struct GetAddonsResponse {
    wow_version: String,
    addons: Vec<wow::Addon>,
}

pub async fn get_addons_handler(
    share_dir: impl AsRef<Path>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let addons_file_path = share_dir.as_ref().join("wow").join("addons.json");

    let file = match File::open(&addons_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to read wow addons file {:?}: {}",
                addons_file_path, e
            )));
        }
    };

    let reader = BufReader::new(file);

    let addons: AddonsFile = match serde_json::from_reader(reader) {
        Ok(addons) => addons,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to parse wow addons file: {}",
                e
            )));
        }
    };

    Ok(Box::new(warp::reply::json(&GetAddonsResponse {
        wow_version: addons.wow_version,
        addons: addons.addons,
    })))
}

#[derive(Serialize)]
struct GetMacrosResponse {
    macro_classes: Vec<wow::MacroClass>,
}

pub async fn get_macros_handler(
    share_dir: impl AsRef<Path>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let macros_file_path = share_dir.as_ref().join("wow").join("macros.json");

    let file = match File::open(&macros_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to read wow macros file {:?}: {}",
                macros_file_path, e
            )));
        }
    };

    let reader = BufReader::new(file);

    let macro_classes = match serde_json::from_reader(reader) {
        Ok(macro_classes) => macro_classes,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to parse wow macros file: {}",
                e
            )));
        }
    };

    Ok(Box::new(warp::reply::json(&GetMacrosResponse {
        macro_classes,
    })))
}

#[derive(Serialize)]
struct GetScreenshotsResponse {
    screenshots: Vec<wow::Screenshots>,
}

pub async fn get_screenshots_handler(
    id: impl AsRef<str>,
    share_dir: impl AsRef<Path>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let screenshots_file_path = share_dir
        .as_ref()
        .join("wow")
        .join(format!("{}.json", id.as_ref()));

    let file = match File::open(&screenshots_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to read wow screenshots file {:?}: {}",
                screenshots_file_path, e
            )));
        }
    };

    let reader = BufReader::new(file);

    let screenshots = match serde_json::from_reader(reader) {
        Ok(screenshots) => screenshots,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to parse wow screenshots file: {}",
                e
            )));
        }
    };

    Ok(Box::new(warp::reply::json(&GetScreenshotsResponse {
        screenshots,
    })))
}
