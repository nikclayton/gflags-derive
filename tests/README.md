# Tests

## Documentation tests

There are no documentation tests, they don't play nicely with procedural
macros.

## `Derive` tests

The `derive_*.rs` tests each check a specific aspect of the derivation. They
need to be separate files (rather than multiple tests per file) because each
crate has a single `gflags::inventory`, so they would overwrite each other.

Each test has the same basic structure:

- Define a `struct` and derive various `gflags` options
- Fetch the defined flags, using `common::fetch_flags()`
- Create a `common::ExpectedFlag` and compare the expectation against the
  reality using `common::check_flag()`
  
## Compile tests

As a procedural macro various user mistakes may trigger compile-time errors.

The pairs of files in `expected_failures/` are a `.rs` file that should trigger
a particular compilation failure, and the `STDERR` of the error messages from
the compiler.

`expected_failures.rs` in this directory drives those tests.