use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use clap::Parser;
use hello_grpc_tonic_lib::hello_world::greeter_server::GreeterServer;
use hello_grpc_tonic_lib::MyGreeter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 50051)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(args.port);

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let addr = format!("[::0]:{}", port).parse()?;
    let greeter = MyGreeter::default();

    info!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
