#![cfg_attr(not(test), no_std)]
#![feature(
    fn_traits,
    unboxed_closures,
    array_value_iter,
)]

//! Fancy helpers for functional programming.

pub mod iter_ref;
pub mod iter_move;
pub mod free;
pub mod bound;

pub use iter_ref::IterRef;
pub use iter_move::IterMove;

pub use free::Not;
pub use bound::Dot;

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
mod test {
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
