pub mod downloads;
pub mod news;
pub mod pictures;
pub mod wow;

use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for ErrorResponse {}

fn internal_error(message: impl AsRef<str>) -> Box<dyn warp::Reply> {
    error!("{}", message.as_ref());

    Box::new(warp::reply::with_status(
        warp::reply::json(&ErrorResponse {
            message: "Internal Error".to_owned(),
        }),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
