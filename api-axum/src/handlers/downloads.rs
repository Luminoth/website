use axum::{debug_handler, extract::State, Json};
use serde::Serialize;

use energonsoftware::aws::dynamodb;

use crate::error::AppError;
use crate::models::downloads;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct GetDownloadsResponse {
    downloads: Vec<downloads::Download>,
}

#[debug_handler]
pub async fn get_downloads_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetDownloadsResponse>, AppError> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("download")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to build expression: {}", e).into());
        }
    };

    let client = dynamodb::connect(&app_state.aws_config).await;

    let mut downloads = Vec::new();
    let result = dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut download = downloads::DbDownload::default();
        deserialize(&mut download)?;

        downloads.push(download.clone());

        Ok((download, false))
    })
    .await;
    match result {
        Ok(_) => (),
        Err(e) => {
            return Err(anyhow::anyhow!("Error reading downloads: {}", e).into());
        }
    }

    let downloads = downloads.drain(..).map(|x| x.into()).collect();

    Ok(Json(GetDownloadsResponse { downloads }))
}

#[derive(Debug, Serialize)]
pub struct GetDownloadCategoriesResponse {
    download_categories: Vec<downloads::DownloadCategory>,
}

#[debug_handler]
pub async fn get_download_categories_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetDownloadCategoriesResponse>, AppError> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("download_category")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to build expression: {}", e).into());
        }
    };

    let client = dynamodb::connect(&app_state.aws_config).await;

    let mut download_categories = Vec::new();
    let result = dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut download_category = downloads::DbDownloadCategory::default();
        deserialize(&mut download_category)?;

        download_categories.push(download_category.clone());

        Ok((download_category, false))
    })
    .await;
    match result {
        Ok(_) => (),
        Err(e) => {
            return Err(anyhow::anyhow!("Error reading download categories: {}", e).into());
        }
    }

    let download_categories = download_categories.drain(..).map(|x| x.into()).collect();

    Ok(Json(GetDownloadCategoriesResponse {
        download_categories,
    }))
}
