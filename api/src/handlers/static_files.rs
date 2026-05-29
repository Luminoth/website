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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpg_and_jpeg_map_to_image_jpeg() {
        assert_eq!(content_type_from_path("photo.jpg"), "image/jpeg");
        assert_eq!(content_type_from_path("photo.jpeg"), "image/jpeg");
    }

    #[test]
    fn png_maps_to_image_png() {
        assert_eq!(content_type_from_path("image.png"), "image/png");
    }

    #[test]
    fn gif_maps_to_image_gif() {
        assert_eq!(content_type_from_path("anim.gif"), "image/gif");
    }

    #[test]
    fn svg_maps_to_image_svg_xml() {
        assert_eq!(content_type_from_path("icon.svg"), "image/svg+xml");
    }

    #[test]
    fn webp_maps_to_image_webp() {
        assert_eq!(content_type_from_path("photo.webp"), "image/webp");
    }

    #[test]
    fn css_maps_to_text_css() {
        assert_eq!(content_type_from_path("styles.css"), "text/css");
    }

    #[test]
    fn js_maps_to_application_javascript() {
        assert_eq!(content_type_from_path("app.js"), "application/javascript");
    }

    #[test]
    fn unknown_extension_maps_to_octet_stream() {
        assert_eq!(content_type_from_path("data.bin"), "application/octet-stream");
    }

    #[test]
    fn no_extension_maps_to_octet_stream() {
        assert_eq!(content_type_from_path("Makefile"), "application/octet-stream");
    }

    #[test]
    fn extension_matching_is_case_insensitive() {
        assert_eq!(content_type_from_path("photo.JPG"), "image/jpeg");
        assert_eq!(content_type_from_path("photo.PNG"), "image/png");
    }

    #[test]
    fn path_with_directories_uses_final_extension() {
        assert_eq!(content_type_from_path("images/vacation/photo.jpg"), "image/jpeg");
    }
}
