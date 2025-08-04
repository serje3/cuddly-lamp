use crate::client::ClientService;
use crate::services::base::{OpenAIService, ServiceProperties};
use crate::TEXT_GPT_MODEL;
use openai_api_rs::v1::assistant::{AssistantObject, AssistantRequest};
use openai_api_rs::v1::error::APIError;
use openai_api_rs::v1::message::{
    CreateMessageRequest, ListMessage, MessageObject, MessageRole,
};
use openai_api_rs::v1::run::{CreateRunRequest, RunObject};
use openai_api_rs::v1::thread::{CreateThreadRequest, ThreadObject};
use std::future::Future;
use tonic::Status;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use openai_grpc_proto::chat::AssistantResponse;

pub struct AssistantService {
    pub props: ServiceProperties,
}

pub async fn wrap_tonic_status<T>(
    future: impl Future<Output = Result<T, APIError>>,
) -> Result<T, Status> {
    match future.await {
        Ok(res) => Ok(res),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

impl OpenAIService for AssistantService {
    fn properties(&self) -> &ServiceProperties {
        &self.props
    }
}

impl AssistantService {
    pub fn new(client_service: ClientService) -> Self {
        Self {
            props: ServiceProperties::new(client_service),
        }
    }

    async fn create_assistant(&mut self, instructions: String) -> Result<AssistantObject, Status> {
        let req = AssistantRequest::new(TEXT_GPT_MODEL.to_string());
        let req = req.clone().instructions(instructions);

        wrap_tonic_status(self.props.client.create_assistant(req)).await
    }

    async fn create_thread(&mut self) -> Result<ThreadObject, Status> {
        let thread_req = CreateThreadRequest::new();
        wrap_tonic_status(self.props.client.create_thread(thread_req)).await
    }

    async fn create_message(
        &mut self,
        thread_id: String,
        role: MessageRole,
        message: String,
    ) -> Result<MessageObject, Status> {
        let message_req = CreateMessageRequest::new(role, message);
        let message_result =
            wrap_tonic_status(self.props.client.create_message(thread_id, message_req)).await;
        message_result
    }

    async fn create_run(
        &mut self,
        assistant_id: String,
        thread_id: String,
    ) -> Result<RunObject, Status> {
        let run_req = CreateRunRequest::new(assistant_id);
        wrap_tonic_status(self.props.client.create_run(thread_id, run_req)).await
    }

    async fn retrieve_run(
        &mut self,
        thread_id: String,
        run_id: String,
    ) -> Result<RunObject, Status> {
        wrap_tonic_status(self.props.client.retrieve_run(thread_id, run_id)).await
    }

    async fn list_messages(&mut self, thread_id: String) -> Result<ListMessage, Status> {
        wrap_tonic_status(self.props.client.list_messages(thread_id)).await
    }

    pub async fn get_text(
        &mut self,
        message: &str,
        assistant_id: Option<String>,
        thread_id: Option<String>,
        instructions: Option<String>,
    ) -> Result<(String, String), Status> {
        let thread_id = if let Some(id) = thread_id {
            id
        } else {
            self.create_thread().await?.id
        };

        let message = self
            .create_message(thread_id.clone(), MessageRole::user, message.to_string())
            .await?;

        let assistant_id = if let Some(id) = assistant_id {
            id
        } else {
            self.create_assistant(instructions.unwrap_or("Be yourself".to_string()))
                .await?
                .id
        };

        let run = self
            .create_run(
                assistant_id,
                message.id,
            )
            .await?;

        loop {
            let run = self
                .retrieve_run(thread_id.clone(), run.id.clone())
                .await?;
            if run.status == "completed" {
                break;
            } else {
                println!("waiting...");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        let list_messages = self.list_messages(thread_id.clone()).await?;
        let message: String = if let Some(msg) = list_messages.data.last() {
            let mut answer = String::new();
            for content in msg.content.iter() {
                answer.push_str(content.text.value.as_str());
            }
            answer
        } else {
            String::from("")
        };

        Ok((message, thread_id))
    }

    pub async fn stream_assistant(
        &self,
        message: &str,
        assistant_id: Option<String>,
        thread_id: Option<String>,
        instructions: Option<String>,
    ) -> Result<Pin<Box<ReceiverStream<Result<AssistantResponse, Status>>>>, Status> {
        // Создаем поток для отправки сообщений
        let (tx, rx) = mpsc::channel(4);
        
        // Клонируем данные для асинхронной задачи
        let message = message.to_string();
        let assistant_id = assistant_id;
        let thread_id = thread_id;
        let instructions = instructions;
        let client_service = self.props.client_service.clone();
        
        tokio::spawn(async move {
            // Создаем новый экземпляр сервиса для асинхронной задачи
            let mut assistant_service = AssistantService::new(client_service);
            
            // Выполняем обычную операцию с ассистентом
            match assistant_service.get_text(
                &message,
                assistant_id,
                thread_id,
                instructions,
            ).await {
                Ok((answer, thread_id)) => {
                    // Отправляем финальный ответ
                    let response = AssistantResponse {
                        message: answer,
                        thread_id: thread_id,
                    };
                    
                    if let Err(_) = tx.send(Ok(response)).await {
                        // Канал закрыт, игнорируем ошибку
                    }
                }
                Err(e) => {
                    // Отправляем ошибку
                    if let Err(_) = tx.send(Err(e)).await {
                        // Канал закрыт, игнорируем ошибку
                    }
                }
            }
        });

        Ok(Box::pin(ReceiverStream::new(rx)))
    }
}
