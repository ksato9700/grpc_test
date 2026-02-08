fn main() {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["proto/helloworld.proto", "proto/blood_type.proto"], &["proto"])
        .unwrap();
}
