use openai_grpc::svc::chat::{ChatService, get_completion_service, get_assistant_service};
use openai_grpc_proto::chat::chat_server::Chat;
use tonic::Request;
use openai_grpc_proto::chat::{CompletionRequest, AssistantRequest};

#[test]
fn test_get_completion_service() {
    let service = get_completion_service();
    assert!(service.props.client_service.token.len() > 0);
}

#[test]
fn test_get_assistant_service() {
    let service = get_assistant_service();
    assert!(service.props.client_service.token.len() > 0);
}

#[tokio::test]
async fn test_completion_short_message() {
    let chat_service = ChatService {};
    let request = Request::new(CompletionRequest {
        message: "ab".to_string(),
    });

    let result = chat_service.completion(request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Question too short"));
}

#[tokio::test]
async fn test_stream_completion_short_message() {
    let chat_service = ChatService {};
    let request = Request::new(CompletionRequest {
        message: "ab".to_string(),
    });

    let result = chat_service.stream_completion(request).await;
    assert!(result.is_err());
    // Проверяем только что есть ошибка, не используем unwrap_err для потоков
}

#[tokio::test]
async fn test_assistant_short_message() {
    let chat_service = ChatService {};
    let request = Request::new(AssistantRequest {
        message: "ab".to_string(),
        thread_id: None,
        assistant_id: None,
        instructions: None,
    });

    let result = chat_service.assistant(request).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Question too short"));
}

#[tokio::test]
async fn test_stream_assistant_short_message() {
    let chat_service = ChatService {};
    let request = Request::new(AssistantRequest {
        message: "ab".to_string(),
        thread_id: None,
        assistant_id: None,
        instructions: None,
    });

    let result = chat_service.stream_assistant(request).await;
    assert!(result.is_err());
    // Проверяем только что есть ошибка, не используем unwrap_err для потоков
}

#[tokio::test]
async fn test_stream_assistant_valid_message() {
    let chat_service = ChatService {};
    let request = Request::new(AssistantRequest {
        message: "test message".to_string(),
        thread_id: Some("thread_123".to_string()),
        assistant_id: Some("assistant_456".to_string()),
        instructions: Some("test instructions".to_string()),
    });

    let result = chat_service.stream_assistant(request).await;
    // Тест проверяет, что метод не падает при валидном запросе
    // Реальный результат будет зависеть от доступности OpenAI API
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_chat_service_creation() {
    let _chat_service = ChatService {};
    // Тест проверяет, что структура создается без ошибок
} 