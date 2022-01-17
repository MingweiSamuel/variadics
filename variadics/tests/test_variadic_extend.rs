use variadics::variadic as v;

#[test]
fn test_single_spread() {
    let a = v!(1, 2, 3);
    let b = v!(...a);
    assert_eq!(a, b);
}

#[test]
fn test_extend_fn() {
    let a = v!(2, 3);
    let b = v!(4, 5);
    let z = v!(1, ...variadics::Extend::extend(a, v!(...b)));
    assert_eq!(v!(1, 2, 3, 4, 5), z);
}

#[test]
#[cfg(feature = "complex-spread-syntax")]
fn test_extend_complex() {
    let a = v!(2, 3);
    let b = v!(4, 5);
    let c = v!(7, 8);
    let z = v!(1, ...a, ...b, 6, ...c);
    assert_eq!(v!(1, 2, 3, 4, 5, 6, 7, 8), z);
}
