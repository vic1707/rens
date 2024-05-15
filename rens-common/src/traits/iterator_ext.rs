use core::iter::Map;

pub trait IteratorExt: Iterator {
    #[inline]
    fn i_tap<F>(self, func: F) -> Map<Self, impl FnMut(Self::Item) -> Self::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item)
    {
        self.map(move |item| {
            func(&item);
            item
        })
    }
}

impl<I: Iterator> IteratorExt for I {}
