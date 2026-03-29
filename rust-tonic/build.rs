fn main() {
    let proto_dir = if std::path::Path::new("../proto").exists() {
        "../proto"
    } else {
        "proto"
    };
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                &format!("{}/helloworld.proto", proto_dir),
                &format!("{}/blood_type.proto", proto_dir),
            ],
            &[proto_dir],
        )
        .unwrap();
}
