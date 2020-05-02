# Release process

## Check for format and clippy errors

```
cargo fmt && cargo clippy
```

## Update `README.md`:

> *Note*: Requires `cargo install cargo-readme`.

```
cargo readme > README.md
```

## Bump version numbers

- `Cargo.toml`, `version` field.
- `src/lib.rs`, `html_root_url` entry.

## Final test run

```
cargo test
```

## Dry-run publishing

```
cargo publish --dry-run
```

## Publish

```
cargo publish
```