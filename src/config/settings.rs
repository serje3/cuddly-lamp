use config::ConfigError;
use serde::Deserialize;
use crate::config::env::require_env_var;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub base_gpt_url: String,
    pub openai_api_key: String,
    pub bind_addr: String,
    pub log_level: String
}


impl Default for AppConfig {
    fn default() -> Self {
        Self {
            base_gpt_url: "".to_string(),
            openai_api_key: "".to_string(),
            bind_addr: "127.0.0.1:50051".to_string(),
            log_level: "INFO".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }

    pub fn validate(&self) -> Result<(), crate::config::errors::ConfigError> {
        if self.openai_api_key.len() == 0 || self.openai_api_key.is_empty() {
            return Err(crate::config::errors::ConfigError::MissingOAIKey)
        }
        Ok(())
    }
}