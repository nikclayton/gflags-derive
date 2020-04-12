use anyhow::Result;
use gflags_derive::GFlags;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, GFlags)]
struct Config {
    /// Path to configuration file to load
    config_file: String,

    #[gflags(skip)]
    log: log::Config,

    #[gflags(skip)]
    pwgen: pwgen::Config,
}

fn main() -> Result<()> {
    let args: HashSet<&'static str> = HashSet::from_iter(gflags::parse().iter().cloned());
    if args.contains("help") {
        gflags::print_help_and_exit(0);
    }

    let mut c = Config::default();

    c.log = log::new(c.log)?;
    c.pwgen = pwgen::new(c.pwgen)?;

    println!("Suggested password: {}", c.pwgen.generate());

    Ok(())
}
