use chrono::Utc;
use env_logger;
use log::{debug, info};
use tonic::{transport::Server, Request, Response, Status};
use tonic_health::server::health_reporter;

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

    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();
    let message_service = MyMessageService::default();
    let (mut health_reporter, health_service) = health_reporter();
    health_reporter.set_serving::<GreeterServer<MyGreeter>>().await;
    health_reporter.set_serving::<MessageServiceServer<MyMessageService>>().await;

    info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(GreeterServer::new(greeter))
        .add_service(MessageServiceServer::new(message_service))
        .serve(addr)
        .await?;

    Ok(())
}
