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
