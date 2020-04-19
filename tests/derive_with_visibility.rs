extern crate gflags_derive;

mod common;
use common::*;

// Derive the flags in an inner module. The generated `TO_STDERR` variable
// should not be visible outside this module, the `DIR` variable should be.
//
// This is one half of this test -- verifying that `DIR` is visible in the
// `super` crate.
//
// The second half is in `expected_failures/derive_with_visibility.rs` to
// verify that trying to access `TO_STDERR` generates a compile-time error.
mod inner_for_test {
    use gflags_derive::GFlags;

    #[derive(GFlags)]
    #[allow(dead_code)]
    struct Config {
        /// True if log messages should also be sent to STDERR
        to_stderr: bool,

        /// The directory to write log files to
        #[gflags(visibility = "pub(super)")]
        dir: String,
    }
}

#[test]
fn derive_with_visibility() {
    let mut flags = fetch_flags();

    check_flag(
        Some(ExpectedFlag::<&str> {
            doc: &["The directory to write log files to"],
            name: "dir",
            placeholder: None,
            generated_flag: &inner_for_test::DIR,
        }),
        flags.remove("dir"),
    );
}
