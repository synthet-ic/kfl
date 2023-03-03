use core::ops::{
    Range, RangeFrom, RangeTo, RangeFull, RangeBounds, Bound
};

use crate::repr::Repr;

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

impl From<&Hir> for Repr<char> {
    fn from(value: &Hir) -> Self {
        Self(value.clone())
    }
}

impl From<Range<char>> for Repr<char> {
    fn from(value: Range<char>) -> Self {
        Self::range(value)
    }
}

impl<T: Into<Repr<char>>> From<[T; 1]> for Repr<char> {
    fn from(value: [T; 1]) -> Self {
        value.into_iter().nth(0).unwrap().into() * ..
    }
}

// impl Deref for Repr<char> {
//     type Target = *mut Self;

//     fn deref(&self) -> &Self::Target {
//         &(*self * ..)
//     }
// }

impl Into<Regex> for Repr<char> {
    fn into(self) -> Regex {
        let mut pat = String::new();
        Printer::new().print(&self.0, &mut pat).unwrap();
        Regex::new(&pat).unwrap()
    }
}
