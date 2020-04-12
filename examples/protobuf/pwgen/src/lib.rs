use rand::Rng;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/pwgen.config.v1.rs"));
}

#[derive(Clone, Debug)]
pub struct Config {
    charset: String,
    length: u32,
}

impl From<proto::Config> for Config {
    fn from(pb: proto::Config) -> Self {
        let charset = if proto::PW_CHARSET.is_present() {
            proto::PW_CHARSET.flag.to_string()
        } else if pb.charset.is_empty() {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()
        } else {
            pb.charset
        };

        let length = if proto::PW_LENGTH.is_present() {
            proto::PW_LENGTH.flag
        } else if pb.length == 0 {
            10
        } else {
            pb.length
        };

        Self { charset, length }
    }
}

impl Config {
    /// Generate a terrible password
    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();

        (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0, self.charset.len());
                self.charset
                    .chars()
                    .nth(idx)
                    .expect("Unexpected missing character")
            })
            .collect()
    }
}
