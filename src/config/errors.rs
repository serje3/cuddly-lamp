use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    // #[error("Invalid port number")]
    // InvalidPort,

    #[error("Missing OPENAI_API_KEY")]
    MissingOAIKey,

    #[error("Config parsing error: {0}")]
    ParseError(#[from] config::ConfigError),
}