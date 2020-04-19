extern crate gflags_derive;
use gflags;
use gflags_derive::GFlags;

mod common;
use common::*;

#[test]
fn derive_with_prefix_snakecase() {
    #[derive(GFlags)]
    #[gflags(prefix = "log_")]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        to_stderr: bool,

        /// The directory to write log files to
        dir: String,
    }

    let mut flags = fetch_flags();

    check_flag(
        Some(ExpectedFlag::<bool> {
            doc: &["True if log messages should also be sent to STDERR"],
            name: "log_to_stderr",
            placeholder: None,
            generated_flag: &LOG_TO_STDERR,
        }),
        flags.remove("log_to_stderr"),
    );

    check_flag(
        Some(ExpectedFlag::<&str> {
            doc: &["The directory to write log files to"],
            name: "log_dir",
            placeholder: None,
            generated_flag: &LOG_DIR,
        }),
        flags.remove("log_dir"),
    );
}
