mod svc;
mod client;
mod services;
mod config;

use openai_grpc_proto::chat::chat_server::ChatServer;
use tonic::transport::Server;
use log::info;
use crate::svc::chat::ChatService;


const BASE_GPT_URL: &str = "https://api.deepseek.com";
const TEXT_GPT_MODEL: &str = "gpt-4.1-mini";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::init()?;
    env_logger::init();
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:50051".to_string());

    info!("starting openai grpc server");
    println!("grpc server listening on {}", addr);

    let chat_svc: ChatServer<ChatService> = ChatServer::new(ChatService {});

    Server::builder()
        .add_service(chat_svc)
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
