//! Helpers for methods.

use core::ops::{Deref, DerefMut};

/// Glue for using arbitary functions/closures as if they were methods.
///
/// This trait supports unary functions, and supports passing the receiver by move, `&(mut)`, and `Deref`(`Mut`).
pub trait Dot {
    /// Call the provided function as a `self` method.
    fn dot<B, F>(self, func: F) -> B
    where
        Self: Sized,
        F: FnOnce(Self) -> B,
    {
        func(self)
    }

    /// Call the provided function as a `&self` method.
    fn dot_ref<B, F>(&self, func: F) -> B
    where F: FnOnce(&Self) -> B {
        func(self)
    }

    /// Call the provided function as a `&mut self` method.
    fn dot_mut<B, F>(&mut self, func: F) -> B
    where F: FnOnce(&mut Self) -> B {
        func(self)
    }

    /// Call the provided function as a `&self` method on the receiver's `Deref` target.
    fn dot_deref<B, F>(&mut self, func: F) -> B
    where
        Self: Deref,
        F: FnOnce(&<Self as Deref>::Target) -> B,
    {
        func(self.deref())
    }

    /// Call the provided function as a `&mut self` method on the receiver's `DerefMut` target.
    fn dot_deref_mut<B, F>(&mut self, func: F) -> B
    where
        Self: DerefMut,
        F: FnOnce(&mut <Self as Deref>::Target) -> B
    {
        func(self.deref_mut())
    }
}

impl<T> Dot for T {}

/// Reference an inherent or trait method, with a receiver pre-bound.
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
/// let doubled = (1..=3)
///     .map(bind!({1 + 1}::mul))
///     .collect::<Vec<_>>();
///
/// assert_eq!(vec![2, 4, 6], doubled);
/// ```
#[macro_export]
macro_rules! bind {
    ($receiver:ident::$method:ident) => { |x| $receiver.$method(x) };
    ({$receiver:expr}::$method:ident) => { |x| { $receiver }.$method(x) };
}

#[cfg(test)]
mod test {
    use super::*;
    fn char_count(s: &str) -> usize { s.chars().count() }
    fn char_mut_count(s: &mut str) -> usize { s.chars().count() }
    fn vec_len(v: &Vec<i32>) -> usize { v.len() }
    fn vec_second(v: &mut Vec<i32>) -> i32 { v.remove(1) }

    #[test]
    fn dot() {
        assert_eq!(5, "hello".dot(char_count));
    }

    #[test]
    fn dot_ref() {
        assert_eq!(3, vec![1,2,3].dot_ref(vec_len));
    }

    #[test]
    fn dot_mut() {
        assert_eq!(5, vec![4,5,6].dot_mut(vec_second));
    }

    #[test]
    fn dot_deref() {
        assert_eq!(5, String::from("hello").dot_deref(char_count));
    }

    #[test]
    fn dot_deref_mut() {
        assert_eq!(5, String::from("hello").dot_deref_mut(char_mut_count));
    }

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
