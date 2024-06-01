/* Built-in imports */
use core::iter::FilterMap;

pub trait ResultIteratorExt<T, E>:
    Iterator<Item = Result<T, E>> + Sized
{
    #[inline]
    fn filter_map_ok(
        self,
        err_handler: impl Fn(E),
    ) -> FilterMap<Self, impl FnMut(Self::Item) -> Option<T>> {
        self.filter_map(move |result_item| match result_item {
            Ok(ok) => Some(ok),
            Err(err) => {
                err_handler(err);
                None
            },
        })
    }

    #[inline]
    fn filter_err(
        self,
    ) -> FilterMap<Self, impl FnMut(Self::Item) -> Option<E>> {
        self.filter_map(Result::err)
    }

    #[inline]
    fn filter_ok(self) -> FilterMap<Self, impl FnMut(Self::Item) -> Option<T>> {
        self.filter_map(Result::ok)
    }
}

impl<T, E, I: Iterator<Item = Result<T, E>>> ResultIteratorExt<T, E> for I {}
