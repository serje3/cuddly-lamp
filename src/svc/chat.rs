use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use openai_grpc_proto::chat::chat_server::{Chat};
use openai_grpc_proto::chat::{CompletionRequest, CompletionResponse, AssistantRequest, AssistantResponse};
use crate::client::ClientService;
use crate::services::assistant::AssistantService;
use crate::services::completion::CompletionService;

pub fn get_completion_service() -> CompletionService {
    CompletionService::new(ClientService::new())
}

pub fn get_assistant_service() -> AssistantService {
    AssistantService::new(ClientService::new())
}

pub struct ChatService {
}

#[tonic::async_trait]
impl Chat for ChatService {


    async fn completion(
        &self,
        request: Request<CompletionRequest>,
    ) -> Result<Response<CompletionResponse>, Status> {
        let question = request.into_inner().message;
        if question.len() < 3 {
            return Err(Status::invalid_argument("Question too short"));
        }

        let answer = CompletionResponse {
            message: get_completion_service()
                .completion(&question).await?.to_string(),
        };

        Ok(Response::new(answer))
    }

    type StreamCompletionStream =
    Pin<Box<dyn Stream<Item = Result<CompletionResponse, tonic::Status>> + Send + Sync + 'static>>;

    async fn stream_completion(
        &self,
        request: Request<CompletionRequest>,
    ) -> Result<Response<Self::StreamCompletionStream>, Status> {
        let question = request.into_inner().message;
        if question.len() < 3 {
            return Err(Status::invalid_argument("Question too short"));
        }

        Ok(Response::new(get_completion_service().stream_completion(&question).await?))
    }

    /// Server streaming response type for the StreamAssistant method.
    type StreamAssistantStream = Pin<Box<dyn Stream<Item =
    Result<AssistantResponse, Status>> + Send + Sync + 'static>>;


    async fn assistant(
        &self,
        request: Request<AssistantRequest>,
    ) -> Result<Response<AssistantResponse>, Status> {
        let req = request.into_inner();
        let question = req.message;
        let assistant_id = req.assistant_id;
        let instructions = req.instructions;
        let thread_id = req.thread_id;
        if question.len() < 3 {
            return Err(Status::invalid_argument("Question too short"));
        }
        let (answer, thread_id) = get_assistant_service()
            .get_text(question.as_str(), assistant_id, thread_id, instructions).await?;
        let answer = AssistantResponse {
            message: answer.to_string(),
            thread_id: thread_id.to_string(),
        };

        Ok(Response::new(answer))
    }

    async fn stream_assistant(&self, request: Request<AssistantRequest>) -> Result<Response<Self::StreamAssistantStream>, Status> {
        let req = request.into_inner();
        let question = req.message;
        let assistant_id = req.assistant_id;
        let instructions = req.instructions;
        let thread_id = req.thread_id;
        
        if question.len() < 3 {
            return Err(Status::invalid_argument("Question too short"));
        }

        Ok(Response::new(get_assistant_service().stream_assistant(
            &question,
            assistant_id,
            thread_id,
            instructions,
        ).await?))
    }

}
