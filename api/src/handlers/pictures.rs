use serde::Serialize;

use crate::models::pictures;

#[derive(Serialize)]
struct GetPicturesResponse {
    pictures: Vec<pictures::Pictures>,
}

pub async fn get_pictures_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let pictures = vec![];

    Ok(warp::reply::json(&GetPicturesResponse { pictures }))
}

#[derive(Serialize)]
struct GetPictureResponse {
    picture_id: String,
}

pub async fn get_picture_handler(picture_id: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&GetPictureResponse { picture_id }))
}
