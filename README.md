# gflags-derive

Derive command line arguments from `struct` fields using
[`gflags`][gflags].

This is an alternative to the "Defining flags" section of the
[`gflags`][gflags] manual.

[gflags]: https://docs.rs/gflags

## Defining flags

Create a struct to contain the configuration data for your library or
binary.

For example, this hypothetical logging library that defines two
configuration options.

```rust
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    dir: String,
}
```

Flags are added to the registry by deriving `gflags_derive::Gflags` on the
struct.

```rust
use gflags_derive::GFlags;

#[derive(GFlags)]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    dir: String,
}
```

You now have two new flags, as if you had written:

```rust
gflags::define! {
    /// True if log messages should also be sent to STDERR
    --to_stderr: bool
}

gflags::define! {
    /// The directory to write log files to
    --dir: &str
}
```

Note that:

- The comment on each struct field is also the documentation comment for
  the flag, which becomes its help text.
- The type for the `--dir` flag has been converted from `String` to `&str`.

## Defining a flag prefix

You might want all the flag names to have the same prefix, without needing
to use that prefix on the field names. For example, a logging module might
want all the flags to start `log-` or `log_`.

To support this, use the `#[gflags(prefix = "...")]` attribute on the
struct.

```rust
use gflags_derive::GFlags;

#[derive(GFlags)]
#[gflags(prefix = "log_")]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    dir: String,
}
```

The flag definitions now include the prefix, as if you had written:

```rust
gflags::define! {
    /// True if log messages should also be sent to STDERR
    --log_to_stderr: bool
}

gflags::define! {
    /// The directory to write log files to
    --log_dir: &str
}
```

If the flag prefix ends with `-` then the macro converts the flag names to
kebab-case instead of snake_case. So writing:

```rust
use gflags_derive::GFlags;

#[derive(GFlags)]
#[gflags(prefix = "log-")]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    dir: String,
}
```

generates the following flags:

```rust
gflags::define! {
    /// True if log messages should also be sent to STDERR
    --log-to-stderr: bool
}

gflags::define! {
    /// The directory to write log files to
    --log-dir: &str
}
```

## Handling `Option<T>`

Your configuration `struct` may have fields that have `Option<T>` types.
For these fields `gflags_derive` creates a flag of the inner type `T`.

## Customising the default value

To specify a default value for the flag add a `#[gflags(default = ...)]`
attribute to the field.

The value for the attribute is the literal value, not a quoted value.
Only quote the value if the type of the field is a string or can be
created from a string.

For example, to set the default value of the `--log-to-stderr` flag to
`true`:

```rust
use gflags_derive::GFlags;

#[derive(GFlags)]
#[gflags(prefix = "log-")]
struct Config {
    /// True if log messages should also be sent to STDERR
    #[gflags(default = true)]
    to_stderr: bool,

    /// The directory to write log files to
    dir: String,
}
```

Specifying this with quotes, `#[gflags(default = "true")]` will give a
compile time error:

```
expected `bool`, found `&str`
```

> **Important**: This does *not* change the default value when an instance
of the `Config` struct is created. It only changes the default value of
the `LOG_TO_STDERR.flag` variable.

## Customising the type

To use a different type for the field and the command line flag add a
`#[gflags(type = "...")]` attribute to the field.  For example, to store
the log directory as a `PathBuf` but accept a string on the command line:

```rust
use gflags_derive::GFlags;
use std::path::PathBuf;

#[derive(GFlags)]
#[gflags(prefix = "log-")]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    #[gflags(type = "&str")]
    dir: PathBuf,
}
```

## Customising the visibility

To use a different visibility for the flags add a
`#[gflags(visibility = "...")]` attribute to the field and give a Rust
visibility specifier.

In this example the `LOG_DIR` flag variable will be visible in the parent
module.

```rust
use gflags_derive::GFlags;
use std::path::PathBuf;

#[derive(GFlags)]
#[gflags(prefix = "log-")]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    #[gflags(visibility = "pub(super)")]
    #[gflags(type = "&str")]
    dir: PathBuf,
}
```

## Specifying a placeholder

To give a placeholder that will appear in the flag's `help` output add a
`#[gflags(placeholder = "...")]` attribute to the field. This will be
wrapped in `<...>` for display.

```rust
use gflags_derive::GFlags;
use std::path::PathBuf;

#[derive(GFlags)]
#[gflags(prefix = "log-")]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    #[gflags(placeholder = "DIR")]
    #[gflags(type = "&str")]
    dir: PathBuf,
}
```

In the help output the `--log-dir` flag will appear as:

```
--log-dir <DIR>
        The directory to write log files to
```

## Skipping flags

To skip flag generation for a field add a `#[gflags(skip)]` attribute to
the field.

```rust
use gflags_derive::GFlags;
use std::path::PathBuf;

#[derive(GFlags)]
#[gflags(prefix = "log-")]
struct Config {
    /// True if log messages should also be sent to STDERR
    to_stderr: bool,

    /// The directory to write log files to
    #[gflags(skip)]
    dir: PathBuf,
}
```

No `--log-dir` flag will be generated.

## Providing multiple attributes

If you want to provide multiple attributes on a field then you can mix
and match specifing multiple options in a single `#[gflags(...)]` attribute
and specifying multiple `#[gflags(...)]` attributes. The following examples
are identical.

```rust
...
    /// The directory to write log files to
    #[gflags(type = "&str", visibility = "pub(super)")]
    dir: PathBuf,
...
```

```rust
...
    /// The directory to write log files to
    #[gflags(type = "&str")]
    #[gflags(visibility = "pub(super)")]
    dir: PathBuf,
...
```

## Deserializing and merging flags

This supports a powerful pattern for configuring an application that is
composed of multiple crates, where each crate exports a configuration and
supports multiple flags, and the application crate defines a configuration
that imports the configuration structs from the component crates.

This master configuration can be deserialized from e.g. a JSON file, and
then each component crate can have the opportunity to override the loaded
configuration with information from the command line flags that are specific
to that crate.

See the `examples/json` directory for a complete application that does
this.

## Use with `prost`

This macro can be used to derive flags for `structs` generated from
Protobuffer schemas using `prost` and `prost-build`.

Given this `.proto` file

```proto
syntax = "proto3"

package log.config.v1;

message Config {
    // True if log messages should also be sent to STDERR
    bool to_stderr = 1;

    // The directory to write log files to
    string dir = 2;
}
```

This `build.rs` file will add the relevant attributes to add the `log-`
prefix and skip the `dir` field.

```rust
fn main() {
    let mut config = prost_build::Config::new();

    config.type_attribute(".log.config.v1.Config", "#[derive(gflags_derive::GFlags)]");
    config.type_attribute(".log.config.v1.Config", "#[gflags(prefix=\"log-\")]");

    config.field_attribute(".log.config.v1.Config.dir", "#[gflags(skip)]");

    config
        .compile_protos(&["proto/log/config/v1/config.proto"], &["proto"])
        .unwrap();
}
```

See the `examples/protobuf` directory for a complete application that
does this.

License: MIT OR Apache-2.0
