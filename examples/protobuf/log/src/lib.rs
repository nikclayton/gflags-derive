use std::str::FromStr;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/log.config.v1.rs"));
}

#[derive(Clone, Default, Debug)]
pub struct Config {
    to_stderr: bool,
    to_stderr_level: proto::Level,
    dir: String,
}

impl From<proto::Config> for Config {
    fn from(pb: proto::Config) -> Self {
        let to_stderr = if proto::LOG_TO_STDERR.is_present() {
            proto::LOG_TO_STDERR.flag
        } else {
            pb.to_stderr
        };

        let to_stderr_level = if proto::LOG_TO_STDERR_LEVEL.is_present() {
            proto::Level::from_str(proto::LOG_TO_STDERR_LEVEL.flag).expect("Invalid level")
        } else if proto::Level::is_valid(pb.to_stderr_level) {
            if pb.to_stderr_level == proto::Level::Unspecified as i32 {
                proto::Level::Info
            } else {
                proto::Level::from_i32(pb.to_stderr_level).unwrap()
            }
        } else {
            proto::Level::Info
        };

        let dir = if proto::LOG_DIR.is_present() {
            proto::LOG_DIR.flag.to_string()
        } else {
            pb.dir
        };

        Self {
            to_stderr,
            to_stderr_level,
            dir,
        }
    }
}
