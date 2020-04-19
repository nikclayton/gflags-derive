extern crate gflags_derive;
use gflags_derive::GFlags;

#[derive(GFlags)]
#[allow(dead_code)]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    #[gflags(skip=1)]
    dir: String,
}

fn main() {}