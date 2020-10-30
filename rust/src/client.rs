use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;

mod helloworld;
use helloworld::blood_type::BloodType;
use helloworld::helloworld::HelloRequest;
use helloworld::helloworld_grpc::GreeterClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name("Ken".to_string());
    req.set_ver(123);
    req.set_bloodType(BloodType::B);
    match client.say_hello(&req) {
        Ok(rep) => {
            println!("greeter client received: {}", rep.message);
        }
        Err(err) => {
            println!("ERRRORR: {}", err);
        }
    };
}
