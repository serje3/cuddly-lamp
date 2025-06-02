mod settings;
mod env;
mod errors;

use config::ConfigError;
use dotenv::dotenv;
pub use settings::AppConfig;

/// Инициализация конфигурации
pub fn init() -> Result<AppConfig, ConfigError> {
    dotenv().ok();
    AppConfig::from_env()
}