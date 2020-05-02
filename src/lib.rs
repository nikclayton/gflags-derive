//! Derive command line arguments from `struct` fields using
//! [`gflags`][gflags].
//!
//! This is an alternative to the "Defining flags" section of the
//! [`gflags`][gflags] manual.
//!
//! [gflags]: https://docs.rs/gflags
//!
//! # Defining flags
//!
//! Create a struct to contain the configuration data for your library or
//! binary.
//!
//! For example, this hypothetical logging library that defines two
//! configuration options.
//!
//! ```ignore
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     dir: String,
//! }
//! ```
//!
//! Flags are added to the registry by deriving `gflags_derive::Gflags` on the
//! struct.
//!
//! ```ignore
//! use gflags_derive::GFlags;
//!
//! #[derive(GFlags)]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     dir: String,
//! }
//! ```
//!
//! You now have two new flags, as if you had written:
//!
//! ```ignore
//! gflags::define! {
//!     /// True if log messages should also be sent to STDERR
//!     --to_stderr: bool
//! }
//!
//! gflags::define! {
//!     /// The directory to write log files to
//!     --dir: &str
//! }
//! ```
//!
//! Note that:
//!
//! - The comment on each struct field is also the documentation comment for
//!   the flag, which becomes its help text.
//! - The type for the `--dir` flag has been converted from `String` to `&str`.
//!
//! # Defining a flag prefix
//!
//! You might want all the flag names to have the same prefix, without needing
//! to use that prefix on the field names. For example, a logging module might
//! want all the flags to start `log-` or `log_`.
//!
//! To support this, use the `#[gflags(prefix = "...")]` attribute on the
//! struct.
//!
//! ```ignore
//! use gflags_derive::GFlags;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log_")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     dir: String,
//! }
//! ```
//!
//! The flag definitions now include the prefix, as if you had written:
//!
//! ```ignore
//! gflags::define! {
//!     /// True if log messages should also be sent to STDERR
//!     --log_to_stderr: bool
//! }
//!
//! gflags::define! {
//!     /// The directory to write log files to
//!     --log_dir: &str
//! }
//! ```
//!
//! If the flag prefix ends with `-` then the macro converts the flag names to
//! kebab-case instead of snake_case. So writing:
//!
//! ```ignore
//! use gflags_derive::GFlags;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log-")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     dir: String,
//! }
//! ```
//!
//! generates the following flags:
//!
//! ```ignore
//! gflags::define! {
//!     /// True if log messages should also be sent to STDERR
//!     --log-to-stderr: bool
//! }
//!
//! gflags::define! {
//!     /// The directory to write log files to
//!     --log-dir: &str
//! }
//! ```
//!
//! # Handling `Option<T>`
//!
//! Your configuration `struct` may have fields that have `Option<T>` types.
//! For these fields `gflags_derive` creates a flag of the inner type `T`.
//!
//! # Customising the default value
//!
//! To specify a default value for the flag add a `#[gflags(default = ...)]`
//! attribute to the field.
//!
//! The value for the attribute is the literal value, not a quoted value.
//! Only quote the value if the type of the field is a string or can be
//! created from a string.
//!
//! For example, to set the default value of the `--log-to-stderr` flag to
//! `true`:
//!
//! ```ignore
//! use gflags_derive::GFlags;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log-")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     #[gflags(default = true)]
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     dir: String,
//! }
//! ```
//!
//! Specifying this with quotes, `#[gflags(default = "true")]` will give a
//! compile time error:
//!
//! ```text
//! expected `bool`, found `&str`
//! ```
//!
//! > **Important**: This does *not* change the default value when an instance
//! of the `Config` struct is created. It only changes the default value of
//! the `LOG_TO_STDERR.flag` variable.
//!
//! # Customising the type
//!
//! To use a different type for the field and the command line flag add a
//! `#[gflags(type = "...")]` attribute to the field.  For example, to store
//! the log directory as a `PathBuf` but accept a string on the command line:
//!
//! ```ignore
//! use gflags_derive::GFlags;
//! use std::path::PathBuf;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log-")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     #[gflags(type = "&str")]
//!     dir: PathBuf,
//! }
//! ```
//!
//! # Customising the visibility
//!
//! To use a different visibility for the flags add a
//! `#[gflags(visibility = "...")]` attribute to the field and give a Rust
//! visibility specifier.
//!
//! In this example the `LOG_DIR` flag variable will be visible in the parent
//! module.
//!
//! ```ignore
//! use gflags_derive::GFlags;
//! use std::path::PathBuf;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log-")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     #[gflags(visibility = "pub(super)")]
//!     #[gflags(type = "&str")]
//!     dir: PathBuf,
//! }
//! ```
//!
//! # Specifying a placeholder
//!
//! To give a placeholder that will appear in the flag's `help` output add a
//! `#[gflags(placeholder = "...")]` attribute to the field. This will be
//! wrapped in `<...>` for display.
//!
//! ```ignore
//! use gflags_derive::GFlags;
//! use std::path::PathBuf;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log-")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     #[gflags(placeholder = "DIR")]
//!     #[gflags(type = "&str")]
//!     dir: PathBuf,
//! }
//! ```
//!
//! In the help output the `--log-dir` flag will appear as:
//!
//! ```text
//! --log-dir <DIR>
//!         The directory to write log files to
//! ```
//!
//! # Skipping flags
//!
//! To skip flag generation for a field add a `#[gflags(skip)]` attribute to
//! the field.
//!
//! ```ignore
//! use gflags_derive::GFlags;
//! use std::path::PathBuf;
//!
//! #[derive(GFlags)]
//! #[gflags(prefix = "log-")]
//! struct Config {
//!     /// True if log messages should also be sent to STDERR
//!     to_stderr: bool,
//!
//!     /// The directory to write log files to
//!     #[gflags(skip)]
//!     dir: PathBuf,
//! }
//! ```
//!
//! No `--log-dir` flag will be generated.
//!
//! # Providing multiple attributes
//!
//! If you want to provide multiple attributes on a field then you can mix
//! and match specifing multiple options in a single `#[gflags(...)]` attribute
//! and specifying multiple `#[gflags(...)]` attributes. The following examples
//! are identical.
//!
//! ```ignore
//! ...
//!     /// The directory to write log files to
//!     #[gflags(type = "&str", visibility = "pub(super)")]
//!     dir: PathBuf,
//! ...
//! ```
//!
//! ```ignore
//! ...
//!     /// The directory to write log files to
//!     #[gflags(type = "&str")]
//!     #[gflags(visibility = "pub(super)")]
//!     dir: PathBuf,
//! ...
//! ```
//!
//! # Deserializing and merging flags
//!
//! This supports a powerful pattern for configuring an application that is
//! composed of multiple crates, where each crate exports a configuration and
//! supports multiple flags, and the application crate defines a configuration
//! that imports the configuration structs from the component crates.
//!
//! This master configuration can be deserialized from e.g. a JSON file, and
//! then each component crate can have the opportunity to override the loaded
//! configuration with information from the command line flags that are specific
//! to that crate.
//!
//! See the `examples/json` directory for a complete application that does
//! this.
//!
//! # Use with `prost`
//!
//! This macro can be used to derive flags for `structs` generated from
//! Protobuffer schemas using `prost` and `prost-build`.
//!
//! Given this `.proto` file
//!
//! ```proto
//! syntax = "proto3"
//!
//! package log.config.v1;
//!
//! message Config {
//!     // True if log messages should also be sent to STDERR
//!     bool to_stderr = 1;
//!
//!     // The directory to write log files to
//!     string dir = 2;
//! }
//! ```
//!
//! This `build.rs` file will add the relevant attributes to add the `log-`
//! prefix and skip the `dir` field.
//!
//! ```ignore
//! fn main() {
//!     let mut config = prost_build::Config::new();
//!
//!     config.type_attribute(".log.config.v1.Config", "#[derive(gflags_derive::GFlags)]");
//!     config.type_attribute(".log.config.v1.Config", "#[gflags(prefix=\"log-\")]");
//!
//!     config.field_attribute(".log.config.v1.Config.dir", "#[gflags(skip)]");
//!
//!     config
//!         .compile_protos(&["proto/log/config/v1/config.proto"], &["proto"])
//!         .unwrap();
//! }
//! ```
//!
//! See the `examples/protobuf` directory for a complete application that
//! does this.

extern crate proc_macro;

use crate::FlagCase::{KebabCase, SnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{
    punctuated::Punctuated, Attribute, Data, DataStruct, Field, Fields, FieldsNamed,
    GenericArgument, Lit, Meta, NestedMeta, Path, PathArguments, PathSegment, Token, Type,
};

#[derive(Debug, PartialEq)]
enum FlagCase {
    SnakeCase,
    KebabCase,
}

#[derive(Debug)]
struct Config {
    /// Prefix to apply to flag names
    prefix: String,

    flag_case: FlagCase,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: "".to_string(),
            flag_case: KebabCase,
        }
    }
}

fn impl_gflags_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let fields: Vec<&Field> = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: fields, .. }),
            ..
        }) => fields.into_iter().collect(),
        _ => abort_call_site!("expected a struct with named fields"),
    };

    let config = config_from_attributes(&ast.attrs);

    let mut flags: Vec<TokenStream> = vec![];

    for field in fields {
        let flag = flag_from_field(&config, field);
        flags.push(flag);
    }

    let gen = quote! {
        #(#flags)*
    };

    gen.into()
}

/// Represents a `#[gflags(...)]` attribute on a struct or field.
#[derive(Debug, Default)]
struct GFlagsAttribute {
    /// True if this field should be skipped (do not generate a flag for it)
    skip: bool,

    /// Prefix to apply to this flag (or global)
    prefix: Option<String>,

    /// Casing for this flag
    flag_case: Option<FlagCase>,

    /// Tokens that define the type to use for this flag
    ty: Option<TokenStream>,

    /// Visibility for the flag
    visibility: Option<TokenStream>,

    /// Placeholder to display in the help
    placeholder: Option<TokenStream>,

    /// Default value if the flag is not set
    default: Option<TokenStream>,
}

impl From<Meta> for GFlagsAttribute {
    fn from(meta: Meta) -> Self {
        let meta = match meta {
            Meta::List(meta) => meta,
            _ => abort!(meta, "`#[gflags(...)]` expects a parameter list"),
        };

        if meta.nested.is_empty() {
            abort!(meta, "`#[gflags(...)]` expects a non-empty parameter list");
        }

        let mut config = GFlagsAttribute::default();

        let keywords: HashSet<&'static str> = [
            "default",
            "placeholder",
            "prefix",
            "skip",
            "type",
            "visibility",
        ]
        .iter()
        .cloned()
        .collect();

        for kv in meta.nested {
            let kv = match kv {
                NestedMeta::Meta(Meta::Path(path)) => {
                    let keyword = path.get_ident().expect("No ident found");
                    if !keywords.contains(&keyword.to_string().as_ref()) {
                        abort!(path, "Invalid keyword `{}`", keyword);
                    }

                    if path.is_ident("skip") {
                        config.skip = true;
                        break;
                    }

                    abort!(path, "Keyword `{}` requires a value", keyword);
                }
                NestedMeta::Meta(Meta::NameValue(kv)) => kv,
                _ => abort!(kv, "`#[gflags(...)]` expects key=value pairs"),
            };

            if kv.path.is_ident("default") {
                let lit = kv.lit;
                config.default = Some(quote! { = #lit });
                continue;
            }

            if kv.path.is_ident("placeholder") {
                config.placeholder = match kv.lit {
                    Lit::Str(lit) => {
                        if lit.value().is_empty() {
                            abort!(
                                lit,
                                "`#[gflags(placeholder=...)]` expects a non-empty quoted string"
                            )
                        }
                        let tokens = lit.parse::<TokenStream>().unwrap();
                        Some(quote! { < #tokens > })
                    }
                    _ => abort!(
                        kv.lit,
                        "`#[gflags(placeholder=...)]` expects a quoted string"
                    ),
                };
                continue;
            }

            if kv.path.is_ident("prefix") {
                let mut prefix = match kv.lit {
                    Lit::Str(lit) => {
                        if lit.value().is_empty() {
                            abort!(
                                lit,
                                "`#[gflags(prefix=...)]` expects a non-empty quoted string"
                            );
                        }

                        lit.value()
                    }
                    _ => abort!(kv.lit, "`#[gflags(prefix=...)]` expects a quoted string"),
                };

                if prefix.ends_with('_') {
                    config.flag_case = Some(SnakeCase);
                    prefix.pop();
                }

                if prefix.ends_with('-') {
                    config.flag_case = Some(KebabCase);
                    prefix.pop();
                }

                config.prefix = Some(prefix);
                continue;
            }

            if kv.path.is_ident("skip") {
                abort!(kv.lit, "`#[gflags(skip)]` does not take a value");
            }

            if kv.path.is_ident("type") {
                config.ty = match kv.lit {
                    Lit::Str(lit) => {
                        if lit.value().is_empty() {
                            abort!(
                                lit,
                                "`#[gflags(type=...)]` expects a non-empty quoted string"
                            );
                        }

                        Some(lit.parse().unwrap())
                    }
                    _ => abort!(kv.lit, "`#[gflags(type=...)]` expects a quoted string"),
                };

                continue;
            }

            if kv.path.is_ident("visibility") {
                config.visibility = match kv.lit {
                    Lit::Str(lit) => {
                        if lit.value().is_empty() {
                            abort!(
                                lit,
                                "`#[gflags(visibility=...)]` expects a non-empty quoted string"
                            )
                        }
                        Some(lit.parse().unwrap())
                    }
                    _ => abort!(
                        kv.lit,
                        "`#[gflags(visibility=...)]` expects a quoted string"
                    ),
                };
                continue;
            }

            abort!(
                kv.path,
                "Invalid keyword `{}`",
                kv.path.get_ident().unwrap()
            );
        }

        config
    }
}

impl From<&[Attribute]> for GFlagsAttribute {
    fn from(attrs: &[Attribute]) -> Self {
        let mut config: Self = Default::default();
        for attr in attrs {
            match attr.parse_meta() {
                Ok(meta) => {
                    if !meta.path().is_ident("gflags") {
                        continue;
                    }
                    let parsed_config = GFlagsAttribute::from(meta);

                    // Any results in the parsed config overwrite any existing values.
                    // This allows multiple #[gflags(...)] attributes to exist on
                    // a single field
                    if parsed_config.skip {
                        config.skip = true
                    };

                    if parsed_config.default.is_some() {
                        config.default = parsed_config.default;
                    }

                    if parsed_config.placeholder.is_some() {
                        config.placeholder = parsed_config.placeholder;
                    }

                    if parsed_config.prefix.is_some() {
                        config.prefix = parsed_config.prefix;
                    }

                    if parsed_config.flag_case.is_some() {
                        config.flag_case = parsed_config.flag_case;
                    }

                    if parsed_config.ty.is_some() {
                        config.ty = parsed_config.ty;
                    }

                    if parsed_config.visibility.is_some() {
                        config.visibility = parsed_config.visibility;
                    }
                }
                Err(e) => abort!(attr, e),
            }
        }

        config
    }
}

/// Generate a configuration based on `#[gflags(...)` attribute values
fn config_from_attributes(attrs: &[Attribute]) -> Config {
    let mut config: Config = Default::default();

    let gfa = GFlagsAttribute::from(attrs);

    if gfa.prefix.is_some() {
        config.prefix = gfa.prefix.unwrap();
    }

    if gfa.flag_case.is_some() {
        config.flag_case = gfa.flag_case.unwrap();
    }

    config
}

fn flag_from_field(config: &Config, field: &Field) -> TokenStream {
    let gfa = GFlagsAttribute::from(field.attrs.as_ref());
    if gfa.skip {
        return TokenStream::new();
    }

    // Figure out the flag name
    let flag_name = if config.flag_case == SnakeCase {
        let ident = if !config.prefix.is_empty() {
            format_ident!(
                "{}_{}",
                config.prefix,
                field
                    .ident
                    .as_ref()
                    .expect("Unwrapping field.ident (prefix) failed")
            )
        } else {
            field
                .ident
                .as_ref()
                .expect("Unwrapping field.ident (no-prefix) failed")
                .clone()
        };
        quote! {--#ident}
    } else {
        let span = Span::call_site();
        let mut segments: Punctuated<Ident, Token![-]> = Punctuated::new();
        if !config.prefix.is_empty() {
            segments.push(Ident::new(&config.prefix, span));
        }

        let field = field.ident.as_ref().unwrap().to_string();
        for part in field.split('_') {
            segments.push(Ident::new(part, span));
        }
        quote! {--#segments}
    };

    // Figure out the default value
    let default = match gfa.default {
        Some(default) => default,
        _ => TokenStream::new(),
    };

    // Figure out the placeholder
    let placeholder = match gfa.placeholder {
        Some(placeholder) => placeholder,
        _ => TokenStream::new(),
    };

    // Figure out the visibility
    let visibility = match gfa.visibility {
        Some(visibility) => visibility,
        _ => TokenStream::new(),
    };

    // Figure out the type
    let ty = match gfa.ty {
        Some(ty) => ty,
        _ => match &field.ty {
            Type::Path(ty) => {
                let mut last = ty.path.segments.last().unwrap();
                let mut ident = &last.ident;

                let mut final_type = ty.clone();

                // Replace `Option<T>` with `T` before proceeding
                if *ident == "Option" {
                    let option_type = syn::Type::from(final_type);

                    let new_ty = extract_type_from_option(&option_type);
                    match new_ty {
                        Some(Type::Path(new_ty)) => {
                            final_type = new_ty.clone();
                            last = final_type.path.segments.last().unwrap();
                            ident = &last.ident;
                        }
                        _ => abort!(&field.ty, "Unexpected type"),
                    }
                }

                if *ident == "String" {
                    quote! { &str }
                } else {
                    quote! { #final_type }
                }
            }
            _ => abort!(&field.ty, "Unexpected type"),
        },
    };

    // Figure out the doc string, if there is one
    let mut docs: Vec<Literal> = vec![];

    for attr in &field.attrs {
        if !attr.path.is_ident("doc") {
            continue;
        }
        let tokens = attr.tokens.clone();
        for token in tokens {
            if let TokenTree::Literal(l) = token {
                docs.push(l);
            }
        }
    }

    // Construct the macro call
    let gen = quote! {
        gflags::define! {
            #( #[doc = #docs])*
            #visibility #flag_name #placeholder: #ty #default
        }
    };

    gen
}

/// Given a `syn::Type` that is an `Option<T>`, return the `syn::Type` for the
/// `T`, or `None` if it's not a `syn::Type::Path`.
///
/// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|pair_path_segment| {
            let type_params = &pair_path_segment.arguments;
            // It should have only one angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

/// # Struct level attributes
///
/// `#[gflags(prefix = "...")]` -- apply this prefix to flag names
///
/// # Field level attributes
///
/// `#[gflags(default = ...)]` -- default value for this flag
///
/// `#[gflags(placeholder= "...")]` -- placeholder to display in help
///
/// `#[gflags(skip)]` -- do not generate a flag for this field
///
/// `#[gflags(type = "...")]` -- generate a flag with this type
///
/// `#[gflags(visibility = "...")]` -- generate a flag with this visibility
///
/// Refer to the [crate level documentation](index.html) for a complete example.
#[proc_macro_derive(GFlags, attributes(gflags))]
#[proc_macro_error]
pub fn gflags_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gflags_macro(&ast)
}
