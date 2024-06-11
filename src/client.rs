use hello::{greeter_client::GreeterClient, HelloRequest};
use messages::{message_service_client::MessageServiceClient, SendMessageRequest};

pub mod hello {
    tonic::include_proto!("hello");
}
pub mod messages {
    tonic::include_proto!("messages");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://grpc.cristallum.io";
    let url = "http://localhost:8080";
    let mut client = GreeterClient::connect(url).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    let mut message_client = MessageServiceClient::connect(url).await?;

    let send_request = tonic::Request::new(SendMessageRequest {
        content: "Hello, world!".to_string(),
    });

    let send_response = message_client.send_message(send_request).await?;

    println!("SEND_RESPONSE={:?}", send_response);

    Ok(())
}
