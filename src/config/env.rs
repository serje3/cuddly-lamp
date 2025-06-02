use crate::config::errors::ConfigError;

pub fn get_env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

pub fn require_env_var(key: &str) -> Result<String, ConfigError> {
    std::env::var(key)
        .map_err(|_| ConfigError::MissingEnvVar(key.to_string()))
}