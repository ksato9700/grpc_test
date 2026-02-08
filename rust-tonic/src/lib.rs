use tonic::{Request, Response, Status};
use tracing::info;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::Greeter;
use hello_world::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let remote_addr = request.remote_addr();
        let req = request.into_inner();

        info!("Got a request from {:?}", remote_addr);
        info!("request={:?}", req);

        if req.name == "Donald" {
            return Err(Status::invalid_argument(format!(
                "Ouch! I don't like you, {}",
                req.name
            )));
        }

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", req.name),
        };
        Ok(Response::new(reply))
    }
}

#[cfg(test)]
mod tests {
    use crate::hello_world::greeter_client::GreeterClient;
    use crate::hello_world::greeter_server::{Greeter, GreeterServer};
    use crate::hello_world::{BloodType, HelloRequest};
    use crate::MyGreeter;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::transport::Server;
    use tonic::Request;

    #[tokio::test]
    async fn unit_test_say_hello() {
        let greeter = MyGreeter::default();
        let request = Request::new(HelloRequest {
            name: "Unit Test".into(),
            ver: 1,
            blood_type: BloodType::O as i32,
            extra: None,
        });

        let response = greeter.say_hello(request).await.unwrap();
        assert_eq!(response.into_inner().message, "Hello Unit Test!");
    }

    #[tokio::test]
    async fn unit_test_say_hello_donald() {
        let greeter = MyGreeter::default();
        let request = Request::new(HelloRequest {
            name: "Donald".into(),
            ver: 1,
            blood_type: BloodType::O as i32,
            extra: None,
        });

        let response = greeter.say_hello(request).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_greeter_integration() {
        let greeter = MyGreeter::default();
        let server = GreeterServer::new(greeter);

        let listener = TcpListener::bind("[::1]:0").await.unwrap();
        let addr: SocketAddr = listener.local_addr().unwrap();
        let stream = TcpListenerStream::new(listener);

        tokio::spawn(async move {
            Server::builder()
                .add_service(server)
                .serve_with_incoming(stream)
                .await
                .unwrap();
        });

        let mut client = GreeterClient::connect(format!("http://{}", addr))
            .await
            .unwrap();

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
