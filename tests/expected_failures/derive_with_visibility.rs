extern crate gflags_derive;

// Derive the flags in an inner module. The generated `TO_STDERR` variable
// should not be visible outside this module, the `DIR` variable should be.
//
// This is one half of this test -- verifying that `TO_STDERR` is not visible
// in the `super` crate.
//
// The second half is in `../derive_with_visibility.rs` to verify that trying
// to access `DIR` works.
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

fn main() {
    if inner_for_test::TO_STDERR.is_present() {
        panic!("Can't happen, flag is not visible in this scope");
    }
}
