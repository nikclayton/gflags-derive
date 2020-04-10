# Protobuf for configuration

This example uses protocol buffers to define the configuration schema. There
are several benefits to this approach:

- Other systems can easily introspect the schema and types
- Systems like protobuf make it easier to clearly separate internal and
  external interfaces
- Systems like protobuf make evolving those interfaces over time easier

This comes at the cost of slightly more complicated code.

Compared to the example in the `json` directory, the main change is that each
module now has two different configuration `structs`.

The first, defined by the `.proto` file, represents the public configuration
data.

The second is a regular Rust `struct` which represents a private copy of that
configuration data.

This means there must be a mechanism to convert from the public configuration
to the private. This is provided by implementing the `From` trait to convert
from `proto::Config` to `Config` in each module.

This also provides an opportunity to include default values for missing
configuration entries, as well as do any error checking on the configuration,
in the same way that the `Default` implementations in the equivalent modules
in the `json` example do.

This makes the initialisation in `app/main.rs` slightly more complicated, as
each value in the configuration is optional, and there is the additional
step of calling `...::from(...);` to retrieve opaque config type.