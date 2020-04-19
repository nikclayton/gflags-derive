extern crate gflags_derive;
use gflags;
use gflags_derive::GFlags;

mod common;
use common::*;

#[test]
fn derive_with_multiple_comments() {
    #[derive(GFlags)]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        /// Multiple lines of comments are supported
        to_stderr: bool,
    }

    let mut flags = fetch_flags();

    check_flag(
        Some(ExpectedFlag::<bool> {
            doc: &[
                "True if log messages should also be sent to STDERR",
                "Multiple lines of comments are supported",
            ],
            name: "to-stderr",
            placeholder: None,
            generated_flag: &TO_STDERR,
        }),
        flags.remove("to-stderr"),
    );
}
