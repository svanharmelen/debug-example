fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .cargo_out_dir("protos")
        .include("src")
        .input("src/example.proto")
        .run_from_script();
}
