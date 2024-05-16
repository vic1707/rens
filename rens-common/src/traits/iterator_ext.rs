use core::iter::Map;

pub trait IteratorExt: Iterator {
    #[inline]
    fn i_tap<F>(
        self,
        func: F,
    ) -> Map<Self, impl FnMut(Self::Item) -> Self::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item),
    {
        self.map(move |item| {
            func(&item);
            item
        })
    }

    #[inline]
    fn map_if<P, F>(
        self,
        mut predicate: P,
        mut func: F,
    ) -> Map<Self, impl FnMut(Self::Item) -> Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.map(move |item| if predicate(&item) { func(item) } else { item })
    }
}

impl<I: Iterator> IteratorExt for I {}
