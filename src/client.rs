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
    let mut client = GreeterClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    let mut message_client = MessageServiceClient::connect("http://0.0.0.0:50051").await?;

    let send_request = tonic::Request::new(SendMessageRequest {
        content: "Hello, world!".to_string(),
    });

    let send_response = message_client.send_message(send_request).await?;

    println!("SEND_RESPONSE={:?}", send_response);

    Ok(())
}
