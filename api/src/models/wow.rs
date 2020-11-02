use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Addon {
    name: String,
    version: String,
    latest_version: String,
    enabled: bool,
    url: String,
    description: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Macro {
    name: String,
    r#macro: String,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct MacroClass {
    character_class: String,
    macros: Vec<Macro>,
}
