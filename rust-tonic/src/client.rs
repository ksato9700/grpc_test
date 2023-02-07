use hello_world::greeter_client::GreeterClient;
use hello_world::BloodType;
use hello_world::HelloRequest;

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

    let request = tonic::Request::new(HelloRequest {
        name: "Donald".into(),
        ver: 321,
        blood_type: BloodType::Ab as i32,
    });
    client.say_hello(request).await?;

    Ok(())
}
