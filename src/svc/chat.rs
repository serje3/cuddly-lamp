use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use openai_grpc_proto::chat::chat_server::{Chat};
use openai_grpc_proto::chat::{CompletionRequest, CompletionResponse, AssistantRequest, AssistantResponse};
use crate::client::ClientService;
use crate::services::completion::CompletionService;

fn get_completion_service() -> CompletionService {
    CompletionService::new(ClientService::new())
}

fn get_assistant_service() {

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

    async fn stream_assistant(&self, request: Request<AssistantRequest>) -> Result<Response<Self::StreamAssistantStream>, Status> {
        todo!()
    }

}
