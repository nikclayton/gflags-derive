fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.extern_path(".log.config.v1", "::log::proto");
    prost_build.extern_path(".pwgen.config.v1", "::pwgen::proto");
    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto", "../"])
        .unwrap();
}
