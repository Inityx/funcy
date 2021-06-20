//! Helpers for methods.

use core::ops::{Deref, DerefMut};

/// Reference methods with a receiver pre-bound.
///
/// # Examples
///
/// Binding variables:
///
/// ```
/// use funcy::bind;
///
/// let mut v = Vec::new();
/// (1..=3).for_each(bind!(v::push));
/// assert_eq!(vec![1, 2, 3], v);
/// ```
///
/// Binding expressions:
///
/// ```
/// use std::ops::Mul;
/// use funcy::bind;
///
/// let doubled: Vec<_> = (1..=3)
///     .map(bind!({1 + 1}::mul))
///     .collect();
///
/// assert_eq!(vec![2, 4, 6], doubled);
/// ```
#[macro_export]
macro_rules! bind {
    ($receiver:ident::$method:ident) => { |x| $receiver.$method(x) };
    ({$receiver:expr}::$method:ident) => { |x| { $receiver }.$method(x) };
}

#[cfg(test)]
mod bind_test {
    #[test]
    fn bind_val_inherent() {
        let mut v = Vec::new();
        (1..=3).for_each(bind!(v::push));
        assert_eq!(vec![1, 2, 3], v);
    }

    #[test]
    fn bind_expr_inherent() {
        fn get() -> &'static str { "hello" }

        assert!(
            (0..5)
                .map(bind!({get()}::is_char_boundary))
                .all(std::convert::identity)
        )
    }

    #[test]
    fn bind_val_trait() {
        use std::ops::Mul;

        let two = 2;
        let doubled = (1..=3)
            .map(bind!(two::mul))
            .collect::<Vec<_>>();

        assert_eq!(vec![2, 4, 6], doubled);
    }

    #[test]
    fn bind_expr_trait() {
        use std::ops::Mul;

        let doubled = (1..=3)
            .map(bind!({1 + 1}::mul))
            .collect::<Vec<_>>();

        assert_eq!(vec![2, 4, 6], doubled);
    }
}

/// Glue for using arbitary functions as methods.
///
/// This trait supports unary functions passing the receiver by:
/// * value
/// * `&`
/// * `&mut`
/// * [`Deref`]
/// * [`DerefMut`]
pub trait Dot {
    /// Call as a `self` method.
    fn dot<B, F>(self, func: F) -> B
    where
        Self: Sized,
        F: FnOnce(Self) -> B,
    {
        func(self)
    }

    /// Call `func` as a `&self` method.
    fn dot_ref<'a, B, F>(&'a self, func: F) -> B
    where F: FnOnce(&'a Self) -> B {
        func(self)
    }

    /// Call `func` as a `&mut self` method.
    fn dot_refmut<'a, B, F>(&'a mut self, func: F) -> B
    where F: FnOnce(&'a mut Self) -> B {
        func(self)
    }

    /// Call `func` as a `&self` method on `Self`'s [`Deref`] target.
    fn dot_deref<'a, B, F>(&'a self, func: F) -> B
    where
        Self: Deref,
        F: FnOnce(&'a <Self as Deref>::Target) -> B,
    {
        func(self.deref())
    }

    /// Call `func` as a `&mut self` method on `Self`'s [`Deref`] target.
    fn dot_derefmut<'a, B, F>(&'a mut self, func: F) -> B
    where
        Self: DerefMut,
        F: FnOnce(&'a mut <Self as Deref>::Target) -> B
    {
        func(self.deref_mut())
    }
}

impl<T> Dot for T {}

#[cfg(test)]
mod dot_test {
    use super::*;

    #[test]
    fn dot() {
        fn char_count(s: &str) -> usize { s.chars().count() }

        assert_eq!(5, "hello".dot(char_count));
    }

    #[test]
    fn dot_ref() {
        fn first_half (v: &Vec<i32>) -> &[i32] { v.split_at(v.len() / 2).0 }
        fn second_half(v: &Vec<i32>) -> &[i32] { v.split_at(v.len() / 2).1 }

        let vec = vec![1, 2, 3, 4, 5];
        let first_half  = vec.dot_ref(first_half);
        let second_half = vec.dot_ref(second_half);

        assert_eq!(&[1, 2,  ][..], first_half );
        assert_eq!(&[3, 4, 5][..], second_half);
    }

    #[test]
    fn dot_mut() {
        fn first(v: &mut Vec<i32>) -> i32 { v.remove(0) }

        let mut vec = vec![4,5,6];

        assert_eq!(4, vec.dot_refmut(first));
        assert_eq!(5, vec.dot_refmut(first));
    }

    #[test]
    fn dot_deref() {
        fn until_l(s: &str) -> Option<&str> { s.split('l').next() }
        fn after_l(s: &str) -> Option<&str> { s.rsplit('l').next() }

        let hello = String::from("hello");
        let he = hello.dot_deref(until_l);
        let o  = hello.dot_deref(after_l);

        assert_eq!(Some("he"), he);
        assert_eq!(Some("o"), o);
    }

    #[test]
    fn dot_deref_mut() {
        fn count(s: &mut str) -> usize { s.chars().count() }

        assert_eq!(5, String::from("hello").dot_derefmut(count));
    }
}
