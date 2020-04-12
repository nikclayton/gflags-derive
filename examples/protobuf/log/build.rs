fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.type_attribute(".log.config.v1.Config", "#[derive(gflags_derive::GFlags)]");
    prost_build.type_attribute(".log.config.v1.Config", "#[gflags(prefix=\"log-\")]");

    prost_build.type_attribute(
        ".log.config.v1.Level",
        "#[derive(strum_macros::EnumString, strum_macros::Display)]",
    );
    prost_build.field_attribute(
        ".log.config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".log.config.v1.Config.to_stderr_level",
        "#[gflags(type = \"&str\")]",
    );

    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto"])
        .unwrap();
}
