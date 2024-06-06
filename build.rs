fn main() {
    let proto_file = "proto/hello.proto";

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("proto/hello_descriptor.bin")
        .compile(&[proto_file], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
