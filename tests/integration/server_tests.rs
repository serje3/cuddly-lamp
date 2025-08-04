use openai_grpc::config::AppConfig;

#[test]
fn test_config_initialization() {
    // Тест инициализации конфигурации
    let result = openai_grpc::config::init();
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert!(config.openai_api_key.len() > 0);
}

#[test]
fn test_config_default_values() {
    let config = AppConfig::default();
    assert_eq!(config.openai_api_key, "");
    assert_eq!(config.bind_addr, Some("127.0.0.1:50051".to_string()));
    assert_eq!(config.log_level, Some("INFO".to_string()));
}

#[test]
fn test_config_validation() {
    let mut config = AppConfig::default();
    config.openai_api_key = "test_key".to_string();
    
    let result = config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_config_validation_empty_key() {
    let config = AppConfig::default();
    
    let result = config.validate();
    assert!(result.is_err());
} 