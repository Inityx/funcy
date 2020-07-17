/// A trait providing convenience methods for predicating on consuming functions.
pub trait IterMove: Iterator {
    /// Filter with a consuming predicate.
    ///
    /// The created iterator clones each item in order to test it.
    fn filter_move<P>(self, pred: P) -> FilterMove<Self, P>
    where
        Self: Sized,
        Self::Item: Clone,
        P: FnMut(Self::Item) -> bool,
    {
        FilterMove { iter: self, pred }
    }

    /// Searches for an element that satisfies a consuming predicate.
    ///
    /// The created iterator clones each item in order to test it.
    fn find_move<P>(&mut self, mut pred: P) -> Option<Self::Item>
    where
        Self::Item: Clone,
        P: FnMut(Self::Item) -> bool,
    {
        for item in self { if pred(item.clone()) { return Some(item); } }
        None
    }

    /// Tests if any element matches a consuming predicate.
    fn any_move<P>(&mut self, mut pred: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self { if pred(item) { return true; } }
        false
    }

    /// Tests if every element matches a consuming predicate.
    fn all_move<P>(&mut self, mut pred: P) -> bool
    where P: FnMut(Self::Item) -> bool {
        for item in self { if !pred(item) { return false; } }
        true
    }

    /// Searches for an element in an iterator, returning its index.
    fn position_move<P>(&mut self, mut pred: P) -> Option<usize>
    where P: FnMut(Self::Item) -> bool {
        self.enumerate().filter_map(|(i, item)|
            if pred(item) { Some(i) }
            else { None }
        ).next()
    }

    /// Searches backwards for an element in an iterator, returning its index.
    fn rposition_move<P>(&mut self, mut pred: P) -> Option<usize>
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

/// An iterator that filters the values of `iter` with `pred`, taken by value.
///
/// This `struct` is created by the [`filter_move`] method on [`IterMove`].
///
/// [`filter_move`]: trait.IterMove.html#method.filter_move
/// [`IterMove`]: trait.IterMove.html
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
        loop {
            match self.iter.next() {
                Some(item) if (self.pred)(item.clone()) => break Some(item),
                None => break None,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filter_move() {
        let negatives = vec![-1, -2, 3, -4, 5, -6]
            .into_iter()
            .filter_move(i32::is_negative)
            .collect::<Vec<_>>();

        assert_eq!(vec![-1, -2, -4, -6], negatives);
    }

    #[test]
    fn find_move() {
        let first_positive = vec![-1, -2, 3, -4, 5, -6]
            .into_iter()
            .find_move(i32::is_positive);

        assert_eq!(Some(3), first_positive);
    }

    #[test]
    fn any_move() {
        assert!( vec![-1, -2, 3, -4, 5, -6].into_iter().any_move(i32::is_negative));
        assert!(!vec![ 1,  2, 3,  4, 5,  6].into_iter().any_move(i32::is_negative));
    }

    #[test]
    fn all_move() {
        assert!(!vec![-1, -2, 3, -4, 5, -6].into_iter().all_move(i32::is_positive));
        assert!( vec![ 1,  2, 3,  4, 5,  6].into_iter().all_move(i32::is_positive));
    }

    #[test]
    fn position_move() {
        let first_positive = vec![-1, -2, 3, -4, 5, -6]
            .into_iter()
            .position_move(i32::is_positive);

        assert_eq!(Some(2), first_positive);
    }

    #[test]
    fn rposition_move() {
        let last_positive = vec![-1, -2, 3, -4, 5, -6]
            .into_iter()
            .rposition_move(i32::is_positive);

        assert_eq!(Some(4), last_positive);
    }
}
