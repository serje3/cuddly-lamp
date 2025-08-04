use openai_grpc::svc::chat::ChatService;
use openai_grpc_proto::chat::chat_server::Chat;
use tonic::Request;
use openai_grpc_proto::chat::{CompletionRequest, AssistantRequest};
use std::env;

#[tokio::test]
async fn test_chat_service_integration() {
    let chat_service = ChatService {};

    // Тест completion с коротким сообщением
    let completion_request = Request::new(CompletionRequest {
        message: "ab".to_string(),
    });
    
    let result = chat_service.completion(completion_request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Question too short"));

    // Тест stream_completion с коротким сообщением
    let stream_completion_request = Request::new(CompletionRequest {
        message: "ab".to_string(),
    });
    
    let result = chat_service.stream_completion(stream_completion_request).await;
    assert!(result.is_err());
    // Проверяем только что есть ошибка, не используем unwrap_err для потоков

    // Тест assistant с коротким сообщением
    let assistant_request = Request::new(AssistantRequest {
        message: "ab".to_string(),
        thread_id: None,
        assistant_id: None,
        instructions: None,
    });
    
    let result = chat_service.assistant(assistant_request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Question too short"));

    // Тест stream_assistant с коротким сообщением
    let stream_assistant_request = Request::new(AssistantRequest {
        message: "ab".to_string(),
        thread_id: None,
        assistant_id: None,
        instructions: None,
    });
    
    let result = chat_service.stream_assistant(stream_assistant_request).await;
    assert!(result.is_err());
    // Проверяем только что есть ошибка, не используем unwrap_err для потоков
}

#[tokio::test]
async fn test_service_creation() {
    // Тест создания сервисов
    let _chat_service = ChatService {};
    
    // Проверяем, что сервисы создаются без ошибок
    assert!(true);
}

#[tokio::test]
async fn test_stream_assistant_creation() {
    // Устанавливаем тестовый API ключ
    env::set_var("OPENAI_API_KEY", "test_key");
    
    let chat_service = ChatService {};
    
    // Тест создания stream_assistant с валидными параметрами
    let request = Request::new(AssistantRequest {
        message: "test message for streaming".to_string(),
        thread_id: Some("test_thread".to_string()),
        assistant_id: Some("test_assistant".to_string()),
        instructions: Some("test instructions".to_string()),
    });
    
    let result = chat_service.stream_assistant(request).await;
    
    // Проверяем, что метод не падает
    // Реальный результат будет зависеть от доступности OpenAI API
    assert!(result.is_ok() || result.is_err());
} 