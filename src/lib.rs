#![cfg_attr(not(test), no_std)]
#![feature(
    fn_traits,
    unboxed_closures,
    bool_to_option
)]

//! Fancy helpers for functional programming.

mod iter_ref;
mod iter_move;
mod function;
mod binding;

pub use iter_ref::IterRef;
pub use iter_move::IterMove;

pub use function::Not;
pub use binding::Dot;
