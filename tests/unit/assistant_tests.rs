use openai_grpc::services::assistant::AssistantService;
use openai_grpc::client::ClientService;
use openai_grpc::services::assistant::wrap_tonic_status;
use openai_grpc::services::base::OpenAIService;
use openai_api_rs::v1::error::APIError;
use tokio::runtime::Runtime;

#[test]
fn test_assistant_service_new() {
    let client_service = ClientService::mock();
    let assistant_service = AssistantService::new(client_service);
    
    assert!(assistant_service.props.client_service.token == "test_token");
}

#[test]
fn test_wrap_tonic_status_success() {
    let future = async {
        Ok::<String, APIError>("test".to_string())
    };
    
    let result = Runtime::new().unwrap().block_on(wrap_tonic_status(future));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
}

#[tokio::test]
async fn test_stream_assistant_creation() {
    let client_service = ClientService::mock();
    let assistant_service = AssistantService::new(client_service);
    
    let result = assistant_service.stream_assistant(
        "test message",
        Some("assistant_id".to_string()),
        Some("thread_id".to_string()),
        Some("test instructions".to_string()),
    ).await;
    
    // Тест проверяет, что метод не падает при создании
    assert!(result.is_ok());
    
    let _stream = result.unwrap();
    // Проверяем, что поток создан успешно
}

#[tokio::test]
async fn test_stream_assistant_with_minimal_params() {
    let client_service = ClientService::mock();
    let assistant_service = AssistantService::new(client_service);
    
    let result = assistant_service.stream_assistant(
        "test",
        None,
        None,
        None,
    ).await;
    
    assert!(result.is_ok());
}

#[test]
fn test_assistant_service_properties() {
    let client_service = ClientService::mock();
    let assistant_service = AssistantService::new(client_service);
    
    let properties = assistant_service.properties();
    assert_eq!(properties.client_service.token, "test_token");
} 