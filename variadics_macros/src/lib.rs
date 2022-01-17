#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]

mod variadic;
mod variadic_fn;
mod variadic_trait;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{Expr, Path, PathArguments, PathSegment, Type};
use variadic::AmbigItem;

fn get_crate_path(item: Path, span: Span) -> Path {
    let found_crate = crate_name("variadics").expect("variadics should be present in `Cargo.toml`");

    let prefix = match found_crate {
        FoundCrate::Itself => Ident::new("crate", span),
        FoundCrate::Name(name) => Ident::new(&name, span),
    };
    let segment = PathSegment {
        ident: prefix,
        arguments: PathArguments::None,
    };

    let mut path = vec![segment];
    path.extend(item.segments.into_iter());
    let segments = path.into_iter().collect();

    Path {
        leading_colon: None,
        segments,
    }
}

#[proc_macro]
pub fn ignore(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn variadic_expr(input: TokenStream) -> TokenStream {
    variadic::variadic::<Expr>(input)
}
#[proc_macro]
pub fn variadic_type(input: TokenStream) -> TokenStream {
    variadic::variadic::<Type>(input)
}

#[proc_macro]
pub fn variadic(input: TokenStream) -> TokenStream {
    variadic::variadic::<AmbigItem>(input)
}

#[proc_macro]
pub fn variadic_fn(input: TokenStream) -> TokenStream {
    variadic_fn::variadic_fn(input)
}

#[proc_macro]
pub fn variadic_trait(input: TokenStream) -> TokenStream {
    variadic_trait::variadic_trait(input)
}
