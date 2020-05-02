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
        /// The directory to write log files to
        #[gflags(type = "&str")]
        #[gflags(placeholder = "DIR")]
        dir: PathBuf,
    }

    let mut flags = fetch_flags();

    // The flag should be an `&str` not a `PathBuf`
    check_flag(
        Some(ExpectedFlag::<&str> {
            doc: &["The directory to write log files to"],
            name: "dir",
            placeholder: Some("DIR"),
            generated_flag: &DIR,
        }),
        flags.remove("dir"),
    );
}
