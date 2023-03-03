use alloc::{
    boxed::Box,
    vec::Vec
};
use core::{
    cmp,
    fmt::Debug,
    slice::Iter
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Repr<I: Integral> {
    Zero,  // TODO(rnarkk) let it hold word boundary
    /// A single character, where a character is either
    /// defined by a Unicode scalar value or an arbitrary byte. Unicode characters
    /// are preferred whenever possible. In particular, a `Byte` variant is only
    /// ever produced when it could match invalid UTF-8.
    One(I),
    Seq(Seq<I>),
    Not(Box<Repr<I>>),
    Or(Box<Repr<I>>, Box<Repr<I>>),
    And(Box<Repr<I>>, Box<Repr<I>>),
    Xor(Box<Repr<I>>, Box<Repr<I>>),
    Add(Box<Repr<I>>, Seq<I>),
    Sub(Box<Repr<I>>, Seq<I>),
    Mul(Box<Repr<I>>, Range),
    // Map(Box<Repr<I>>, Fn(Box<Repr<I>>), Fn(Box<Repr<I>>))
}

impl<I: Integral> Repr<I> {
    pub const fn empty() -> Self {
        Self::Zero
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

impl<const N: usize, I: Integral> const Into<[I; N]> for Repr<I> {
    fn into(self) -> [I; N] {
        match self {
            Repr::Empty => [],
            Repr::Not(repr) => {
                
            }
            Repr::Xor(lhs, rhs) => lhs.clone().or(rhs).sub(lhs.and(rhs)),
        }
    }
}

#[derive(Debug)]
pub struct ReprIter<'a, I>(Iter<'a, I>);

impl<'a, I> const Iterator for ReprIter<'a, I> {
    type Item = &'a I;

    fn next(&mut self) -> Option<&'a I> {
        self.0.pop()
    }
}

// impl<T> const IntoIterator for Repr<T> {
//     type Item = T;
//     type IntoIter: ReprIter<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         let mut iter = Vec::new();
//         match self {
//         }
//     }
// }

#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, Default, PartialEq, PartialOrd, Ord)]
pub struct Seq<I: Integral>(pub I, pub I);

impl<I: Integral> Seq<I> {
    pub const fn new(from: I, to: I) -> Self {
        if from <= to {
            Seq(from, to)
        } else {
            Seq(to, from)
        }
    }
    
    /// Intersect this range with the given range and return the result.
    ///
    /// If the intersection is empty, then this returns `None`.
    pub const fn and(&self, other: &Self) -> Option<Self> {
        let from = cmp::max(self.0, other.0);
        let to = cmp::min(self.1, other.1);
        if from <= to {
            Some(Self::new(from, to))
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
        let from = cmp::max(self.0, other.0);
        let to = cmp::min(self.1, other.1);
        Some(Self::new(from, to))
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

#[const_trait]
pub trait Integral: Copy + Clone + Debug + Eq + ~const PartialEq + ~const  PartialOrd + Ord {
    const MIN: Self;
    const MAX: Self;
    fn as_u32(self) -> u32;
    fn suc(self) -> Self;
    fn pred(self) -> Self;
}

impl const Integral for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
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

impl const Integral for char {
    const MIN: Self = '\x00';
    const MAX: Self = '\u{10FFFF}';
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

// 24bit
pub enum Range {
    Empty,
    From(usize),
    To(usize),
    Full(usize, usize),
}
