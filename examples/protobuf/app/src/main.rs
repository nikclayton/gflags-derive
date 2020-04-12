use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

mod proto {
    include!(concat!(env!("OUT_DIR"), "/config.v1.rs"));
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Config {
    config_file: PathBuf,
    debug: bool,
    log: log::Config,
    #[serde(rename = "pw")]
    pwgen: pwgen::Config,
}

impl From<&proto::Config> for Config {
    fn from(pb: &proto::Config) -> Self {
        let config_file = if proto::CONFIG_FILE.is_present() {
            PathBuf::from(proto::CONFIG_FILE.flag)
        } else {
            PathBuf::from("")
        };

        let debug = if proto::DEBUG.is_present() {
            proto::DEBUG.flag
        } else {
            pb.debug
        };

        let log = if pb.log.is_some() {
            log::Config::from(&pb.log.clone().unwrap())
        } else {
            log::Config::from(&log::proto::Config::default())
        };

        let pwgen = if pb.pwgen.is_some() {
            pwgen::Config::from(&pb.pwgen.clone().unwrap())
        } else {
            pwgen::Config::from(&pwgen::proto::Config::default())
        };

        Self {
            config_file,
            debug,
            log,
            pwgen,
        }
    }
}

fn main() -> Result<()> {
    let args: HashSet<&'static str> = HashSet::from_iter(gflags::parse().iter().cloned());
    if args.contains("help") {
        gflags::print_help_and_exit(0);
    }

    let config_pb: proto::Config = if proto::CONFIG_FILE.is_present() {
        read_config_from_file(proto::CONFIG_FILE.flag)?
    } else {
        proto::Config {
            log: Some(log::proto::Config::default()),
            pwgen: Some(pwgen::proto::Config::default()),
            ..Default::default()
        }
    };

    if proto::DEBUG.is_present() && proto::DEBUG.flag {
        println!(
            "Loaded config:\n{}",
            serde_json::to_string_pretty(&config_pb)?
        );
    }

    let config = Config::from(&config_pb);

    if config.debug {
        println!(
            "Config after command line parsing:\n{}",
            serde_json::to_string_pretty(&config)?
        );
    }

    println!("Suggested password: {}", config.pwgen.generate());
    Ok(())
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<proto::Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let c = serde_json::from_reader(reader)?;

    Ok(c)
}
