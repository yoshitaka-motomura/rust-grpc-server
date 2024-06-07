fn main() {
    let proto_files = &["proto/hello.proto", "proto/message.proto"];

    tonic_build::configure()
        .build_server(true)
        //.file_descriptor_set_path("proto/descriptor.bin")
        .compile(proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
