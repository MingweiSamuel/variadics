use proc_macro::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::fold::{fold_type, fold_type_param, Fold};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, ItemFn, TraitBound, TraitBoundModifier, Type, TypeParam, TypeParamBound,
};

struct VariadicFold;

impl Fold for VariadicFold {
    fn fold_type(&mut self, item: Type) -> Type {
        let item = fold_type(self, item);
        match item {
            Type::Variadic(type_variadic) => {
                type_variadic
                    .span()
                    .unwrap()
                    .warning("Variadic types are not really implemented (TODO)")
                    .emit();
                *type_variadic.elem
            }
            x => x,
        }
    }

    fn fold_type_param(&mut self, item: TypeParam) -> TypeParam {
        let mut item = fold_type_param(self, item);
        if item.ellipses_token.is_some() {
            let span = item.span();
            let variadic_path = syn::parse2(quote_spanned!(span=> Variadic)).unwrap();
            let path = super::get_crate_path(variadic_path, span);
            item.bounds.push(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path,
            }));
        }
        item
    }
}

pub(crate) fn variadic_fn(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let transformed_item = VariadicFold.fold_item_fn(item);

    transformed_item.into_token_stream().into()
}
