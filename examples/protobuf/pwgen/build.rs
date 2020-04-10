fn main() {
    prost_build::compile_protos(&["proto/config/v1/config.proto"], &["proto"]).unwrap();
}
