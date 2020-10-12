use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Pictures {
    images: Vec<String>,
    text: String,
}
