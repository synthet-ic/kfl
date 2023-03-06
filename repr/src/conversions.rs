use core::ops::{self, RangeBounds, Bound};

use crate::repr::{Repr, Seq, Range, Integral};

impl<S, I: ~const Integral<S>> const From<I> for Repr<S, I> {
    fn from(value: I) -> Repr<S, I> {
        Self::One(value)
    }
}   

// impl From<&str> for Repr<char> {
//     fn from(value: &str) -> Self {
//         Self::new(value)
//     }
// }

// TODO(rnarkk) Is ther any use to generalise it to R: RangeBounds<usize>?
impl<S, I: ~const Integral<S>> const From<ops::Range<I>> for Repr<S, I> {
    fn from(value: ops::Range<I>) -> Repr<S, I> {
        Self::Seq(value.into())
    }
}

// impl<S, I: ~const Integral<S>, T: Into<Repr<S, I>>> const From<[T; 1]> for Repr<S, I> {
//     fn from(value: [T; 1]) -> Repr<S, I> {
//         value.into_iter().nth(0).unwrap().into() * ..
//     }
// }

impl<R: RangeBounds<usize>> const From<R> for Range {
    fn from(range: R) -> Self {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Unbounded, Bound::Unbounded) => Range::Empty,
            (Bound::Unbounded, Bound::Excluded(end)) => Range::To(end),
            (Bound::Included(start), Bound::Unbounded) => Range::From(start),
            (Bound::Included(start), Bound::Excluded(end))
                => Range::Full(start, end),
            _ => panic!("Try m..n in place of m..=n.")
        }
    }
}
