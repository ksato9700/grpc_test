fn main() {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::compile_protos("proto/helloworld.proto").unwrap();
}
