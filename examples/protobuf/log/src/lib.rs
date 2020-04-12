use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/log.config.v1.rs"));
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    to_stderr: bool,
    to_stderr_level: proto::Level,
    dir: String,
}

impl From<&proto::Config> for Config {
    fn from(pb: &proto::Config) -> Self {
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
            pb.dir.clone()
        };

        Self {
            to_stderr,
            to_stderr_level,
            dir,
        }
    }
}

/// Serialize/deserialize the `Level` enum value. In the `.proto` file it's
/// an enum that is represented as an i32, when round-tripping through JSON
/// use the name.
mod serde_level {
    use super::proto;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(n: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let level = proto::Level::from_i32(*n).expect("Failed to parse level");
        let string = level.to_string();
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let l = proto::Level::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(l as i32)
    }
}
