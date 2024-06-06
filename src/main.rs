use std::fs::read;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;

use env_logger;
use log::{debug, info};

pub mod hello {
    tonic::include_proto!("hello");
}

use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloRequest, HelloResponse,
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
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    info!("Starting gRPC server on {}", addr);
    // Load the encoded file descriptor set from the file system.
    // let encoded_file_descriptor_set = read("proto/hello_descriptor.bin").unwrap();
    let encoded_file_descriptor_set = read("/usr/local/bin/proto/hello_descriptor.bin").unwrap();
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(&encoded_file_descriptor_set)
        .build()
        .unwrap();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
