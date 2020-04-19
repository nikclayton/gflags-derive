extern crate gflags_derive;
use gflags;
use gflags_derive::GFlags;

mod common;
use common::*;
use std::path::PathBuf;

#[test]
fn derive_with_custom_type() {
    #[derive(GFlags)]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        to_stderr: bool,

        /// The directory to write log files to
        #[gflags(type = "&str")]
        dir: PathBuf,
    }

    let mut flags = fetch_flags();

    check_flag(
        Some(ExpectedFlag::<bool> {
            doc: &["True if log messages should also be sent to STDERR"],
            name: "to-stderr",
            placeholder: None,
            generated_flag: &TO_STDERR,
        }),
        flags.remove("to-stderr"),
    );

    // The flag should be an `&str` not a `PathBuf`
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
