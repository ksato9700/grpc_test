
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use tonic::{transport::Server, Request, Response, Status};
    use crate::tests::hello_world::greeter_client::GreeterClient;
    use crate::tests::hello_world::{HelloRequest, BloodType, HelloReply};
    use crate::tests::hello_world::greeter_server::{Greeter, GreeterServer};

    pub mod hello_world {
        tonic::include_proto!("helloworld");
    }

    #[derive(Default)]
    pub struct MyGreeter {}

    #[tonic::async_trait]
    impl Greeter for MyGreeter {
        async fn say_hello(
            &self,
            request: Request<HelloRequest>,
        ) -> Result<Response<HelloReply>, Status> {
            println!("Got a request from {:?}", request.remote_addr());
            println!("request={:?}", request);
            println!("request.extra={:?}", request.get_ref().extra);

            let reply = hello_world::HelloReply {
                message: format!("Hello {}!", request.into_inner().name),
            };
            Ok(Response::new(reply))
        }
    }

    #[tokio::test]
    async fn test_greeter() {
        let addr = "[::1]:50051".parse().unwrap();
        let greeter = MyGreeter::default();

        let server = Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr);

        tokio::spawn(server);
        sleep(Duration::from_secs(1)).await;

        let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
            ver: 1,
            blood_type: BloodType::A as i32,
            extra: None,
        });

        let response = client.say_hello(request).await.unwrap();

        assert_eq!(response.into_inner().message, "Hello Tonic!");
    }
}
