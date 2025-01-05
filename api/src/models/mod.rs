pub mod downloads;
pub mod news;
pub mod pictures;
pub mod wow;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    version: String,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
