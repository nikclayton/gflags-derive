use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default)]
struct Config {
    /// Path to configuration file to load
    config_file: String,

    log: log::Config,
    pwgen: pwgen::Config,
}

fn main() {
    let args: HashSet<&'static str> = HashSet::from_iter(gflags::parse().iter().cloned());
    if args.contains("help") {
        gflags::print_help_and_exit(0);
    }

    let c = Config::default();

    c.log.init();
    c.pwgen.init();

    println!("Suggested password: {}", c.pwgen.generate());
}
