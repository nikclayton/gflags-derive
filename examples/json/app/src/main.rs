use anyhow::Result;
use gflags_derive::GFlags;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;

#[derive(Clone, Debug, Default, Deserialize, Serialize, GFlags)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
struct Config {
    /// Path to configuration file to load
    config_file: String,

    /// Print additional debug information
    debug: bool,

    #[gflags(skip)]
    log: log::Config,

    #[gflags(skip)]
    #[serde(rename = "pw")]
    pwgen: pwgen::Config,
}

impl Config {
    fn new() -> Result<Self> {
        let mut config = if CONFIG_FILE.is_present() {
            read_config_from_file(CONFIG_FILE.flag)?
        } else {
            Config::default()
        };

        if DEBUG.is_present() {
            config.debug = DEBUG.flag;
        }

        if config.debug {
            println!("Loaded config:\n{}", serde_json::to_string_pretty(&config)?);
        }

        config.log = log::new(config.log).expect("Error parsing log flags");
        config.pwgen = pwgen::new(config.pwgen).expect("Error parsing pw flags");

        if config.debug {
            println!(
                "Config after command line parsing:\n{}",
                serde_json::to_string_pretty(&config)?
            );
        }

        Ok(config)
    }
}

fn main() -> Result<()> {
    let args: HashSet<&'static str> = HashSet::from_iter(gflags::parse().iter().cloned());
    if args.contains("help") {
        gflags::print_help_and_exit(0);
    }

    let c = Config::new().expect("Config did not parse");

    println!("Suggested password: {}", c.pwgen.generate());
    Ok(())
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c = serde_json::from_reader(reader)?;

    Ok(c)
}
