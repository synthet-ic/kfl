use alloc::{
    boxed::Box,
    vec::Vec
};
use core::{
    cmp,
    slice::Iter
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Repr<I> {
    Zero,  // TODO(rnarkk) let it hold word boundary
    /// A single character, where a character is either
    /// defined by a Unicode scalar value or an arbitrary byte. Unicode characters
    /// are preferred whenever possible. In particular, a `Byte` variant is only
    /// ever produced when it could match invalid UTF-8.
    One(I),
    Range(Interval<I>),  // Or(Empty, I)?
    Not(Box<Repr<I>>),
    Or(Box<Repr<I>>, Box<Repr<I>>),
    And(Box<Repr<I>>, Box<Repr<I>>),
    Xor(Box<Repr<I>>, Box<Repr<I>>),
    Add(Box<Repr<I>>, Interval<I>),
    Sub(Box<Repr<I>>, Interval<I>),
    Mul(Box<Repr<I>>, Range<u32>),
    // Map(Box<Repr<I>>, Fn(Box<Repr<I>>), Fn(Box<Repr<I>>))
}

impl<I> for Repr<I> {
    pub const fn empty() -> Self {
        Self::Empty
    }
    
    pub const fn not(self) -> Self {
        Self::Not(box self)
    }
    
    pub const fn and(self, other: Self) -> Self {
        Self::And(box self, box other)
    }
    
    pub const fn or(self, other: Self) -> Self {
        Self::Or(box self, box other)
    }
    
    pub const fn xor(self, other: Self) -> Self {
        Self::Xor(box self, box other)
    }
    
    pub const fn add(self, range: I) -> Self {
        Self::Add(box self, range)
    }
    
    pub const fn sub(self, range: I) -> Self {
        Self::Sub(box self, range)
    }
    
    pub const fn mul(self, range: I) -> Self {
        Self::Mul(box self, range)
    }
}

impl<const N: usize> const Into<[I; N]> for Repr<I> {
    fn into(self) -> T {
        use Self::*;
        match self {
            Empty => [],
            Not(repr) => {
                
            }
            Xor(lhs, rhs) => lhs.clone().or(rhs).sub(lhs.and(rhs)),
        }
    }
}

#[derive(Debug)]
pub struct ReprIter<'a, I>(Vec<I>);

impl<'a, I> const Iterator for ReprIter<'a, I> {
    type Item = &'a I;

    fn next(&mut self) -> Option<&'a I> {
        self.0.pop()
    }
}

impl const IntoIterator for Repr<I> {
    type Item = I;
    type IntoIter: ReprIter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = Vec::new();
        match self {
        }
    }
}

#[derive(Clone + Copy + Debug + Default + Eq + PartialEq + PartialOrd + Ord)]
pub struct Interval<Bound: Bound>(pub Bound, pub Bound);

impl<Bound: Bound> Interval<Bound> {
    pub const fn new(start: Bound, end: Bound) -> Self {
        let mut output = Self::default();
        if start <= end {
            output.0 = start;
            output.1 = end;
        } else {
            output.0 = end;
            output.1 = start;
        }
        output
    }
    
    /// Intersect this range with the given range and return the result.
    ///
    /// If the intersection is empty, then this returns `None`.
    pub const fn and(&self, other: &Self) -> Option<Self> {
        let start = cmp::max(self.0, other.0);
        let end = cmp::min(self.1, other.1);
        if start <= end {
            Some(Self::new(start, end))
        } else {
            None
        }
    }
    
    /// Union the given overlapping range into this range.
    ///
    /// If the two ranges aren't contiguous, then this returns `None`.
    pub const fn or(&self, other: &Self) -> Option<Self> {
        if !self.is_contiguous(other) {
            return None;
        }
        let start = cmp::max(self.0, other.0);
        let end = cmp::min(self.1, other.1);
        Some(Self::create(start, end))
    }
    
    /// Compute the symmetric difference the given range from this range. This
    /// returns the union of the two ranges minus its intersection.
    pub const fn xor(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        let or = match self.or(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(or) => or,
        };
        let and = match self.and(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(and) => and,
        };
        or.sub(&and)
    }
}

pub trait Bound: Copy + Clone + Debug + Eq + PartialEq + PartialOrd + Ord {
    pub const MIN: Bound;
    pub const MAX: Bound;
    pub const fn as_u32(self) -> u32;
    pub const fn suc(self) -> Self;
    pub const fn pred(self) -> Self;
}

impl Bound for u8 {
    const MIN = u8::MIN;
    const MAX = u8::MAX;
    fn as_u32(self) -> u32 {
        self as u32
    }
    fn suc(self) -> Self {
        self.checked_add(1).unwrap()
    }
    fn pred(self) -> Self {
        self.checked_sub(1).unwrap()
    }
}

impl Bound for char {
    const MIN = '\x00';
    const MAX = '\u{10FFFF}'
    fn as_u32(self) -> u32 {
        self as u32
    }

    fn suc(self) -> Self {
        match self {
            '\u{D7FF}' => '\u{E000}',
            c => char::from_u32((c as u32).checked_add(1).unwrap()).unwrap(),
        }
    }

    fn pred(self) -> Self {
        match self {
            '\u{E000}' => '\u{D7FF}',
            c => char::from_u32((c as u32).checked_sub(1).unwrap()).unwrap(),
        }
    }
}
