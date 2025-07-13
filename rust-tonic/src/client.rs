use hello_grpc_tonic_lib::hello_world::greeter_client::GreeterClient;
use hello_grpc_tonic_lib::hello_world::BloodType;
use hello_grpc_tonic_lib::hello_world::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Ken".into(),
        ver: 123,
        blood_type: BloodType::B as i32,
        extra: None,
    });
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(HelloRequest {
        name: "Donald".into(),
        ver: 321,
        blood_type: BloodType::Ab as i32,
        extra: None,
    });
    client.say_hello(request).await?;

    Ok(())
}
