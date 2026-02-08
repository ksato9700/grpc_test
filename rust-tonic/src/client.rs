use clap::Parser;
use hello_grpc_tonic_lib::hello_world::greeter_client::GreeterClient;
use hello_grpc_tonic_lib::hello_world::BloodType;
use hello_grpc_tonic_lib::hello_world::HelloRequest;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://[::1]:50051")]
    addr: String,

    /// Names to greet. If the first argument contains ':', it's treated as the address.
    #[arg(trailing_var_arg = true)]
    names: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    // Heuristic: if the first positional arg looks like an address, use it.
    if !args.names.is_empty() && args.names[0].contains(':') {
        args.addr = if args.names[0].starts_with("http") {
            args.names[0].clone()
        } else {
            format!("http://{}", args.names[0])
        };
        args.names.remove(0);
    }

    if args.names.is_empty() {
        args.names.push("Rust".to_string());
    }

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Connecting to {}", args.addr);
    let mut client = GreeterClient::connect(args.addr).await?;

    for name in args.names {
        let request = tonic::Request::new(HelloRequest {
            name: name.clone(),
            ver: 123,
            blood_type: BloodType::B as i32,
            extra: None,
        });
        match client.say_hello(request).await {
            Ok(response) => {
                info!("Greeting for {}: {}", name, response.into_inner().message);
            }
            Err(e) => {
                info!("RPC failed for {}: {}", name, e);
            }
        }
    }

    Ok(())
}
