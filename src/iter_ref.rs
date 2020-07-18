//! Helpers for transforming with non-consuming functions.

use core::ops::{DerefMut, Deref};

/// Convenience methods for transforming with non-consuming functions.
pub trait IterRef: Sized + Iterator {
    /// Map with a closure that takes its argument by ref.
    ///
    /// This is useful for mapping unary `&self` methods over an iterator of values.
    fn map_ref<B, F>(self, func: F) -> MapRef<Self, F>
    where F: FnMut(&Self::Item) -> B {
        MapRef { iter: self, func }
    }

    /// Map with a closure that takes its argument by mut ref.
    ///
    /// This is useful for mapping unary `&mut self` methods over an iterator of values.
    fn map_ref_mut<B, F>(self, func: F) -> MapMut<Self, F>
    where F: FnMut(&mut Self::Item) -> B {
        MapMut { iter: self, func }
    }

    /// Map with a closure that takes the `Item`'s `Deref::Target` by ref.
    ///
    /// This is useful for mapping unary `Deref::Target` `&self` methods over an iterator of values.
    fn map_deref<B, F>(self, func: F) -> MapDeref<Self, F>
    where
        Self::Item: Deref,
        F: FnMut(&<Self::Item as Deref>::Target) -> B,
    {
        MapDeref { iter: self, func }
    }

    /// Map with a closure that takes the `Item`'s `Deref::Target` by mut ref.
    ///
    /// This is useful for mapping unary `Deref::Target` `&mut self` methods over an iterator of values.
    fn map_deref_mut<B, F>(self, func: F) -> MapDerefMut<Self, F>
    where
        Self::Item: DerefMut,
        F: FnMut(&mut <Self::Item as Deref>::Target) -> B,
    {
        MapDerefMut { iter: self, func }
    }
}

impl<T: Iterator> IterRef for T {}

/// An iterator that maps the values of `iter` with `func`, taken by reference.
///
/// This `struct` is created by the [`map_ref`] method on [`IterRef`].
///
/// [`map_ref`]: trait.IterRef.html#method.map_ref
/// [`IterRef`]: trait.IterRef.html
#[derive(Clone, Copy, Debug)]
pub struct MapRef<I, F> {
    iter: I,
    func: F,
}

impl<B, I: Iterator, F> Iterator for MapRef<I, F>
where F: FnMut(&I::Item) -> B {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().as_ref().map(&mut self.func)
    }
}

/// An iterator that maps the values of `iter` with `func`, taken by mutable reference.
///
/// This `struct` is created by the [`map_ref_mut`] method on [`IterRef`].
///
/// [`map_ref_mut`]: trait.IterRef.html#method.map_mut
/// [`IterRef`]: trait.IterRef.html
#[derive(Clone, Copy, Debug)]
pub struct MapMut<I, F> {
    iter: I,
    func: F,
}

impl<B, I: Iterator, F> Iterator for MapMut<I, F>
where F: FnMut(&mut I::Item) -> B {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().as_mut().map(&mut self.func)
    }
}

/// An iterator that maps the values of `iter` with `func`, taken via [`Deref`].
///
/// This `struct` is created by the [`map_deref`] method on [`IterRef`].
///
/// [`map_deref`]: trait.IterRef.html#method.map_deref
/// [`IterRef`]: trait.IterRef.html
#[derive(Clone, Copy, Debug)]
pub struct MapDeref<I, F> {
    iter: I,
    func: F,
}

impl<B, I: Iterator, F> Iterator for MapDeref<I, F>
where
    I::Item: Deref,
    F: FnMut(&<I::Item as Deref>::Target) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().as_deref().map(&mut self.func)
    }
}

/// An iterator that maps the values of `iter` with `func`, taken via [`DerefMut`].
///
/// This `struct` is created by the [`map_deref_mut`] method on [`IterRef`].
///
/// [`map_deref_mut`]: trait.IterRef.html#method.map_deref_mut
/// [`IterRef`]: trait.IterRef.html
#[derive(Clone, Copy, Debug)]
pub struct MapDerefMut<I, F> {
    iter: I,
    func: F,
}

impl<B, I: Iterator, F> Iterator for MapDerefMut<I, F>
where
    I::Item: DerefMut,
    F: FnMut(&mut <I::Item as Deref>::Target) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().as_deref_mut().map(&mut self.func)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::iter::once;

    struct IntWrapper(i32);

    impl IntWrapper {
        fn get(&self) -> i32 { self.0 }
        fn pop_half(&mut self) -> i32 {
            let half = self.0 / 2;
            self.0 -= half;
            half
        }
    }

    #[test]
    fn map_ref() {
        assert_eq!(
            5,
            once(IntWrapper(5))
                .map_ref(IntWrapper::get)
                .next().unwrap(),
        );
    }

    #[test]
    fn map_mut() {
        assert_eq!(
            2,
            once(IntWrapper(5))
                .map_ref_mut(IntWrapper::pop_half)
                .next()
                .unwrap(),
        );
    }

    #[test]
    fn map_deref() {
        assert_eq!(
            5,
            once(Box::new(IntWrapper(5)))
                .map_deref(IntWrapper::get)
                .next().unwrap(),
        );
    }

    #[test]
    fn map_deref_mut() {
        assert_eq!(
            2,
            once(Box::new(IntWrapper(5)))
                .map_deref_mut(IntWrapper::pop_half)
                .next().unwrap(),
        );
    }
}
