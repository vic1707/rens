/* Built-in imports */
use core::iter::Map;

pub trait IteratorExt: Iterator + Sized {
    #[inline]
    fn tap_for_each(
        self,
        func: impl Fn(&Self::Item),
    ) -> Map<Self, impl FnMut(Self::Item) -> Self::Item> {
        self.map(move |item| {
            func(&item);
            item
        })
    }

    #[inline]
    fn map_if(
        self,
        mut predicate: impl FnMut(&Self::Item) -> bool,
        mut func: impl FnMut(Self::Item) -> Self::Item,
    ) -> Map<Self, impl FnMut(Self::Item) -> Self::Item> {
        self.map(move |item| if predicate(&item) { func(item) } else { item })
    }
}

impl<I: Iterator> IteratorExt for I {}
