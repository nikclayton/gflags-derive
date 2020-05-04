# Examples

The examples in the `json` and `protobuf` directories implement the same
application, and take slightly different routes to specifying the external
configuration for the application.

Each application follows the same general structure.
 
The `json` example is simpler, so this documentation walks through the
`json` example first, and then notes the differences in the `protobuf`
section.

## JSON

The sample application is a password generator run from the command line.

If run (all commands run from the `json` directory) it prints a password:

```shell script
% cargo run
Suggested password: BEPELYJFWP
```

You can specify flags to control the password character set and length:

```shell script
% cargo run -- --pw-length 5 --pw-charset ABCDE
Suggested password: DCCDD
```

It supports other flags, which can be seen with the `help` command.

```shell script
% cargo run -- help
    --config-file
            Path to configuration file to load

    --debug
            Print additional debug information

    --log-dir
            The directory to log to

    --log-to-stderr
            Whether to log to STDERR

    --log-to-stderr-level
            If logging to STDERR, what level to log at

    --pw-charset
            String to use for password characters

    --pw-length
            Desired password length
```

And it can be given a configuration file specifying these options, where
they can be overriden from the command line.

In this config file a subset of options are specified, including the password
length:

```shell script
% cat config.json
{
  "log": {
    "to-stderr": true,
    "to-stderr-level": "Info"
  },
  "pw": {
    "length": 12
  }
}
```

But the `--pw-length` flag overrides this:

```shell script
% cargo run -- --config-file config.json --pw-length 3
Suggested password: CZA
```

The flags, help values, defaults, and JSON de/serialisation are driven from
a single struct and attributes on the struct.

### Code structure

The example consists of three crates:

- `log` - doesn't do anything, and exists to show this approach allows
  multiple crates to define independent configuration information
- `pwgen` - generates a password, has configuration data to specify the
  characters the password should be drawn from and the length of the
  password.
- `app` - has configuration information specifying the config file to
  load and whether to enable `debug` mode, and imports the
  configuration definitions of the other two crates.
  
The rest of this section looks at how the code is structured to achieve this.

### `pwgen`

The `pwgen` crate has four sections.

> Note: The code copied in these examples may not match the current version
of the code, but should be close enough to be understandable.

#### The configuration schema.

```rust
#[derive(Clone, Debug, Deserialize, Serialize, GFlags)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
#[gflags(prefix = "pw")]
pub struct Config {
    /// String to use for password characters
    charset: String,

    /// Desired password length
    length: u32,
}
```

This is a normal Rust struct with two fields to specify the character
set and the minimum password length.

Command line flags will be generated from this struct because it derives
`GFlags`. The `prefix` attribute means they will be called `--pw-charset`
and `--pw-length`.

This struct will be read from a JSON config file, so Serde is used to derive
the De/Serialize traits, and `#[serde(default)]` is used to ensure any
missing fields read from the config file take their default values.

#### Default configuration

The library should provide some default configuration, with a normal Rust
`impl Default`:

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            charset: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            length: 10,
        }
    }
}
```

#### Merging configuration with command line flags

The `new()` method takes a `Config` and returns a new `Config` with values
taken from any command line flags (if they are present).

This process could fail -- maybe the user has specified a parseable but
invalid value on the command line, such as a negative password length. This
code does not check for this (to keep the example simple) but it could.

```rust
pub fn new(config: Config) -> Result<Config> {
    let mut config = config;

    if PW_CHARSET.is_present() {
        config.charset = PW_CHARSET.flag.to_string();
    }

    if PW_LENGTH.is_present() {
        config.length = PW_LENGTH.flag;
    }

    Ok(config)
}
```

#### Implementing functionality

The module provides a `generate` method on the `Config` which actually creates
and returns the password.

```rust
impl Config {
    /// Generate a terrible password
    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();

        (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0, self.charset.len());
                self.charset
                    .chars()
                    .nth(idx)
                    .expect("Unexpected missing character")
            })
            .collect()
    }
}
```

### `log`

The `log` crate is similar to the `pwgen` crate. As mentioned it doesn't
provide any functionality, it exists to show multiple crates can co-exist
with this approach.

The code also demonstrates some features `pwgen` did not.

#### The configuration schema

```rust
#[derive(Clone, Default, Debug, Deserialize, Serialize, GFlags)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
#[gflags(prefix = "log-")]
pub struct Config {
    // Whether to log to STDERR
    to_stderr: bool,

    // If logging to STDERR, what level to log at
    to_stderr_level: Level,

    // The directory to log to
    dir: String,
}
```

This is similar to `pwgen`. The flags will be named `--log-to-stderr`, 
`--log-to-stderr-level`, and `--log-dir` respectively.

Serde's `rename_all` attribute is used so the keys in the JSON config
file will have the same kebab-case names as the command line flags.

> *Note*: This is entirely optional. If the gflags prefix was `log_` then the
flags would be snake case, `--log_to_stderr` etc, and the `rename_all` could
be removed. I just happen to prefer kebab-case.

Because this uses an enum as a field value there is some additional work to
do.

The enum must be defined:

```rust
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Level {
    Fatal,
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}
```

A default value for the enum should be provided:

```rust
impl Default for Level {
    fn default() -> Self {
        Level::Info
    }
}
```

And `gflags` requires the enum implement the `Value` trait to convert
from the string value provided on the command line to the actual enum value.

```rust
impl Value for Level {
    fn parse(arg: Arg) -> gflags::custom::Result<Self> {
        match arg.get_str().to_ascii_lowercase().as_ref() {
            "fatal" => Ok(Level::Fatal),
            "critical" => Ok(Level::Critical),
            "error" => Ok(Level::Error),
            "warning" => Ok(Level::Warning),
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            "trace" => Ok(Level::Trace),
            _ => Err(Error::new("invalid logging level")),
        }
    }
}
```

#### Merging configuration with command line flags

Just like `pwgen` a `new` method receives a `Config` and returns a `Result`
containing a new configuration with the command line flag values merged in.

Again, this is allowed to fail -- perhaps the user has specified a
non-existent directory to the `--log-dir` flag.

### `app`

The `app` crate ties everything together. It has four sections.

#### The configuration schema

```rust
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
```

The app has two configuration flags of its own, `--config-file` and `--debug`.
It imports the configuration structs of the `log` and `pwgen` crates but
they are skipped by gflags because those crates already derive the command
line flags they need.

The `pwgen` crate is renamed with Serde so the object keys in the config
file, `pw`, will match the prefix used in the command line flags. This is not
strictly necessary, but ensures consistency between the names of the flags
and the keys in the configuration file.

#### Generating a valid configuration

This is the code in the implementation of `Config::new()`.

```rust
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
```

By this point `gflags::parse()` must have been called (see the next section).
This ensures the flags `CONFIG_FILE` and `DEBUG` have been initialised
from the command line.

If `--config-file` was provided on the command line then
`CONFIG_FILE.is_present()` is true, deserialise the configuration from
the provided file. Otherwise construct a default configuration.

Once the configuration has been loaded any values can be overwritten with
command line flags.

If `--debug` was given then show the "before" state of the configuration.

Call each crate's (`log`, `pwgen`) `::new()` method with the contents of
the loaded configuration. This is the crate's opportunity to perform any
sanity checking and merge in the values from the command line flags, before
returning a final configuration.

If `--debug` was given then show the "after" state of the configuration.

Return the config to the caller.

#### Performing the application functionality

```rust
fn main() -> Result<()> {
    let args: HashSet<&'static str> = HashSet::from_iter(gflags::parse().iter().cloned());
    if args.contains("help") {
        gflags::print_help_and_exit(0);
    }

    let c = Config::new().expect("Config did not parse");

    println!("Suggested password: {}", c.pwgen.generate());
    Ok(())
}
```

`gflags::parse()` returns any unrecognised arguments which are collected
in to a hash. If the hash contains the key `help` then print the help text
and exit.

Otherwise, create the configuration from the environment (configuration
file + flags). `c.pwgen` contains a configured `pwgen::Config`, and call its
`generate()` method to generate the password.

### Benefits of this approach

- Command line flags and configuration file structure remain in lock-step.
You cannot change one without changing the other.

- Saving the current configuration to a file for future use is
straightforward. This makes it easy to share configurations demonstrating
a bug -- pasteable in to tickets, instant messages, e-mail, etc.

- Overriding values in the configuration is completely consistent across all
crates using this approach.

- Keeping crates in control of their configuration and flags ensures the way
to configure the behaviour of a crate is identical across all applications.
Once you've learned `--log-dir` is the flag to control the directory used
by the logging crate you've learned it for all applications using the crate.

- Configuration is private to the crate. The application (`app/src/main.rs`
in this example) does not need to know what configuration data each crate
needs or the command line flags it needs to expose.

This does not work if you are using arbitrary crates from multiple sources.
But it is ideal if you are building a large application -- or suite of
applications -- from a common code base consisteing of multiple crates and
want consistency in configuration and command line flags.

In my experience designing and operating large scale systems, this sort of
consistency is very helpful when moving between different systems. It means
you do not need to maintain information about inconsistencies in behaviour
in your head, reducing the chance of operational error.

## Protobuf

In the JSON example the crate's external configuration interface and
internal configuration are identical, the same `struct Config`.

This can be a problem -- if you need to refactor `Config` for any reason then
the crate's external interface has also changed -- for example, renaming
fields will make existing configuration files and command lines invalid.

So the crate's configuration should be split in two pieces.

The first is the *external* configuration, provided to the library or
application through a combination of a configuration file and / or command
line arguments.

The second is the *internal* configuration, which is the configuration data
the library or application uses when it is running.

Splitting these in two means the internal configuration structure can
change rapidly without needing to be concerned with issues of backwards
compatibility, while the external interface can evolve more slowly in a
backwards compatible manner.

When the external configuration is parsed the crate needs to marshall the
external configuration data to the internal configuration structure.

This split *could* be achieved by defining two distinct Rust structs for
the internal and external configuration and marshalling data between them.

This example goes a step further and defines the external structure through
the Protobuf IDL. This has specific support and tooling for identifying
backwards incompatible changes, and it also allows other applications to
read this program's configuration schema and generate configurations
matching the schema.

The rest of this section assumes general familiarity with the Protobuf
IDL and the [prost](https://docs.rs/prost/) crate.

### Code structure

The `example/protobuf` code follows the same structure as `example/json` so
familiarise yourself with that first.

The application behaves in the same way and takes the same command line
flags and configuration as `examples/json`.

The rest of this documentation will focus on the differences.

### `pwgen`

#### The external configuration schema

The external configuration schema definition is
`proto/config/v1/config.proto`, and looks like this.

```proto
syntax = "proto3";

package pwgen.config.v1;

message Config {
    // String to use for password characters
    string charset = 1;

    // Desired password length
    uint32 length = 2;
}
```

This must be converted to a Rust file containing a struct with similar
attributes on the struct and fields as `examples/json`. This is achieved using
`prost-build`, configured in `build.rs`. 

```rust
fn main() {
    let mut prost_build = prost_build::Config::new();

    prost_build.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
    prost_build.type_attribute(".", "#[serde(rename_all = \"kebab-case\")]");

    prost_build.type_attribute(
        ".pwgen.config.v1.Config",
        "#[derive(gflags_derive::GFlags)]",
    );
    prost_build.type_attribute(
        ".pwgen.config.v1.Config",
        "#[gflags(prefix=\"pw-\")]"
    );
    prost_build.field_attribute(
        ".pwgen.config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".pwgen.config.v1.Config.charset",
        "#[gflags(type=\"&str\")]",
    );

    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto"])
        .unwrap();
}
```

Note the addition of `#[gflags(visibility = "pub(super)")]"` to the fields.
This will be relevant in `pwgen/src/lib.rs`.

When `cargo build` is run this will generate `pwgen.config.v1.rs` (from the
`package` entry in `config.proto`) in `$OUT_DIR`.

This must be imported in to `pwgen/src/lib.rs`, and this is achieved with

```rust
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/pwgen.config.v1.rs"));
}
```

The practical upshot of this is within `pwgen/src/lib.rs` the external
`Config` struct can be referenced as `proto::Config`, and because the fields
have visibility `pub(super)` the flags generated by `gflags` can be referenced
as `proto::PW_CHARSET` and `proto::PW_LENGTH`.

#### The internal configuration schema

The internal configuration schema is a regular Rust `struct`:

```rust
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    charset: String,
    length: u32,
}
```

#### Merging configuration with command line flags

This example shows an alternative approach to merging the command line flags.
`examples/json` used a `new()` method.

Here we opt to model creating a `Config` as an operation converting *from*
a `proto::Config`, so implementing Rust's `From` trait seems appropriate.

```rust
impl From<&proto::Config> for Config {
    fn from(pb: &proto::Config) -> Self {
        let charset = if proto::PW_CHARSET.is_present() {
            proto::PW_CHARSET.flag.to_string()
        } else if pb.charset.is_empty() {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()
        } else {
            pb.charset.clone()
        };

        let length = if proto::PW_LENGTH.is_present() {
            proto::PW_LENGTH.flag
        } else if pb.length == 0 {
            10
        } else {
            pb.length
        };

        Self { charset, length }
    }
}
```

Every field in a protobuf is optional, so this has to perform a little
extra work, checking to see if a field in the `proto::Config` struct has
a non-default value before deciding whether to use it.

For conversions that can fail `TryFrom` could also be used. You could also
conceptualise the external configuration as *wrapping* the internal
configuration, in which case implementing `into_inner()` on the external
configuration and returning the internal configuration could also be
appropriate.

#### Implementing functionality

The crate's functionality is implemented identically to `examples/json`.

#### A note on naming things

In this example I opted to use `Config` as the name of the both external and
internal configurations, differentiating between them using the `proto::`
namespace, and importing the generated `pwgen.config.v1.rs` file in to a
`proto` module.

This is not required by this approach. You could use a different name for the
external configuration. For example, `ExternalConfig`:

```proto
message ExternalConfig {
   // ...
}
``` 

and import it in to `src/lib.rs` without the wrapper `proto` module.

```rust
include!(concat!(env!("OUT_DIR"), "/pwgen.config.v1.rs"));
```

This would keep everything in the same namespace (and you could drop the
addition of the `pub(super)` visibility specifiers in `build.rs`).

Both approaches are valid, and don't affect using the crate.

### `log`

#### The external configuration schema

The external configuration schema definition is 
`proto/config/v1/config.proto`, and looks like this.

```rust
syntax = "proto3";

package log.config.v1;

enum Level {
    LEVEL_UNSPECIFIED = 0;
    LEVEL_FATAL = 1;
    LEVEL_CRITICAL = 2;
    LEVEL_ERROR = 3;
    LEVEL_WARNING = 4;
    LEVEL_INFO = 5;
    LEVEL_DEBUG = 6;
    LEVEL_TRACE = 7;
}

message Config {
    // Whether to log to STDERR
    // line two of this comment
    bool to_stderr = 1;

    // If logging to STDERR, what level to log at
    Level to_stderr_level = 2;

    // The directory to log to
    string dir = 3;
}
```

This is very similar to the definition in `examples/json`. The difference
is the `Level` enum has an additional value, `LEVEL_UNSPECIFIED`.

In Protobuf messages you cannot tell the difference between a field being
unset (and therefore having its default value) or the field being explicitly
set to the default value.

Best practice with Protobuf enums is to ensure the first enum value
explicitly identifies an unspecified value, so the application can
decide what to do.

If you do not do this the default value will be whichever enum value is
defined first, which is inappropriate in this case.

As with `pwgen` this must be converted to a Rust file in `build.rs`.

```rust
fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");

    prost_build.type_attribute(".", "#[serde(rename_all = \"kebab-case\")]");

    prost_build.type_attribute(
        ".log.config.v1.Config",
        "#[derive(gflags_derive::GFlags)]"
    );
    prost_build.type_attribute(
        ".log.config.v1.Config",
        "#[gflags(prefix=\"log-\")]"
    );

    prost_build.field_attribute(
        ".log.config.v1.Config.to_stderr_level",
        "#[serde(with=\"super::serde_level\")]",
    );
    prost_build.field_attribute(
        ".log.config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".log.config.v1.Config.to_stderr_level",
        "#[gflags(type=\"&str\")]",
    );
    prost_build.type_attribute(
        ".log.config.v1.Level",
        "#[derive(strum_macros::EnumString, strum_macros::Display)]",
    );

    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto"])
        .unwrap();
}
```

This is similar to the file for `pwgen` but needs to do some extra work to
handle the `Level` enum.

Protobuf enums are recorded as numbers, so the default de/serialisation of
them would expect to de/serialise to/from numbers.

We want to be able to de/serialise to/from strings insteasd. To support this
the [strum](https://docs.rs/strum/) crate is used to derive methods to
de/serialise the enum value to/from a string.

The code to actually do this has not been shown yet, but is in the
`serde_level` module. The `with` attribute configures Serde to use it.

As with `pwgen` building this crate will generate `log.config.v1.rs` and
this is included in `src/lib.rs` with the lines

```rust
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/log.config.v1.rs"));
}
```

#### The internal configuration schema

This replicates the structure of the external configuration:

```rust
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    to_stderr: bool,
    to_stderr_level: proto::Level,
    dir: String,
}
```

Rather than create an internal enum to represent the log level and map
between them use the `Level` enum from the proto file.

#### De/serialising

As mentioned the `Level` value must be de/serialised, the default behaviour
is to serialise it the numeric representation for the enum.

`build.rs` configured Serde to use de/serialisation code in the
`serde_level` module. This module follows:

```rust
/// Serialize/deserialize the `Level` enum value. In the `.proto` file it's
/// an enum that is represented as an i32, when round-tripping through JSON
/// use the name.
mod serde_level {
    use super::proto;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(n: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let level = proto::Level::from_i32(*n).expect("Failed to parse level");
        let string = level.to_string();
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let l = proto::Level::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(l as i32)
    }
}
```

#### Merging configuration with command line flags

As with `pwgen` this is carried out by implementing the `From` trait.

```rust
impl From<&proto::Config> for Config {
    fn from(pb: &proto::Config) -> Self {
        let to_stderr = if proto::LOG_TO_STDERR.is_present() {
            proto::LOG_TO_STDERR.flag
        } else {
            pb.to_stderr
        };

        let to_stderr_level = if proto::LOG_TO_STDERR_LEVEL.is_present() {
            proto::Level::from_str(proto::LOG_TO_STDERR_LEVEL.flag).expect("Invalid level")
        } else if proto::Level::is_valid(pb.to_stderr_level) {
            if pb.to_stderr_level == proto::Level::Unspecified as i32 {
                proto::Level::Info
            } else {
                proto::Level::from_i32(pb.to_stderr_level).unwrap()
            }
        } else {
            proto::Level::Info
        };

        let dir = if proto::LOG_DIR.is_present() {
            proto::LOG_DIR.flag.to_string()
        } else {
            pb.dir.clone()
        };

        Self {
            to_stderr,
            to_stderr_level,
            dir,
        }
    }
}
```

This marshalls data from the provided `proto::Config` overwriting it with
data from flags as appropriate.

Notice when `to_stderr_level` is defined the incoming value is checked
against `proto::Level::Unspecified` to see if it was empty, so the
default value (`proto::Level::Info`) can be used if no flag was provided. 

#### Implementing functionality

Like `example/json` this module provides no functionality, it exists to
show multiple modules using this approach can co-exist and provide
flags, and highlight the work needed when de/serialising enums.

### `app`

#### The external configuration schema

The external configuration schema is in
`proto/config/v1/config.proto` and looks like this:

```proto
syntax = "proto3";

package config.v1;

import "log/proto/config/v1/config.proto";
import "pwgen/proto/config/v1/config.proto";

message Config {
    string config_file = 1;
    bool debug = 2;

    log.config.v1.Config log = 3;
    pwgen.config.v1.Config pwgen = 4;
}
```

This has to import the specific versions of the crate configuration protos
it wants to use.

If any of these ever introduce a backwards-incompatible change they would
need to bump version number in the `package` directive in their
`config.proto` files. This means the application can decide when to
support the new configuration format.

As before this is converted to `config.v1.rs` with code in `build.rs`.

This code is very similar to code we've already seen.

```rust
n main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.extern_path(".log.config.v1", "::log::proto");
    prost_build.extern_path(".pwgen.config.v1", "::pwgen::proto");

    prost_build.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
    prost_build.type_attribute(".", "#[serde(rename_all = \"kebab-case\")]");
    prost_build.type_attribute(".", "#[serde(default)]");

    prost_build.type_attribute(
        ".config.v1.Config",
        "#[derive(gflags_derive::GFlags)]"
    );
    prost_build.field_attribute(
        ".config.v1.Config",
        "#[gflags(visibility = \"pub(super)\")]",
    );
    prost_build.field_attribute(
        ".config.v1.Config.config_file",
        "#[gflags(type=\"&std::path::Path\")]",
    );
    prost_build.field_attribute(
        ".config.v1.Config.log",
        "#[gflags(skip)]"
    );
    prost_build.field_attribute(
        ".config.v1.Config.pwgen",
        "#[gflags(skip)]"
    );
    prost_build.field_attribute(
        ".config.v1.Config.pwgen",
        "#[serde(rename=\"pw\")]"
    );

    prost_build
        .compile_protos(&["proto/config/v1/config.proto"], &["proto", "../"])
        .unwrap();
}
```

The two additions are:

1. The use of the `extern_path()` method to bring in the imported protos. This
ensures we use the code compiled in the `log` and `pwgen` crates. If
those crates implemented any custom code for this proto (right now they don't,
but they could) this ensures the custom code is included.

If we did not do this then compiling `app` would compile a new copy of
`log.config.v1.rs` and `pwgen.config.v1.rs` and any custom code would be
ignored.

2. In the call to `compile_protos()` at the bottom of the file there is an
additional include path of `../` to find the `log` and `pwgen` configuration
`.proto` files..

#### The internal configuration schema

This is simply:

```rust
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Config {
    config_file: PathBuf,
    debug: bool,
    log: log::Config,
    #[serde(rename = "pw")]
    pwgen: pwgen::Config,
}
```

> *Note*: The `pwgen` field has been renamed to `pw` for de/serialisation.
This is an unfortunate breakage of the abstraction between the `app` crate
and the `pwgen` crate (which specifies a `pw-` prefix on all flags).

#### Merging configuration with command line flags

This is also implemented using the `From` trait. First the configuration
fields the app supports directly (`config_file` and `debug`) are merged
in, and then the `::from` methods on the `log` and `pwgen` crates are used
to construct their final pieces of config incorporating the command line
flag values however they choose.

Then the final `Config` is generated and returned.

```rust
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
```

#### Implementing functionality

With this done we can tie it all together in the `main()` function.

As in `examples/json` the command line is parsed and any remaining arguments
are checked to see if the help should be shown.

Then the external configuration is generated, some debugging information is
(optionally) displayed, before handing the configuration to `Config::from()`
to derive the final configuration.

And then the password can be generated.

```rust
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
```

### Pros and cons of this approach

This approach can lead to slighly more complicated code, although it is
clearly encapsulated and only affects the configuration and command line
flag parsing -- the rest of the code is insulated from it.

It does allow you to clearly delineate between the external configuration
interface and the internal configuration data that the code needs, making
it much easier to change the internal interface without imposing a breaking
change on your users.