#[test]
fn test_variadic() {
    use variadics::variadic as v;

    let _: v!(u8, u16, u32) = v!(200, 2000, 20_000);
}

#[test]
fn test_variadic_basic() {
    use variadics::variadic as v;

    let v!(a, b, c): v!(u8, u16, u32) = v!(200, 500, 20_000);
    assert_eq!(200, a);
    assert_eq!(500, b);
    assert_eq!(20_000, c);
}

#[test]
fn test_variadic_spread() {
    use variadics::variadic as v;

    let acd: v!(u8, &'static str, char) = v!(10, "hello", '😊');
    let v!(a, ...cd) = acd;
    let v!(b, c, d) = v!(-1, ...cd);

    assert_eq!(10, a);
    assert_eq!(-1, b);
    assert_eq!("hello", c);
    assert_eq!('😊', d);
}
