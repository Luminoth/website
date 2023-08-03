use serde::Serialize;

use energonsoftware::aws::dynamodb;

use super::internal_error;
use crate::models::downloads;
use crate::state::AppState;

#[derive(Serialize)]
struct GetDownloadCategoriesResponse {
    download_categories: Vec<downloads::DownloadCategory>,
}

pub async fn get_download_categories_handler(
    app_state: AppState,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("download_category")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Ok(internal_error(format!("Failed to build expression: {}", e)));
        }
    };

    let client = dynamodb::connect(app_state.get_aws_config()).await;

    let mut download_categories = Vec::new();
    match dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut download_category = downloads::DbDownloadCategory::default();
        deserialize(&mut download_category)?;

        download_categories.push(download_category.clone());

        Ok((download_category, false))
    })
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Ok(internal_error(format!(
                "Error reading download categories: {}",
                e
            )));
        }
    }

    let download_categories = download_categories.drain(..).map(|x| x.into()).collect();

    Ok(Box::new(warp::reply::json(
        &GetDownloadCategoriesResponse {
            download_categories,
        },
    )))
}

#[derive(Serialize)]
struct GetDownloadsResponse {
    downloads: Vec<downloads::Download>,
}

pub async fn get_downloads_handler(
    app_state: AppState,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("download")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Ok(internal_error(format!("Failed to build expression: {}", e)));
        }
    };

    let client = dynamodb::connect(app_state.get_aws_config()).await;

    let mut downloads = Vec::new();
    match dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut download = downloads::DbDownload::default();
        deserialize(&mut download)?;

        downloads.push(download.clone());

        Ok((download, false))
    })
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Ok(internal_error(format!("Error reading downloads: {}", e)));
        }
    }

    let downloads = downloads.drain(..).map(|x| x.into()).collect();

    Ok(Box::new(warp::reply::json(&GetDownloadsResponse {
        downloads,
    })))
}
