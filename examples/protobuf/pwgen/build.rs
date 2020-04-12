fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.type_attribute(
        ".pwgen.config.v1.Config",
        "#[derive(gflags_derive::GFlags)]",
    );
    prost_build.type_attribute(".pwgen.config.v1.Config", "#[gflags(prefix=\"pw-\")]");

    prost_build.field_attribute(
        ".pwgen.config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".pwgen.config.v1.Config.charset",
        "#[gflags(type=\"&str\")]",
    );

    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto"])
        .unwrap();
}
