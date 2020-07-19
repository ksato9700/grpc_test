use apis::helloworld::HelloRequest;
use apis::helloworld_grpc::GreeterClient;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name("Ken".to_string());
    req.set_ver(123);
    let rep = client.say_hello(&req).expect("RPC Failed!");
    println!("greeter client received: {}", rep.message);
}
