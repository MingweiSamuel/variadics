use sealed::sealed;

use variadics_macros::variadic as v;
pub use variadics_macros::*;

#[sealed]
pub trait Variadic {}
#[sealed]
impl Variadic for v!() {}
#[sealed]
impl<X, Rest> Variadic for v!(X, ...Rest) where Rest: Variadic {}

#[sealed]
pub trait Extend<Suffix>
where
    Suffix: Variadic,
{
    type Extended: Variadic;
    fn extend(self, input: Suffix) -> Self::Extended;
}
#[sealed]
impl<Suffix> Extend<Suffix> for v!()
where
    Suffix: Variadic,
{
    type Extended = Suffix;
    fn extend(self, suffix: Suffix) -> Self::Extended {
        suffix
    }
}
#[sealed]
impl<Item, Rest, Suffix> Extend<Suffix> for v!(Item, ...Rest)
where
    Rest: Extend<Suffix>,
    Suffix: Variadic,
{
    type Extended = v!(Item, ...Rest::Extended);
    fn extend(self, suffix: Suffix) -> Self::Extended {
        let v!(item, ...rest) = self;
        v!(item, ...rest.extend(suffix))
    }
}

// #[sealed]
pub trait VariadicAsRef<'a>: 'a + Variadic {
    type Output;

    fn as_ref(&'a self) -> Self::Output;
}
// #[sealed]
impl<'a> VariadicAsRef<'a> for v!() {
    type Output = v!();

    fn as_ref(&'a self) -> Self::Output {
        v!()
    }
}
// #[sealed]
impl<'a, X: 'a, Rest> VariadicAsRef<'a> for v!(X, ...Rest)
where
    Rest: VariadicAsRef<'a>,
{
    type Output = v!(&'a X, ...Rest::Output);

    fn as_ref(&'a self) -> Self::Output {
        let v!(this, ...rest) = self;
        v!(this, ...rest.as_ref())
    }
}

pub trait Tuple {
    type Variadic: Variadic;

    fn variadic(self) -> Self::Variadic;
}
pub trait Flat: Variadic {
    type Tuple: Tuple;

    fn flat(self) -> Self::Tuple;
}

macro_rules! variadic_impls {
    ($($t:tt),*) => {
        impl<$($t,)*> Tuple for ($($t,)*) {
            type Variadic = v!($($t,)*);

            fn variadic(self) -> Self::Variadic {
                #[allow(non_snake_case, unused_parens)]
                let ($($t,)*) = self;
                v!($($t,)*)
            }
        }
        impl<$($t,)*> Flat for v!($($t,)*) {
            #[allow(unused_parens)]
            type Tuple = ($($t,)*);

            fn flat(self) -> Self::Tuple {
                #[allow(non_snake_case)]
                let v!($($t,)*) = self;
                ($($t,)*)
            }
        }
    }
}

variadic_impls!();
variadic_impls!(T1);
variadic_impls!(T1, T2);
variadic_impls!(T1, T2, T3);
variadic_impls!(T1, T2, T3, T4);
variadic_impls!(T1, T2, T3, T4, T5);
variadic_impls!(T1, T2, T3, T4, T5, T6);
variadic_impls!(T1, T2, T3, T4, T5, T6, T7);
variadic_impls!(T1, T2, T3, T4, T5, T6, T7, T8);
variadic_impls!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
variadic_impls!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
variadic_impls!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
variadic_impls!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
