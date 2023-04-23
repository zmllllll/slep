fn main() -> Result<(), Box<dyn std::error::Error>> {
    let core_service = tonic_build::manual::Service::builder()
        .name("Core")
        .package("json")
        .method(
            tonic_build::manual::Method::builder()
                .name("command")
                .route_name("Command")
                .input_type("crate::resources::Resources")
                .output_type("crate::resources::Resources")
                .codec_path("crate::rpc::coder::JsonCodec")
                .build(),
        )
        .method(
            tonic_build::manual::Method::builder()
                .name("core_stream")
                .route_name("CoreStream")
                .server_streaming()
                .input_type("crate::rpc::Login")
                .output_type("crate::resources::Resources")
                .codec_path("crate::rpc::coder::JsonCodec")
                .build(),
        )
        .build();

    tonic_build::manual::Builder::new()
        .out_dir("../json.core")
        .compile(&[core_service]);

    Ok(())
}
