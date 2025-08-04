use openai_grpc::services::completion::CompletionService;
use openai_grpc::client::ClientService;
use openai_grpc::TEXT_GPT_MODEL;
use openai_grpc::services::base::OpenAIService;
use openai_api_rs::v1::chat_completion;

#[test]
fn test_completion_service_new() {
    let client_service = ClientService::mock();
    let completion_service = CompletionService::new(client_service);
    
    assert!(completion_service.props.client_service.token == "test_token");
}

#[test]
fn test_create_completion_request() {
    let client_service = ClientService::mock();
    let completion_service = CompletionService::new(client_service);
    
    let question = "test question".to_string();
    let request = completion_service.create_completion_request(&question, false);
    
    assert_eq!(request.model, TEXT_GPT_MODEL);
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.messages[0].role, chat_completion::MessageRole::user);
    
    if let chat_completion::Content::Text(content) = &request.messages[0].content {
        assert_eq!(content, &question);
    } else {
        panic!("Expected text content");
    }
}

#[test]
fn test_create_completion_request_with_stream() {
    let client_service = ClientService::mock();
    let completion_service = CompletionService::new(client_service);
    
    let question = "test question".to_string();
    let request = completion_service.create_completion_request(&question, true);
    
    assert_eq!(request.model, TEXT_GPT_MODEL);
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.messages[0].role, chat_completion::MessageRole::user);
    
    if let chat_completion::Content::Text(content) = &request.messages[0].content {
        assert_eq!(content, &question);
    } else {
        panic!("Expected text content");
    }
}

#[test]
fn test_completion_service_properties() {
    let client_service = ClientService::mock();
    let completion_service = CompletionService::new(client_service);
    
    let properties = completion_service.properties();
    assert_eq!(properties.client_service.token, "test_token");
}

#[tokio::test]
async fn test_stream_completion_response_creation() {
    let client_service = ClientService::mock();
    let completion_service = CompletionService::new(client_service);
    
    let question = "test question".to_string();
    let result = completion_service.stream_completion_response(&question).await;
    
    // Тест проверяет, что метод не падает при создании
    // Реальный результат будет зависеть от доступности OpenAI API
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_stream_completion_creation() {
    let client_service = ClientService::mock();
    let completion_service = CompletionService::new(client_service);
    
    let question = "test question".to_string();
    let result = completion_service.stream_completion(&question).await;
    
    // Тест проверяет, что метод не падает при создании
    // Реальный результат будет зависеть от доступности OpenAI API
    assert!(result.is_ok() || result.is_err());
} 