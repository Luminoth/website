use std::sync::Arc;

use crate::options::Options;

#[derive(Debug, Clone)]
pub struct AppState {
    pub options: Arc<Options>,
    pub aws_config: Arc<aws_config::SdkConfig>,
}

impl AppState {
    pub fn new(options: Options, aws_config: aws_config::SdkConfig) -> Self {
        Self {
            options: Arc::new(options),
            aws_config: Arc::new(aws_config),
        }
    }
}
