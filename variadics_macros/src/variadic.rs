use std::iter::Peekable;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Token};

use crate::ParseContext;

pub(crate) struct AmbigItem {
    token_stream: TokenStream2,
}
impl Parse for AmbigItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.step(|cursor| {
            let mut token_stream = TokenStream2::default();

            let mut depth: i32 = 0;
            let mut prev = *cursor;
            while let Some((tt, next)) = prev.token_tree() {
                if let TokenTree::Punct(punct) = &tt {
                    match punct.as_char() {
                        ',' => {
                            if depth <= 0 {
                                break;
                            }
                        }
                        '<' => {
                            depth += 1;
                        }
                        '>' => {
                            depth -= 1;
                            if depth < 0 {
                                punct.span().unwrap().error(
                                    "Expressions with `<` or `>` must be put in parenthesis `( ... )` or braces `{ ... }` to avoid ambiguity.",
                                );
                            }
                        }
                        _ => {}
                    }
                }
                token_stream.extend([tt]);

                prev = next;
            }
            Ok((Self { token_stream }, prev))
        })
    }
}
impl ToTokens for AmbigItem {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.extend(self.token_stream.clone());
    }
}

pub(crate) struct SpreadItem<Item>
where
    Item: Parse + ToTokens,
{
    pub spread_token: Option<Token![...]>,
    pub elem: Item,
}
impl<Item> Parse for SpreadItem<Item>
where
    Item: Parse + ToTokens,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let spread_token = input.parse()?;
        let elem = input.parse()?;
        Ok(Self { spread_token, elem })
    }
}
impl<Item> ToTokens for SpreadItem<Item>
where
    Item: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.spread_token.to_tokens(tokens);
        self.elem.to_tokens(tokens)
    }
}

pub(crate) struct VariadicList<Item>
where
    Item: Parse + ToTokens,
{
    pub elems: Punctuated<SpreadItem<Item>, Token![,]>,
}
impl<Item> Parse for VariadicList<Item>
where
    Item: Parse + ToTokens,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut elems = Punctuated::new();
        while !input.is_empty() {
            elems.push_value(input.parse()?);
            if input.is_empty() {
                break;
            }
            elems.push_punct(input.parse()?);
        }
        Ok(Self { elems })
    }
}
impl<Item> ToTokens for VariadicList<Item>
where
    Item: Parse + ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.elems.to_tokens(tokens)
    }
}
impl<Item> VariadicList<Item>
where
    Item: Parse + ToTokens,
{
    pub fn into_const_tuple(self, type_context: ParseContext) -> TokenStream2 {
        fn helper<Item>(mut iter: Peekable<impl Iterator<Item = SpreadItem<Item>>>, type_context: ParseContext) -> TokenStream2
        where
            Item: Parse + ToTokens,
        {
            match iter.next() {
                Some(item) => {
                    if let Some(spread_token) = item.spread_token {
                        if iter.peek().is_some() {
                            if cfg!(feature = "complex-spread-syntax") {
                                let recurse = helper(iter, type_context);

                                let crate_path = super::get_crate_path(spread_token.span());
                                let item_elem = item.elem;
                                return match &type_context {
                                    ParseContext::Type => quote! { < #item_elem as #crate_path ::Extend< #recurse >>::Extended },
                                    ParseContext::Expr => quote! { #crate_path ::Extend::extend (#item_elem, #recurse) },
                                };
                            }
                            else {
                                spread_token
                                    .span()
                                    .unwrap()
                                    .error("Spread (`...`) is only supported on the last element of a variadic tuple type unless the `complex-spread-syntax` feature is enabled.")
                                    .emit();
                            }
                        }
                        item.elem.into_token_stream()
                    } else {
                        let recurse = helper(iter, type_context);
                        quote! { (#item, #recurse) }
                    }
                }
                None => {
                    quote! { () }
                }
            }
        }
        helper(self.elems.into_iter().peekable(), type_context)
    }
}

pub(crate) fn variadic<Item>(input: TokenStream, type_context: Option<ParseContext>) -> TokenStream
where
    Item: Parse + ToTokens,
{
    let item = parse_macro_input!(input as VariadicList<Item>);
    item.into_const_tuple(type_context.unwrap()).into()
}
