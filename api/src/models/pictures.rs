use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Pictures {
    images: Vec<String>,

    #[serde(default)]
    text: String,
}
