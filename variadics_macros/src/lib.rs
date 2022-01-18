#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]

mod variadic;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{Expr, Type};
use variadic::AmbigItem;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ParseContext {
    Expr,
    Type,
}

fn get_crate_path(span: Span) -> Ident {
    let found_crate = crate_name("variadics").expect("variadics should be present in `Cargo.toml`");
    match found_crate {
        FoundCrate::Itself => Ident::new("crate", span),
        FoundCrate::Name(name) => Ident::new(&name, span),
    }
}

#[proc_macro]
pub fn ignore(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn variadic(input: TokenStream) -> TokenStream {
    variadic::variadic::<AmbigItem>(input, Some(ParseContext::Expr))
}

#[proc_macro]
pub fn variadic_expr(input: TokenStream) -> TokenStream {
    variadic::variadic::<Expr>(input, Some(ParseContext::Expr))
}
#[proc_macro]
pub fn variadic_type(input: TokenStream) -> TokenStream {
    variadic::variadic::<Type>(input, Some(ParseContext::Type))
}
