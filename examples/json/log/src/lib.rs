#[derive(Clone, Copy, Debug)]
pub enum Level {
    Fatal,
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Default for Level {
    fn default() -> Self {
        Level::Info
    }
}

#[derive(Clone, Default, Debug)]
pub struct Config {
    // Whether to log to STDERR
    to_stderr: bool,

    // If logging to STDERR, what level to log at
    to_stderr_level: Level,

    // The directory to log to
    dir: String,
}

impl Config {
    pub fn init(&self) {
        println!("Log module initialized");
    }
}
