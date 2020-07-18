extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/protos";
    protoc_grpcio::compile_grpc_protos(
        &["hello/helloworld.proto"],
        &[proto_root],
        &proto_root,
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
