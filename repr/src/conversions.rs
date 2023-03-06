use core::ops::{self, RangeBounds, Bound};

use crate::repr::{Repr, Seq, Range, Integral};

impl<I: ~const Integral> const From<I> for Repr<I> {
    fn from(value: I) -> Repr<I> {
        Self::One(value)
    }
}   

// impl From<&str> for Repr<char> {
//     fn from(value: &str) -> Self {
//         Self::new(value)
//     }
// }

// TODO(rnarkk) Is ther any use to generalise it to R: RangeBounds<usize>?
impl<I: ~const Integral> const From<ops::Range<I>> for Seq<I> {
    fn from(range: ops::Range<I>) -> Self {
        Seq(range.start, range.end)
    }
}

impl<I: ~const Integral> const From<ops::Range<I>> for Repr<I> {
    fn from(range: ops::Range<I>) -> Self {
        Repr::Seq(range.into())
    }
}

// impl<S, I: ~const Integral, T: Into<Repr<I>>> const From<[T; 1]> for Repr<I> {
//     fn from(value: [T; 1]) -> Repr<I> {
//         value.into_iter().nth(0).unwrap().into() * ..
//     }
// }

impl<R: ~const RangeBounds<usize>> const From<R> for Range {
    fn from(range: R) -> Self {
        use Bound::*;
        match (range.start_bound(), range.end_bound()) {
            (Unbounded, Unbounded) => Range::Empty,
            (Unbounded, Excluded(end)) => Range::To(end.clone()),
            (Included(start), Unbounded) => Range::From(start.clone()),
            (Included(start), Excluded(end)) => Range::Full(start.clone(),
                                                            end.clone()),
            _ => panic!("Try m..n instead of m..=n.")
        }
    }
}
