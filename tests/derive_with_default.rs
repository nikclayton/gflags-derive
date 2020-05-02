extern crate gflags_derive;
use gflags;
use gflags_derive::GFlags;

mod common;
use common::*;
use std::path::PathBuf;

#[test]
fn derive_with_default() {
    #[derive(GFlags)]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        #[gflags(default = true)]
        to_stderr: bool,

        /// The directory to write log files to
        #[gflags(type = "&str")]
        #[gflags(default = "/tmp")]
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

    assert_eq!(
        TO_STDERR.flag, true,
        "TO_STDERR default value should be `true`"
    );

    check_flag(
        Some(ExpectedFlag::<&str> {
            doc: &["The directory to write log files to"],
            name: "dir",
            placeholder: None,
            generated_flag: &DIR,
        }),
        flags.remove("dir"),
    );

    assert_eq!(DIR.flag, "/tmp", "DIR default value should be `/tmp`");
}
