use std::collections::HashSet;
use std::iter::FromIterator;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/config.v1.rs"));
}

fn main() {
    let args: HashSet<&'static str> = HashSet::from_iter(gflags::parse().iter().cloned());
    if args.contains("help") {
        gflags::print_help_and_exit(0);
    }

    let c = proto::Config {
        log: Some(log::proto::Config::default()),
        pwgen: Some(pwgen::proto::Config::default()),
        ..Default::default()
    };

    let log = log::Config::from(c.log.unwrap());
    let pwgen = pwgen::Config::from(c.pwgen.unwrap());

    log.init();
    pwgen.init();

    println!("Suggested password: {}", pwgen.generate());
}
