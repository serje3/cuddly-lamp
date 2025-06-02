use openai_api_rs::v1::api::OpenAIClient;
use crate::client::ClientService;

pub struct ServiceProperties {
    pub(crate) client: OpenAIClient,
    pub(crate) client_service: ClientService
}

pub trait OpenAIService {
    fn properties(&mut self) -> &ServiceProperties;
}