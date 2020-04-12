fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");

    // If you want the official Protobuf field naming then swap the comments on
    // the next two lines.
    //prost_build.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
    prost_build.type_attribute(".", "#[serde(rename_all = \"kebab-case\")]");

    prost_build.type_attribute(".log.config.v1.Config", "#[derive(gflags_derive::GFlags)]");
    prost_build.type_attribute(".log.config.v1.Config", "#[gflags(prefix=\"log-\")]");

    prost_build.field_attribute(
        ".log.config.v1.Config.to_stderr_level",
        "#[serde(with=\"super::serde_level\")]",
    );
    prost_build.field_attribute(
        ".log.config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".log.config.v1.Config.to_stderr_level",
        "#[gflags(type=\"&str\")]",
    );

    prost_build.type_attribute(
        ".log.config.v1.Level",
        "#[derive(strum_macros::EnumString, strum_macros::Display)]",
    );
    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto"])
        .unwrap();
}
