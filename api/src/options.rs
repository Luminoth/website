use std::path::PathBuf;
use std::sync::Arc;

use argh::FromArgs;
use parking_lot::RwLock;

/// Website API
#[derive(FromArgs, Debug)]
pub struct Options {
    /// address to bind to
    #[argh(option, short = 'h', default = "String::from(\"0.0.0.0\")")]
    pub host: String,

    /// port to liten on
    #[argh(option, short = 'p', default = "8000")]
    pub port: u16,

    /// data prefix
    #[argh(option, default = "PathBuf::from(\".\")")]
    prefix: PathBuf,

    /// set this when running in production
    #[argh(switch)]
    pub prod: bool,

    /// enable tokio tracing
    #[argh(switch)]
    pub tracing: bool,
}

impl Options {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[allow(dead_code)]
    pub fn conf_dir(&self) -> PathBuf {
        self.prefix.join("etc")
    }

    #[allow(dead_code)]
    pub fn share_dir(&self) -> PathBuf {
        self.prefix.join("share")
    }
}

pub type SharedOptions = Arc<RwLock<Options>>;
