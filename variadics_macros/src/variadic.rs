use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Token, Type};

#[derive(Debug)]
enum TypeOrExpr {
    Type(Type),
    Expr(Expr),
}
impl Parse for TypeOrExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative;

        let fork = input.fork();
        if let Ok(ty) = fork.parse() {
            input.advance_to(&fork);
            Ok(TypeOrExpr::Type(ty))
        } else {
            Ok(TypeOrExpr::Expr(input.parse()?))
        }
    }
}
impl ToTokens for TypeOrExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Expr(expr) => expr.to_tokens(tokens),
        }
    }
}

#[derive(Debug)]
struct SpreadTypeOrExpr {
    pub spread_token: Option<Token![...]>,
    pub elem: TypeOrExpr,
}
impl Parse for SpreadTypeOrExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let spread_token = input.parse()?;
        let elem = input.parse()?;
        Ok(Self { spread_token, elem })
    }
}
impl ToTokens for SpreadTypeOrExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.spread_token.to_tokens(tokens);
        self.elem.to_tokens(tokens)
    }
}

struct VariadicList {
    pub elems: Punctuated<SpreadTypeOrExpr, Token![,]>,
}
impl Parse for VariadicList {
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
impl ToTokens for VariadicList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.elems.to_tokens(tokens)
    }
}
impl VariadicList {
    pub fn into_const_tuple(self) -> proc_macro2::TokenStream {
        fn helper(mut iter: impl Iterator<Item = SpreadTypeOrExpr>) -> proc_macro2::TokenStream {
            match iter.next() {
                Some(item) => {
                    if let Some(spread_token) = item.spread_token {
                        if iter.next().is_some() {
                            spread_token
                                .span()
                                .unwrap()
                                .error("Spread elements are only supported as the last element in a variadic tuple type.")
                                .emit();
                        }
                        item.elem.into_token_stream()
                    } else {
                        let recurse = helper(iter);
                        quote! { (#item, #recurse) }
                    }
                }
                None => {
                    quote! { () }
                }
            }
        }
        helper(self.elems.into_iter())
    }
}

pub(crate) fn variadic(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as VariadicList);

    item.into_const_tuple().into()
}
