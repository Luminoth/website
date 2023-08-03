use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    aws_config: Arc<aws_config::SdkConfig>,
}

impl AppState {
    pub fn new(aws_config: aws_config::SdkConfig) -> Self {
        Self {
            aws_config: Arc::new(aws_config),
        }
    }

    pub fn get_aws_config(&self) -> &aws_config::SdkConfig {
        &self.aws_config
    }
}
