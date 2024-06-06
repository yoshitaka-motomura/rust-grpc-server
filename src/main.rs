use chrono::Utc;
use env_logger;
use log::{debug, info};
use std::env;
use std::fs::read;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;

pub mod hello {
    tonic::include_proto!("hello");
}
pub mod messages {
    tonic::include_proto!("messages");
}

use hello::{
    greeter_server::{Greeter, GreeterServer},
    GoodbyeRequest, HelloRequest, HelloResponse,
};

use messages::{
    message_service_server::{MessageService, MessageServiceServer},
    MessageRequest, MessageResponse, SendMessageRequest,
};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let reply = hello::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
            success: true,
        };
        Ok(Response::new(reply))
    }
    async fn goodbye(
        &self,
        request: Request<GoodbyeRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        debug!("Request: {:?}", request);

        let reply = hello::HelloResponse {
            message: format!("Request Message {}!", request.into_inner().message).into(),
            success: true,
        };
        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct MyMessageService {}

#[tonic::async_trait]
impl MessageService for MyMessageService {
    async fn get_message(
        &self,
        request: Request<MessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        println!("Got a message request: {:?}", request);

        let reply = MessageResponse {
            content: format!("request by id {}", request.into_inner().id),
            timestamp: Utc::now().timestamp(),
        };
        Ok(Response::new(reply))
    }
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        println!("Got a send message request: {:?}", request);

        let reply = MessageResponse {
            content: format!("{}", request.into_inner().content),
            timestamp: Utc::now().timestamp(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    //TODO: Temporary code for reading the descriptor file.
    let descripptor_file_path = env::var("DESCRIPTOR_FILE_PATH")
        .unwrap_or_else(|_| "/usr/local/bin/proto/descriptor.bin".to_string());
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();
    let message_service = MyMessageService::default();

    info!("Starting gRPC server on {}", addr);

    let encoded_file_descriptor_set = read(descripptor_file_path).unwrap();
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(&encoded_file_descriptor_set)
        .build()
        .unwrap();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(MessageServiceServer::new(message_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
