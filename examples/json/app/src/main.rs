#[derive(Clone, Debug, Default)]
struct Config {
    /// Path to configuration file to load
    config_file: String,

    log: log::Config,
    pwgen: pwgen::Config,
}

fn main() {
    let c = Config::default();

    c.log.init();
    c.pwgen.init();

    println!("Suggested password: {}", c.pwgen.generate());
}
