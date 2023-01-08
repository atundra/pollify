fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");

    tonic_build::configure()
        .out_dir("codegen/grpc")
        .compile(&["../proto/helloworld.proto"], &["../proto"])?;
    Ok(())
}
