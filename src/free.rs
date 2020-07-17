/// A predicate modifier that inverts the result of the wrapped predicate.
#[derive(Clone, Copy, Debug)]
pub struct Not<P>(pub P);

impl<T, P> FnOnce<(T,)> for Not<P>
where P: FnOnce(T) -> bool {
    type Output = bool;
    extern "rust-call" fn call_once(self, (arg,): (T,)) -> Self::Output {
        !(self.0)(arg)
    }
}

impl<T, P> FnMut<(T,)> for Not<P>
where P: FnMut(T) -> bool {
    extern "rust-call" fn call_mut(&mut self, (arg,): (T,)) -> Self::Output {
        !(self.0)(arg)
    }
}

impl<T, P> Fn<(T,)> for Not<P>
where P: Fn(T) -> bool {
    extern "rust-call" fn call(&self, (arg,): (T,)) -> Self::Output {
        !(self.0)(arg)
    }
}

#[cfg(test)]
mod test {
    use super::Not;
    use std::collections::HashSet;

    #[test]
    fn not_fn() {
        fn is_odd(val: &i32) -> bool {
            val % 2 != 0
        }

        let even = Not(is_odd);
        drop(even); // Check for `Copy`

        let evens = vec![1, 2, 3, 4, 5, 6]
            .into_iter()
            .filter(even)
            .collect::<Vec<_>>();

        assert_eq!(vec![2, 4, 6], evens);
    }

    #[test]
    fn not_fn_mut() {
        let mut seen = HashSet::new();
        let unique = |&val: &i32| seen.insert(val);

        let first_repeat = vec![1, 2, 3, 4, 2, 6].into_iter().find(Not(unique));

        assert_eq!(Some(2), first_repeat);
    }

    #[test]
    fn not_fn_once() {
        struct OddTester;
        impl OddTester {
            fn test(self, val: i32) -> bool { val % 2 != 0}
        }

        let odd = OddTester;
        assert_eq!(Some(5).map(Not(|x| odd.test(x))), Some(false));

        let odd = OddTester;
        assert_eq!(Some(4).map(Not(|x| odd.test(x))), Some(true));
    }
}
