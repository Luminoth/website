use dynamodb_expression::*;
use serde::Serialize;

use energonsoftware::aws::dynamodb;

use super::internal_error;
use crate::models::news;
use crate::REGION;

#[derive(Serialize)]
struct GetNewsAuthorsResponse {
    news_authors: Vec<news::NewsAuthor>,
}

pub async fn get_news_authors_handler() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = Builder::new().with_key_condition(key("type").equal(value("news_author")));

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

    let mut news_authors = Vec::new();
    match dynamodb::query(&client, "items", expression, None, |_, deserialize| {
        let mut news_author = news::NewsAuthor::default();
        deserialize(&mut news_author)?;

        news_authors.push(news_author.clone());

        Ok((news_author, false))
    })
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Ok(internal_error(format!(
                "Error reading news authors: {}",
                e.to_string()
            )));
        }
    }

    Ok(Box::new(warp::reply::json(&GetNewsAuthorsResponse {
        news_authors,
    })))
}

#[derive(Serialize)]
struct GetNewsResponse {
    news: Vec<news::News>,
}

pub async fn get_news_handler() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = Builder::new().with_key_condition(key("type").equal(value("news")));

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

    let mut news = Vec::new();
    match dynamodb::query_index_descending(
        &client,
        "items",
        expression,
        Some(10),
        "type-timestamp-index",
        |_, deserialize| {
            let mut news_ = news::News::default();
            deserialize(&mut news_)?;

            news.push(news_.clone());

            Ok((news_, false))
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Ok(internal_error(format!(
                "Error reading news: {}",
                e.to_string()
            )));
        }
    }

    Ok(Box::new(warp::reply::json(&GetNewsResponse { news })))
}
