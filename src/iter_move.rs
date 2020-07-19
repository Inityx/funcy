//! Helpers for predicating on consuming functions.
//!
//! The primary functionality of this module comes from the trait [`IterMove`].

/// Convenience methods on [`Iterator`](core::iter::Iterator) for predicating on
/// consuming functions.
pub trait IterMove: Iterator {
    /// `filter` by move.
    ///
    /// Filter with a consuming predicate. The created iterator clones each item
    /// in order to test it.
    fn filter_m<P>(self, pred: P) -> FilterMove<Self, P>
    where
        Self: Sized,
        Self::Item: Clone,
        P: FnMut(Self::Item) -> bool,
    {
        FilterMove { iter: self, pred }
    }

    /// `find` by move.
    ///
    /// Search for an element with a consuming predicate. The created
    /// iterator clones each item in order to test it.
    fn find_m<P>(&mut self, mut pred: P) -> Option<Self::Item>
    where
        Self::Item: Clone,
        P: FnMut(Self::Item) -> bool,
    {
        for item in self { if pred(item.clone()) { return Some(item); } }
        None
    }

    /// `any` by move.
    ///
    /// Test if any element matches a consuming predicate.
    fn any_m<P>(&mut self, mut pred: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self { if pred(item) { return true; } }
        false
    }

    /// `all` by move.
    ///
    /// Test if every element matches a consuming predicate.
    fn all_m<P>(&mut self, mut pred: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self { if !pred(item) { return false; } }
        true
    }

    /// `position` by move.
    ///
    /// Search for an element with a consuming predicate, returning its index.
    fn position_m<P>(&mut self, mut pred: P) -> Option<usize>
    where P: FnMut(Self::Item) -> bool {
        self.enumerate().filter_map(|(i, item)|
            if pred(item) { Some(i) }
            else { None }
        ).next()
    }

    /// `rposition` by move.
    ///
    /// Search backwards for an element with a consuming predicate, returning
    /// its index.
    fn rposition_m<P>(&mut self, mut pred: P) -> Option<usize>
    where
        Self: ExactSizeIterator + DoubleEndedIterator,
        P: FnMut(Self::Item) -> bool,
    {
        self.enumerate().rev().filter_map(|(i, item)|
            if pred(item) { Some(i) }
            else { None }
        ).next()
    }
}

impl<T: Iterator> IterMove for T {}

/// An iterator filtering with `pred(Item)`.
///
/// This `struct` is created by [`IterMove::filter_m`].
#[derive(Clone, Copy, Debug)]
pub struct FilterMove<I, P> {
    iter: I,
    pred: P,
}

impl<I: Iterator, P> Iterator for FilterMove<I, P>
where
    I::Item: Clone,
    P: FnMut(I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.pred)(item.clone()) {
                return Some(item);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::array::IntoIter;

    #[test]
    fn filter_move() {
        let negatives = IntoIter::new([-1, -2, 3, -4, 5, -6])
            .filter_m(i32::is_negative)
            .collect::<Vec<_>>();

        assert_eq!(vec![-1, -2, -4, -6], negatives);
    }

    #[test]
    fn find_move() {
        let first_positive = IntoIter::new([-1, -2, 3, -4, 5, -6])
            .find_m(i32::is_positive);

        assert_eq!(Some(3), first_positive);
    }

    #[test]
    fn any_move() {
        assert!( IntoIter::new([-1, -2, 3, -4, 5, -6]).any_m(i32::is_negative));
        assert!(!IntoIter::new([ 1,  2, 3,  4, 5,  6]).any_m(i32::is_negative));
    }

    #[test]
    fn all_move() {
        assert!(!IntoIter::new([-1, -2, 3, -4, 5, -6]).all_m(i32::is_positive));
        assert!( IntoIter::new([ 1,  2, 3,  4, 5,  6]).all_m(i32::is_positive));
    }

    #[test]
    fn position_move() {
        let first_positive = IntoIter::new([-1, -2, 3, -4, 5, -6])
            .position_m(i32::is_positive);

        assert_eq!(Some(2), first_positive);
    }

    #[test]
    fn rposition_move() {
        let last_positive = IntoIter::new([-1, -2, 3, -4, 5, -6])
            .rposition_m(i32::is_positive);

        assert_eq!(Some(4), last_positive);
    }
}
