extern crate proc_macro;

use crate::FlagCase::{KebabCase, SnakeCase};
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, Attribute, Data, DataStruct, Field, Fields, FieldsNamed, Lit, Meta,
    NestedMeta, Token, Type,
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
    /// Prefix to apply to this flag (or global)
    prefix: Option<String>,

    /// Casing for this flag
    flag_case: Option<FlagCase>,

    /// Tokens that define the type to use for this flag
    ty: Option<TokenStream>,
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
            prefix: None,
            flag_case: None,
            ty: None,
        };

        for kv in meta.nested {
            let kv = match kv {
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
            config = GFlagsAttribute::from(meta);
            break;
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

    // Figure out the flag name
    let flag_name = if config.flag_case == SnakeCase {
        let ident = format_ident!("{}_{}", config.prefix, field.ident.as_ref().unwrap());
        quote! {--#ident}
    } else {
        let span = Span::call_site();
        let mut segments: Punctuated<Ident, Token![-]> = Punctuated::new();
        segments.push(Ident::new(&config.prefix, span));

        let field = field.ident.as_ref().unwrap().to_string();
        for part in field.split('_') {
            segments.push(Ident::new(part, span));
        }
        quote! {--#segments}
    };

    // Figure out the type
    let ty = match gfa.ty {
        Some(ty) => ty,
        _ => match &field.ty {
            Type::Path(ty) => {
                let last = ty.path.segments.last().unwrap();
                let ident = last.ident.clone();
                let string = last.ident.to_string();

                println!("last segment: {}", string);
                match string.as_ref() {
                    "String" => quote! { &str },
                    _ => quote! { #ident },
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
            #flag_name: #ty
        }
    };

    gen
}

#[proc_macro_derive(GFlags, attributes(gflags))]
#[proc_macro_error]
pub fn gflags_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gflags_macro(&ast)
}
