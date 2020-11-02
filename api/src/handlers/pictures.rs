use std::fs::File;
use std::io::BufReader;

use serde::Serialize;

use super::internal_error;
use crate::models::pictures;
use crate::OPTIONS;

#[derive(Serialize)]
struct GetPicturesResponse {
    pictures: Vec<pictures::Pictures>,
}

pub async fn get_pictures_vacation_handler(
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let pics_file_path = OPTIONS
        .read()
        .share_dir()
        .join("vacation")
        .join(format!("{}.json", id));

    let file = match File::open(&pics_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to read picture file {:?}: {}",
                pics_file_path,
                e.to_string()
            )));
        }
    };

    let reader = BufReader::new(file);

    let pictures = match serde_json::from_reader(reader) {
        Ok(pictures) => pictures,
        Err(e) => {
            return Ok(internal_error(format!(
                "Failed to parse picture file: {}",
                e.to_string()
            )));
        }
    };

    Ok(Box::new(warp::reply::json(&GetPicturesResponse {
        pictures,
    })))
}
