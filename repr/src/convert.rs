use core::ops::{
    self, RangeFrom, RangeTo, RangeFull, RangeBounds, Bound
};

use crate::repr::{Repr, Seq, Range};

impl From<char> for Repr<char> {
    fn from(value: char) -> Self {
        Self::new(&value.to_string())
    }
}   

impl From<&str> for Repr<char> {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<ops::Range<char>> for Repr<char> {
    fn from(value: ops::Range<char>) -> Self {
        Self::range(value)
    }
}

impl<T: Into<Repr<char>>> From<[T; 1]> for Repr<char> {
    fn from(value: [T; 1]) -> Self {
        value.into_iter().nth(0).unwrap().into() * ..
    }
}

impl const From<R: RangeBounds<usize>> for Range {
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

impl Into<Regex> for Repr<char> {
    fn into(self) -> Regex {
        let mut pat = String::new();
        Printer::new().print(&self.0, &mut pat).unwrap();
        Regex::new(&pat).unwrap()
    }
}
