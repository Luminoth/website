use serde::Serialize;

#[derive(Serialize)]
struct GetStaticFilesResponse {}

pub async fn get_static_files_handler(
    _region: impl Into<String>,
    _local: bool,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    // TODO: read the file / dir or whatever list from S3 and return that
    // if not prod tho, do that but using the local static files

    Ok(Box::new(warp::reply::json(&GetStaticFilesResponse {})))
}
