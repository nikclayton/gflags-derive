extern crate gflags_derive;
use gflags;
use gflags_derive::GFlags;

mod common;
use common::*;

#[test]
fn derive_with_skip() {
    #[derive(GFlags)]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        to_stderr: bool,

        /// The directory to write log files to
        #[gflags(skip)]
        dir: String,
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

    check_flag::<bool>(None, flags.remove("dir"));
}
