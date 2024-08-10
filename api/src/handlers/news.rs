use serde::Serialize;

use energonsoftware::aws::dynamodb;

use super::internal_error;
use crate::models::news;
use crate::state::AppState;

#[derive(Serialize)]
struct GetNewsAuthorsResponse {
    news_authors: Vec<news::NewsAuthor>,
}

pub async fn get_news_authors_handler(
    app_state: AppState,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("news_author")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Ok(internal_error(format!("Failed to build expression: {}", e)));
        }
    };

    let client = dynamodb::connect(app_state.get_aws_config()).await;

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
            return Ok(internal_error(format!("Error reading news authors: {}", e)));
        }
    }

    let news_authors = news_authors.drain(..).map(|x| x.into()).collect();

    Ok(Box::new(warp::reply::json(&GetNewsAuthorsResponse {
        news_authors,
    })))
}

#[derive(Serialize)]
struct GetNewsResponse {
    news: Vec<news::News>,
}

pub async fn get_news_handler(
    app_state: AppState,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let builder = dynamodb_expression::Builder::new().with_key_condition(
        dynamodb_expression::key("type").equal(dynamodb_expression::value("news")),
    );

    let expression = match builder.build() {
        Ok(expression) => expression,
        Err(e) => {
            return Ok(internal_error(format!("Failed to build expression: {}", e)));
        }
    };

    let client = dynamodb::connect(app_state.get_aws_config()).await;

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
            return Ok(internal_error(format!("Error reading news: {}", e)));
        }
    }

    let news = news.drain(..).map(|x| x.into()).collect();

    Ok(Box::new(warp::reply::json(&GetNewsResponse { news })))
}
