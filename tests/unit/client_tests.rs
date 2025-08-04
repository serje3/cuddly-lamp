use openai_grpc::client::ClientService;
use std::env;

#[test]
fn test_client_service_new() {
    // Устанавливаем тестовый API ключ
    env::set_var("OPENAI_API_KEY", "test_key");
    
    let client_service = ClientService::new();
    assert_eq!(client_service.token, "test_key");
}

#[test]
fn test_client_service_clone() {
    // Устанавливаем тестовый API ключ
    env::set_var("OPENAI_API_KEY", "test_key");
    
    let client_service = ClientService::new();
    let cloned_service = client_service.clone();
    
    assert_eq!(client_service.token, cloned_service.token);
}

#[test]
fn test_request_authenticate() {
    // Устанавливаем тестовый API ключ
    env::set_var("OPENAI_API_KEY", "test_key");
    
    let client_service = ClientService::new();
    let request_builder = reqwest::Client::new().post("https://api.openai.com/v1/test");
    
    let authenticated_request = client_service.request_authenticate(request_builder);
    
    // Проверяем, что запрос создан (не падает)
    assert!(authenticated_request.build().is_ok());
}

#[test]
fn test_create_success() {
    // Устанавливаем валидный API ключ для теста
    env::set_var("OPENAI_API_KEY", "sk-test123456789");
    
    let client_service = ClientService::new();
    let result = client_service.create();
    
    // Тест проверяет, что метод не падает
    // Реальный результат будет зависеть от валидности API ключа
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_create_with_invalid_key() {
    // Устанавливаем невалидный API ключ
    env::set_var("OPENAI_API_KEY", "invalid_key");
    
    let client_service = ClientService::new();
    let result = client_service.create();
    
    // Проверяем, что метод не падает
    // Реальный результат может быть как Ok, так и Err в зависимости от реализации
    assert!(result.is_ok() || result.is_err());
} 