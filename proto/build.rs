extern crate protoc_grpcio;

fn main() {
    let proto_root = ".";
    println!("cargo:rerun-if-changed={}", "*.proto");
    protoc_grpcio::compile_grpc_protos(
        &["data.proto"], &[""], &"src", None)
        .expect("Failed to compile gRPC definitions!");
}
