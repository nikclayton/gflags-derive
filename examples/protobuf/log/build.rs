fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.type_attribute(".log.config.v1.Config", "#[derive(gflags_derive::GFlags)]");
    prost_build.type_attribute(".log.config.v1.Config", "#[gflags(prefix=\"log-\")]");

    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto"])
        .unwrap();
}
