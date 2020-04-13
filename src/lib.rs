extern crate proc_macro;

use crate::FlagCase::{KebabCase, SnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
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

        let mut config = Self {
            skip: false,
            prefix: None,
            flag_case: None,
            ty: None,
            visibility: None,
        };

        for kv in meta.nested {
            let kv = match kv {
                NestedMeta::Meta(Meta::Path(path)) => {
                    if path.is_ident("skip") {
                        config.skip = true;
                        break;
                    }

                    abort!(path, "Unexpected key");
                }
                NestedMeta::Meta(Meta::NameValue(kv)) => kv,
                _ => abort!(kv, "`#[gflags(...)]` expects key=value pairs"),
            };
            if kv.path.is_ident("prefix") {
                let mut prefix = match kv.lit {
                    Lit::Str(lit) => lit.value(),
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

            if kv.path.is_ident("type") {
                config.ty = match kv.lit {
                    Lit::Str(lit) => Some(lit.parse().unwrap()),
                    _ => abort!(kv.lit, "`#[gflags(type=...)]` expects a quoted string"),
                };

                continue;
            }

            if kv.path.is_ident("visibility") {
                config.visibility = match kv.lit {
                    Lit::Str(lit) => Some(lit.parse().unwrap()),
                    _ => abort!(
                        kv.lit,
                        "`#[gflags(visibility=...)]` expects a quoted string"
                    ),
                };
                continue;
            }

            abort!(kv.path, "Unknown key `{}`", kv.path.get_ident().unwrap());
        }

        config
    }
}

impl From<&[Attribute]> for GFlagsAttribute {
    fn from(attrs: &[Attribute]) -> Self {
        let mut config: Self = Default::default();
        for attr in attrs {
            let meta = attr.parse_meta().unwrap();
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
            #visibility #flag_name: #ty
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

#[proc_macro_derive(GFlags, attributes(gflags))]
#[proc_macro_error]
pub fn gflags_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gflags_macro(&ast)
}
