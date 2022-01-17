use variadics::variadic_trait;

variadic_trait! {
    pub trait VariadicClone: Clone {}
}

variadic_trait! {
    pub trait FancyTest<Rhs: ToString = Self>: PartialEq<Rhs>
    where
        Rhs: Clone,
        Self: Default,
    {}
}

#[test]
fn variadic_trait_test() {}
