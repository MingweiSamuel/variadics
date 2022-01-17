use variadics::{variadic as v, variadic_fn};

#[test]
fn identity() {
    variadic_fn! {
        fn my_fn() -> usize {
            5
        }
    }

    assert_eq!(5, my_fn());
}

#[test]
fn ignored_variadics() {
    variadic_fn! {
        fn noop_all<...Ts>() -> usize {
            5
        }
    }

    assert_eq!(5, noop_all::<v!(usize, usize, String)>());
}

#[test]
fn basic_variadics() {
    variadic_fn! {
        fn format_all<...Ts: std::fmt::Debug>(vals: ...Ts) -> String {
            format!("{:?}", vals)
        }
    }

    assert_eq!(
        "(1, (\"Hi\", ([6, 7, 8], ())))",
        format_all(v!(1, "Hi", [6, 7, 8]))
    );
}
