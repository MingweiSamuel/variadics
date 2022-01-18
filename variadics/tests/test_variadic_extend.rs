use variadics::variadic as v;

#[test]
fn test_single_spread() {
    let a = v!(1, 2, 3);
    let b = v!(...a);
    assert_eq!(a, b);
}

#[test]
fn test_extend_expr_manual() {
    let a = v!(2, 3);
    let b = v!(4, 5);
    let z = v!(1, ...variadics::Extend::extend(a, v!(...b)));
    assert_eq!(v!(1, 2, 3, 4, 5), z);
}

#[test]
#[cfg(feature = "complex-spread-syntax")]
fn test_extend_expr_complex() {
    let a = v!(2, 3);
    let b = v!(4, 5);
    let c = v!(7, 8);
    let z = v!(1, ...a, ...b, 6, ...c);
    assert_eq!(v!(1, 2, 3, 4, 5, 6, 7, 8), z);
}

#[test]
#[cfg(feature = "complex-spread-syntax")]
fn test_extend_type_manual() {
    type A = v!(u8, u16);
    type B = v!(u16, u32);
    type C = v!(u64, u64);
    type Z = v!(u8, ...<A as variadics::Extend<<B as variadics::Extend<(u32, C)>>::Extended>>::Extended);

    let _: Z = v!(0_u8, 0_u8, 0_u16, 0_u16, 0_u32, 0_u32, 0_u64, 0_u64);
}

#[test]
#[cfg(feature = "complex-spread-syntax")]
fn test_extend_type_complex() {
    use variadics::variadic_type as vt;

    type A = v!(u8, u16);
    type B = v!(u16, u32);
    type C = v!(u64, u64);
    type Z = vt!(u8, ...A, ...B, u32, ...C);

    let _: Z = v!(0_u8, 0_u8, 0_u16, 0_u16, 0_u32, 0_u32, 0_u64, 0_u64);
}

#[test]
#[cfg(feature = "complex-spread-syntax")]
fn test_split() {
    use variadics::ignore as split;
    split! { let (...a, b): (...(u8, u8), u8) = (1, 2, 3); }
    split! { let (...a, ...b, ...c): (...(u8, u8), ...(u8, u8), ...(u8, u8)) = (1, 2, 3, 4, 5, 6); }
}
