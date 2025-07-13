use std::env;
use tonic::transport::Server;

use hello_grpc_tonic_lib::hello_world::greeter_server::GreeterServer;
use hello_grpc_tonic_lib::MyGreeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: String = env::var("PORT").unwrap_or("50051".to_string());
    let addr = format!("[::0]:{}", port).parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
