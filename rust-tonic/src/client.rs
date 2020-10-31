use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use hello_world::BloodType;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Ken".into(),
        ver: 123,
        blood_type: BloodType::B as i32,
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
