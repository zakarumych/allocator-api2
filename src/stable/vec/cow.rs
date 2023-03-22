use alloc::borrow::Cow;
use core::iter::FromIterator;

use super::Vec;

impl<'a, T: Clone> From<&'a [T]> for Cow<'a, [T]> {
    /// Creates a [`Borrowed`] variant of [`Cow`]
    /// from a slice.
    ///
    /// This conversion does not allocate or clone the data.
    ///
    /// [`Borrowed`]: crate::borrow::Cow::Borrowed
    #[inline(always)]
    fn from(s: &'a [T]) -> Cow<'a, [T]> {
        Cow::Borrowed(s)
    }
}

impl<'a, T: Clone> From<Vec<T>> for Cow<'a, [T]> {
    /// Creates an [`Owned`] variant of [`Cow`]
    /// from an owned instance of [`Vec`].
    ///
    /// This conversion does not allocate or clone the data.
    ///
    /// [`Owned`]: crate::borrow::Cow::Owned
    #[inline(always)]
    fn from(v: Vec<T>) -> Cow<'a, [T]> {
        Cow::Owned(v)
    }
}

impl<'a, T: Clone> From<&'a Vec<T>> for Cow<'a, [T]> {
    /// Creates a [`Borrowed`] variant of [`Cow`]
    /// from a reference to [`Vec`].
    ///
    /// This conversion does not allocate or clone the data.
    ///
    /// [`Borrowed`]: crate::borrow::Cow::Borrowed
    #[inline(always)]
    fn from(v: &'a Vec<T>) -> Cow<'a, [T]> {
        Cow::Borrowed(v.as_slice())
    }
}

impl<'a, T> FromIterator<T> for Cow<'a, [T]>
where
    T: Clone,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Cow<'a, [T]> {
        Cow::Owned(FromIterator::from_iter(it))
    }
}
