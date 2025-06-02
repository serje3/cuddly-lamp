use std::pin::Pin;
use crate::client::ClientService;
use crate::services::base::{OpenAIService, ServiceProperties};
use crate::TEXT_GPT_MODEL;
use futures_util::StreamExt;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_grpc_proto::chat::CompletionResponse;
use reqwest::{Client, Response};
use serde_json::json;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Status;
pub struct CompletionService {
    props: ServiceProperties,
}

impl OpenAIService for CompletionService {
    fn properties(&self) -> &ServiceProperties {
        &self.props
    }
}

impl CompletionService {
    pub fn new(client_service: ClientService) -> Self {
        Self {
            props: ServiceProperties {
                client: client_service.create().unwrap(),
                client_service: client_service,
            },
        }
    }

    fn create_completion_request(&self, question: &String, stream: bool) -> ChatCompletionRequest {
        let req = ChatCompletionRequest::new(
            TEXT_GPT_MODEL.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(question.clone()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        req.stream(stream)
    }

    pub async fn completion(&mut self, content: &String) -> Result<String, tonic::Status> {
        let openai_req = self.create_completion_request(content, false);

        let openai_resp = match self.props.client.chat_completion(openai_req).await {
            Ok(resp) => resp,
            Err(error) => return Err(tonic::Status::internal(error.to_string())),
        };
        let openai_resp_choice = match openai_resp.choices.first() {
            Some(choice) => choice,
            None => return Err(tonic::Status::internal("no choices")),
        };
        match &openai_resp_choice.message.content {
            Some(content) => Ok(content.clone()),
            None => return Err(tonic::Status::internal("No message found")),
        }
    }

    pub async fn stream_completion_response(&self, question: &String) -> Result<Response, Status> {
        let client = Client::builder().build().expect("No client reqwest");

        match self
            .props
            .client_service
            .request_authenticate(client.post("https://api.openai.com/v1/chat/completions"))
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "model": TEXT_GPT_MODEL,
                    "messages": [
                        {"role": "user", "content": question}
                    ],
                    "stream": true
                })
                .to_string(),
            )
            .send()
            .await
        {
            Ok(res) => Ok(res),
            Err(_) => Err(Status::internal("Wrong stream request")),
        }
    }

    pub async fn stream_completion(&self, question: &String) -> Result<Pin<Box<ReceiverStream<Result<CompletionResponse, Status>>>>, Status> {
        let res = self.stream_completion_response(question).await?;
        let mut stream = res.bytes_stream();

        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.unwrap();
                let text = String::from_utf8_lossy(&chunk);
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let json_line = &line[6..];
                        if json_line == "[DONE]" {
                            break;
                        }

                        let parsed: serde_json::Value = serde_json::from_str(json_line).unwrap();
                        if let Some(answer_delta) = parsed["choices"][0]["delta"]["content"].as_str() {
                            tx.send(Ok(CompletionResponse {
                                message: answer_delta.to_string(),
                            }))
                            .await
                            .unwrap();
                        }
                    }
                }
            }
        });

        Ok(Box::pin(ReceiverStream::new(rx)))
    }
}
