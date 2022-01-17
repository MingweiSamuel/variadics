use variadics::variadic2 as v;

#[test]
fn test_variadic() {
    let _: v!(u8, u16, u32) = v!(200, 2000, 20_000);
}

#[test]
fn test_variadic_basic() {
    let v!(a, b, c): v!(u8, u16, u32) = v!(200, 500, 20_000);
    assert_eq!(200, a);
    assert_eq!(500, b);
    assert_eq!(20_000, c);
}

#[test]
fn test_variadic_spread() {
    let acd: v!(u8, &'static str, char) = v!(10, "hello", '😊');
    let v!(a, ...cd) = acd;
    let v!(b, c, d) = v!(-1, ...cd);

    assert_eq!(10, a);
    assert_eq!(-1, b);
    assert_eq!("hello", c);
    assert_eq!('😊', d);
}

#[test]
fn test_turbofish_bastion() {
    let (the, guardian, stands, resolute) = ("the", "Turbofish", "remains", "undefeated");
    let x = v!(the < guardian, stands > (resolute));

    use std::collections::HashMap;
    type Y = v!(HashMap<usize, usize>);
}
