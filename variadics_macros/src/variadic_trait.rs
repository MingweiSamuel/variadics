use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, GenericParam, ItemTrait, Path, PathArguments, PathSegment,
    PredicateType, Type, TypeParam, TypePath, WherePredicate,
};

pub(crate) fn variadic_trait(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemTrait);

    let ItemTrait {
        attrs,
        vis,
        unsafety,
        auto_token,
        trait_token,
        ident,
        generics,
        colon_token: _,
        supertraits,
        brace_token,
        items,
    } = item;

    if !items.is_empty() {
        brace_token
            .span
            .unwrap()
            .error("Variadic traits currently cannot have items.")
            .emit();
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    // Generics augmented with _Item and _Rest.
    let mut generics_aug = generics.clone();
    let ident_item = Ident::new("_Item", Span::call_site());
    let ident_rest = Ident::new("_Rest", Span::call_site());

    if let Some(where_clause) = &mut generics_aug.where_clause {
        for pred in where_clause.predicates.iter_mut() {
            if let WherePredicate::Type(PredicateType {
                lifetimes: _,
                bounded_ty:
                    Type::Path(TypePath {
                        qself: None,
                        path:
                            Path {
                                leading_colon: None,
                                segments,
                            },
                    }),
                colon_token: _,
                bounds: _,
            }) = pred
            {
                if 1 == segments.len() {
                    if let PathSegment {
                        ident,
                        arguments: PathArguments::None,
                    } = segments.first_mut().unwrap()
                    {
                        if ident == "_Item" || ident == "_Rest" {
                            ident
                                .span()
                                .unwrap()
                                .error(&*format!(
                                    "{} is a reserved generic argument. Use `Self` to bound items.",
                                    ident
                                ))
                                .emit();
                        } else if ident == "Self" {
                            *ident = Ident::new("_Item", ident.span());
                        }
                    }
                }
            }
        }
    }

    generics_aug.params.push(GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ellipses_token: None,
        ident: ident_item.clone(),
        colon_token: None,
        bounds: supertraits, // Forward supertraits onto nested type.
        eq_token: None,
        default: None,
    }));

    generics_aug.params.push(GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ellipses_token: None,
        ident: ident_rest.clone(),
        colon_token: None,
        bounds: parse_quote!( #ident #type_generics ),
        eq_token: None,
        default: None,
    }));

    let (impl_generics_aug, _type_generics_aug, where_clause_aug) = generics_aug.split_for_impl();

    let expanded = quote! {
        #( #attrs )*
        #vis #unsafety #auto_token #trait_token #ident #impl_generics #where_clause {
            #( #items )*
        }

        impl #impl_generics_aug #ident #type_generics for (#ident_item, #ident_rest) #where_clause_aug {
            #( #items )*
        }

        impl #impl_generics #ident #type_generics for () #where_clause {
            #( #items )*
        }
    };

    expanded.into()
}
