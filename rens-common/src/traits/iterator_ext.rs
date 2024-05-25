/* Built-in imports */
use core::iter::{self, FlatMap, Map};
/* Dependencies */
use either::Either;

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
        predicate: impl Fn(&Self::Item) -> bool,
        mapper: impl Fn(Self::Item) -> Self::Item,
    ) -> Map<Self, impl FnMut(Self::Item) -> Self::Item> {
        self.map(move |item| if predicate(&item) { mapper(item) } else { item })
    }

    #[inline]
    fn flat_map_if<I>(
        self,
        condition: impl Fn(&Self::Item) -> bool,
        mapper: impl Fn(Self::Item) -> I,
    ) -> FlatMap<
        Self,
        Either<I, iter::Once<Self::Item>>,
        impl FnMut(Self::Item) -> Either<I, iter::Once<Self::Item>>,
    >
    where
        I: Iterator<Item = Self::Item>,
    {
        self.flat_map(move |item| {
            if condition(&item) {
                Either::Left(mapper(item))
            } else {
                Either::Right(iter::once(item))
            }
        })
    }
}

impl<I: Iterator> IteratorExt for I {}
