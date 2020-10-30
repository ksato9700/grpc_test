extern crate protoc_grpcio;

fn main() {
    protoc_grpcio::compile_grpc_protos(
        &["helloworld.proto", "blood_type.proto"],
        &["../proto"],
        &"src/helloworld",
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
