mod proto {
    include!(concat!(env!("OUT_DIR"), "/config.v1.rs"));
}

fn main() {
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
