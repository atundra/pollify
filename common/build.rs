fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");

    tonic_build::configure()
        .out_dir("codegen/grpc")
        .build_client(false)
        .compile(
            &["../proto/helloworld.proto", "../proto/poll_service.proto"],
            &["../proto"],
        )?;
    Ok(())
}
