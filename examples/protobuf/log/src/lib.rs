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
        let to_stderr = pb.to_stderr;

        let to_stderr_level = if pb.to_stderr_level == proto::Level::Unspecified as i32 {
            proto::Level::Info
        } else {
            proto::Level::from_i32(pb.to_stderr_level).expect("Invalid value for level")
        };

        let dir = pb.dir;

        Self {
            to_stderr,
            to_stderr_level,
            dir,
        }
    }
}

impl Config {
    pub fn init(&self) {
        println!("Log module initialized");
    }
}
