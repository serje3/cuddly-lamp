use crate::client::ClientService;
use openai_api_rs::v1::api::OpenAIClient;


pub struct ServiceProperties {
    pub client_service: ClientService,
    pub client: OpenAIClient,
}

impl ServiceProperties {
    pub(crate) fn new(client_service: ClientService) -> Self {
        ServiceProperties {
            client: client_service.create().unwrap(),
            client_service: client_service,
        }
    }
}

pub trait OpenAIService {
    fn properties(&self) -> &ServiceProperties;
}
