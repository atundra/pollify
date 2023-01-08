fn main() {
    println!("cargo:rerun-if-changed=../proto/helloworld.proto");

    rust_grpc_web::configure()
        .support_streaming(false)
        .out_dir("src/codegen")
        .compile(&["../proto/helloworld.proto"], &["../proto"])
        .unwrap();
}
