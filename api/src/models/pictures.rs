use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Pictures {
    // TODO: should have per-image alt text
    images: Vec<String>,

    #[serde(default)]
    text: String,
}
