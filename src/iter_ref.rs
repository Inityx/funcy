//! Helpers for transforming with non-consuming functions.
//!
//! The primary functionality of this module comes from the trait [`IterRef`].

use core::ops::{DerefMut, Deref};

/// Convenience methods for transforming with non-consuming functions.
pub trait IterRef: Sized + Iterator {
    /// `map` by ref.
    ///
    /// Useful for mapping unary `&self` methods over an iterator of values.
    fn map_r<B, F>(self, func: F) -> MapRef<Self, F>
    where F: FnMut(&Self::Item) -> B {
        MapRef { iter: self, func }
    }

    /// `map` by ref mut.
    ///
    /// Useful for mapping unary `&mut self` methods over an iterator of values.
    fn map_r_m<B, F>(self, func: F) -> MapMut<Self, F>
    where F: FnMut(&mut Self::Item) -> B {
        MapMut { iter: self, func }
    }

    /// `map` by `Deref`.
    ///
    /// Useful for mapping unary `Deref::Target`s' `&self` methods over an
    /// iterator of values.
    fn map_d<B, F>(self, func: F) -> MapDeref<Self, F>
    where
        Self::Item: Deref,
        F: FnMut(&<Self::Item as Deref>::Target) -> B,
    {
        MapDeref { iter: self, func }
    }

    /// `map` by `DerefMut`.
    ///
    /// This is useful for mapping unary `Deref::Target`s' `&mut self` methods
    /// over an iterator of values.
    fn map_d_m<B, F>(self, func: F) -> MapDerefMut<Self, F>
    where
        Self::Item: DerefMut,
        F: FnMut(&mut <Self::Item as Deref>::Target) -> B,
    {
        MapDerefMut { iter: self, func }
    }
}

impl<T: Iterator> IterRef for T {}

/// An iterator mapping `func(&Item)`.
///
/// This `struct` is created by [`IterRef::map_r`].
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

/// An iterator mapping `func(&mut Item)`.
///
/// This `struct` is created by [`IterRef::map_r_m`].
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

/// An iterator mapping `func(&<Item as Deref>::Target)`.
///
/// This `struct` is created by [`IterRef::map_d`].
///
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

/// An iterator mapping `func(&mut <Item as Deref>::Target)`.
///
/// This `struct` is created by [`IterRef::map_d_m`].
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
                .map_r(IntWrapper::get)
                .next().unwrap(),
        );
    }

    #[test]
    fn map_mut() {
        assert_eq!(
            2,
            once(IntWrapper(5))
                .map_r_m(IntWrapper::pop_half)
                .next()
                .unwrap(),
        );
    }

    #[test]
    fn map_deref() {
        assert_eq!(
            5,
            once(Box::new(IntWrapper(5)))
                .map_d(IntWrapper::get)
                .next().unwrap(),
        );
    }

    #[test]
    fn map_deref_mut() {
        assert_eq!(
            2,
            once(Box::new(IntWrapper(5)))
                .map_d_m(IntWrapper::pop_half)
                .next().unwrap(),
        );
    }
}
