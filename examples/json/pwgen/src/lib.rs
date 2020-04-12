use gflags_derive::GFlags;
use rand::Rng;

#[derive(Clone, Debug, GFlags)]
#[gflags(prefix = "pw-")]
pub struct Config {
    /// String to use for password characters
    charset: String,

    /// Desired password length
    length: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            charset: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            length: 10,
        }
    }
}

impl Config {
    pub fn init(&self) {
        println!("PWGen module initialised");
    }

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
