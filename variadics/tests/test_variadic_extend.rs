use variadics::variadic2 as v;

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
fn test_extend_spread() {
    let a = v!(2, 3);
    let b = v!(4, 5);
    let z = v!(1, ...a, ...b);
    // v!(1, ...a.extend(v!(...b)));
    assert_eq!(v!(1, 2, 3, 4, 5), z);
}
