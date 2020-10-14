use dynamodb_expression::*;
use serde::Serialize;

use energonsoftware::aws::dynamodb;

use super::internal_error;
use crate::models::downloads;
use crate::REGION;

#[derive(Serialize)]
struct GetDownloadCategoriesResponse {
    download_categories: Vec<downloads::DownloadCategory>,
}

pub async fn get_download_categories_handler() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = Builder::new().with_key_condition(key("type").equal(value("download_category")));

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to build expression: {}",
                e.to_string()
            )));
        }
    };

    let client = match dynamodb::connect(REGION).await {
        Ok(client) => client,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to connect dynamodb: {}",
                e.to_string()
            )));
        }
    };

    let mut download_categories = Vec::new();
    match dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut download_category = downloads::DownloadCategory::default();
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
                e.to_string()
            )));
        }
    }

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

pub async fn get_downloads_handler() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = Builder::new().with_key_condition(key("type").equal(value("download")));

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to build expression: {}",
                e.to_string()
            )));
        }
    };

    let client = match dynamodb::connect(REGION).await {
        Ok(client) => client,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to connect dynamodb: {}",
                e.to_string()
            )));
        }
    };

    let mut downloads = Vec::new();
    match dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut download = downloads::Download::default();
        deserialize(&mut download)?;

        downloads.push(download.clone());

        Ok((download, false))
    })
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Ok(internal_error(format!(
                "Error reading downloads: {}",
                e.to_string()
            )));
        }
    }

    Ok(Box::new(warp::reply::json(&GetDownloadsResponse {
        downloads,
    })))
}
