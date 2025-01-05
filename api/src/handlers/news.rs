use axum::{debug_handler, extract::State, Json};
use serde::Serialize;

use energonsoftware::aws::dynamodb;

use crate::error::AppError;
use crate::models::news;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct GetNewsResponse {
    news: Vec<news::News>,
}

#[debug_handler]
pub async fn get_news_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetNewsResponse>, AppError> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("news")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to build expression: {}", e).into());
        }
    };

    let client = dynamodb::connect(&app_state.aws_config).await;

    let mut news = Vec::new();
    let result = dynamodb::query_index_descending(
        &client,
        "items",
        expression,
        Some(10),
        "type-timestamp-index",
        |_, deserialize| {
            let mut news_ = news::DbNews::default();
            deserialize(&mut news_)?;

            news.push(news_.clone());

            Ok((news_, false))
        },
    )
    .await;
    match result {
        Ok(_) => (),
        Err(e) => {
            return Err(anyhow::anyhow!("Error reading news: {}", e).into());
        }
    }

    let news = news.drain(..).map(|x| x.into()).collect();

    Ok(Json(GetNewsResponse { news }))
}

#[derive(Debug, Serialize)]
pub struct GetNewsAuthorsResponse {
    news_authors: Vec<news::NewsAuthor>,
}

#[debug_handler]
pub async fn get_news_authors_handler(
    State(app_state): State<AppState>,
) -> Result<Json<GetNewsAuthorsResponse>, AppError> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("news_author")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to build expression: {}", e).into());
        }
    };

    let client = dynamodb::connect(&app_state.aws_config).await;

    let mut news_authors = Vec::new();
    let result = dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut news_author = news::DbNewsAuthor::default();
        deserialize(&mut news_author)?;

        news_authors.push(news_author.clone());

        Ok((news_author, false))
    })
    .await;
    match result {
        Ok(_) => (),
        Err(e) => {
            return Err(anyhow::anyhow!("Error reading news authors: {}", e).into());
        }
    }

    let news_authors = news_authors.drain(..).map(|x| x.into()).collect();

    Ok(Json(GetNewsAuthorsResponse { news_authors }))
}
