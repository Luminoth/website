use serde::Serialize;

use crate::models::wow;

#[derive(Serialize)]
struct GetAddonsResponse {
    wow_version: String,
    addons: Vec<wow::Addon>,
}

pub async fn get_addons_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let wow_version = "1.0".to_owned();
    let addons = vec![];

    Ok(warp::reply::json(&GetAddonsResponse {
        wow_version,
        addons,
    }))
}

#[derive(Serialize)]
struct GetMacrosResponse {
    macros: Vec<wow::MacroClass>,
}

pub async fn get_macros_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let macros = vec![];

    Ok(warp::reply::json(&GetMacrosResponse { macros }))
}

#[derive(Serialize)]
struct GetScreenshotsResponse {
    screenshots: Vec<String>,
}

pub async fn get_screenshots_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let screenshots = vec![];

    Ok(warp::reply::json(&GetScreenshotsResponse { screenshots }))
}

#[derive(Serialize)]
struct GetScreenshotResponse {
    screenshot_id: String,
}

pub async fn get_screenshot_handler(
    screenshot_id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&GetScreenshotResponse { screenshot_id }))
}
