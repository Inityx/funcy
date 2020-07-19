//! Helpers for methods.

use core::ops::{Deref, DerefMut};

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
    fn dot_r<'a, B, F>(&'a self, func: F) -> B
    where F: FnOnce(&'a Self) -> B {
        func(self)
    }

    /// Call `func` as a `&mut self` method.
    fn dot_r_m<'a, B, F>(&'a mut self, func: F) -> B
    where F: FnOnce(&'a mut Self) -> B {
        func(self)
    }

    /// Call `func` as a `&self` method on `Self`'s [`Deref`] target.
    fn dot_d<'a, B, F>(&'a self, func: F) -> B
    where
        Self: Deref,
        F: FnOnce(&'a <Self as Deref>::Target) -> B,
    {
        func(self.deref())
    }

    /// Call `func` as a `&mut self` method on `Self`'s [`Deref`] target.
    fn dot_d_m<'a, B, F>(&'a mut self, func: F) -> B
    where
        Self: DerefMut,
        F: FnOnce(&'a mut <Self as Deref>::Target) -> B
    {
        func(self.deref_mut())
    }
}

impl<T> Dot for T {}

#[cfg(test)]
mod test {
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
        let first_half  = vec.dot_r(first_half);
        let second_half = vec.dot_r(second_half);

        assert_eq!(&[1, 2,  ][..], first_half );
        assert_eq!(&[3, 4, 5][..], second_half);
    }

    #[test]
    fn dot_mut() {
        fn vec_first(v: &mut Vec<i32>) -> i32 { v.remove(0) }
        let mut vec = vec![4,5,6];
        assert_eq!(4, vec.dot_r_m(vec_first));
        assert_eq!(5, vec.dot_r_m(vec_first));
    }

    #[test]
    fn dot_deref() {
        fn until_l(s: &str) -> Option<&str> { s.split('l').next() }
        fn after_l(s: &str) -> Option<&str> { s.rsplit('l').next() }

        let hello = String::from("hello");
        let he = hello.dot_d(until_l);
        let o  = hello.dot_d(after_l);

        assert_eq!(Some("he"), he);
        assert_eq!(Some("o"), o);
    }

    #[test]
    fn dot_deref_mut() {
        fn char_mut_count(s: &mut str) -> usize { s.chars().count() }
        assert_eq!(5, String::from("hello").dot_d_m(char_mut_count));
    }
}
