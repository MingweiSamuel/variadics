use variadics::{variadic as v, ignore, Variadic};

pub trait ZipIters {
    type Item: Variadic;

    fn next(&mut self) -> Option<Self::Item>;
}

impl ZipIters for v!() {
    type Item = v!();

    fn next(&mut self) -> Option<Self::Item> {
        Some(v!())
    }
}
impl<I, Rest> ZipIters for v!(I, ...Rest)
where
    I: Iterator,
    Rest: ZipIters,
{
    type Item = v!(I::Item, ...Rest::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let v!(this, ...rest) = self;
        if let Some(next_item) = this.next() {
            if let Some(rest_items) = rest.next() {
                return Some(v!(next_item, ...rest_items));
            }
        }
        None
    }
}

ignore! {
    impl<...Iters> ZipIters for ...Iters
    where
        for<I: ...Iters> I: Iterator,
    {
        type Item = (for<I: ...Iters> I::Item);

        fn next(&mut self) -> Option<Self::Item> {
            for iter ...in self {
                iter.next()?
            }
        }
    }
}