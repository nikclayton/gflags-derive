use anyhow::Result;
use gflags::custom::{Arg, Error, Value};
use gflags_derive::GFlags;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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

impl Value for Level {
    fn parse(arg: Arg) -> gflags::custom::Result<Self> {
        match arg.get_str().to_ascii_lowercase().as_ref() {
            "fatal" => Ok(Level::Fatal),
            "critical" => Ok(Level::Critical),
            "error" => Ok(Level::Error),
            "warning" => Ok(Level::Warning),
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            "trace" => Ok(Level::Trace),
            _ => Err(Error::new("invalid logging level")),
        }
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, GFlags)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
#[gflags(prefix = "log-")]
pub struct Config {
    /// Whether to log to STDERR
    to_stderr: bool,

    /// If logging to STDERR, what level to log at
    to_stderr_level: Level,

    /// The directory to log to
    dir: String,
}

pub fn new(config: Config) -> Result<Config> {
    let mut config = config;

    if LOG_TO_STDERR.is_present() {
        config.to_stderr = LOG_TO_STDERR.flag;
    }

    if LOG_TO_STDERR_LEVEL.is_present() {
        config.to_stderr_level = LOG_TO_STDERR_LEVEL.flag;
    }

    if LOG_DIR.is_present() {
        config.dir = LOG_DIR.flag.to_string();
    }

    Ok(config)
}
