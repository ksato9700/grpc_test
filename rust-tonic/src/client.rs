use clap::Parser;
use hello_grpc_tonic_lib::hello_world::greeter_client::GreeterClient;
use hello_grpc_tonic_lib::hello_world::BloodType;
use hello_grpc_tonic_lib::hello_world::HelloRequest;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://[::1]:50051")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut client = GreeterClient::connect(args.addr).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Ken".into(),
        ver: 123,
        blood_type: BloodType::B as i32,
        extra: None,
    });
    let response = client.say_hello(request).await?;

    info!("RESPONSE={:?}", response);

    let request = tonic::Request::new(HelloRequest {
        name: "Donald".into(),
        ver: 321,
        blood_type: BloodType::Ab as i32,
        extra: None,
    });
    client.say_hello(request).await?;

    Ok(())
}
