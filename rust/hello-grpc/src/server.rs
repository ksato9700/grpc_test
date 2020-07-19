use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use apis::helloworld::{HelloReply, HelloRequest};
use apis::helloworld_grpc::{self, Greeter};
use futures::channel::oneshot;
use futures::executor::block_on;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let mut rep = HelloReply::new();
        rep.set_message(format!("Hello {}!", req.name));

        let f = sink
            .success(rep.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| println!("Responded with Reply {{ {:?} }}", rep));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = helloworld_grpc::create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for (ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
