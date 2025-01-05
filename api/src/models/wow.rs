use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Addon {
    name: String,
    version: String,
    latest_version: bool,
    enabled: bool,
    url: String,
    description: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Macro {
    name: String,
    r#macro: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MacroClass {
    character_class: String,
    macros: Vec<Macro>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Screenshots {
    // TODO: should have per-image alt text
    images: Vec<String>,

    #[serde(default)]
    text: String,
}
