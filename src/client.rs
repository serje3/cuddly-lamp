use openai_api_rs::v1::api::OpenAIClient;
use reqwest::{Client, RequestBuilder};
use tonic::Status;

pub struct ClientService {
    token: String,
}

impl ClientService {
    pub fn new() -> Self {
        Self {
            token: std::env::var("OPENAI_API_KEY").expect("No OpenAI_API_KEY env var!"),
        }
    }

    pub fn create(&self) -> Result<OpenAIClient, Status> {
        match OpenAIClient::builder().with_api_key(&self.token).build() {
            Ok(client) => Ok(client),
            Err(_) => Err(Status::internal("ошибка не получилось")),
        }
    }

    pub fn request_authenticate(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.bearer_auth(self.token.as_str())
    }
}
