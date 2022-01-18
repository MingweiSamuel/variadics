use variadics::{variadic as v, VariadicAsRef};

#[test]
fn test_as_ref() {
    let x = v!([1, 2, 3, 4, 5], ['a', 'b', 'c', 'd', 'e']);
    let _y = x.as_ref();
    // assert_eq!(x, y); TODO
}
