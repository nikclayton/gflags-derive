fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.extern_path(".log.config.v1", "::log::proto");
    prost_build.extern_path(".pwgen.config.v1", "::pwgen::proto");

    prost_build.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
    prost_build.type_attribute(".", "#[serde(rename_all = \"kebab-case\")]");
    prost_build.type_attribute(".", "#[serde(default)]");

    prost_build.type_attribute(".config.v1.Config", "#[derive(gflags_derive::GFlags)]");
    prost_build.field_attribute(
        ".config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".config.v1.Config.config_file",
        "#[gflags(type=\"&std::path::Path\")]",
    );
    prost_build.field_attribute(".config.v1.Config.log", "#[gflags(skip)]");
    prost_build.field_attribute(".config.v1.Config.pwgen", "#[gflags(skip)]");
    prost_build.field_attribute(".config.v1.Config.pwgen", "#[serde(rename=\"pw\")]");
    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto", "../"])
        .unwrap();
}
