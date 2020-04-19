extern crate gflags_derive;
use gflags;
use gflags_derive::GFlags;

mod common;
use common::*;

#[test]
fn derive_with_option() {
    #[derive(GFlags)]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        to_stderr: Option<bool>,

        /// The directory to write log files to
        dir: Option<String>,
    }

    let mut flags = fetch_flags();

    // `Option<bool>` should have been converted to `bool`
    check_flag(
        Some(ExpectedFlag::<bool> {
            doc: &["True if log messages should also be sent to STDERR"],
            name: "to-stderr",
            placeholder: None,
            generated_flag: &TO_STDERR,
        }),
        flags.remove("to-stderr"),
    );

    // `Option<String>` should have been converted to `&str`
    check_flag(
        Some(ExpectedFlag::<&str> {
            doc: &["The directory to write log files to"],
            name: "dir",
            placeholder: None,
            generated_flag: &DIR,
        }),
        flags.remove("dir"),
    );
}
