use alloc::{
    boxed::Box,
    vec::Vec
};
use core::slice::Iter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Repr<I> {
    Empty,
    Range(I),  // Or(Empty, I)?
    Not(Box<Repr<I>>),
    Or(Box<Repr<I>>, Box<Repr<I>>),
    And(Box<Repr<I>>, Box<Repr<I>>),
    Xor(Box<Repr<I>>, Box<Repr<I>>),
    Add(Box<Repr<I>>, I),
    Sub(Box<Repr<I>>, I),
    // Map(Box<Repr<I>>, Fn(Box<Repr<I>>))
}

impl<I> for Repr<I> {
    pub const fn empty() -> Self {
        Self::Empty
    }
    
    pub const fn not(self) -> Self {
        Self::Not(box self)
    }

    pub const fn or(self, other: Self) -> Self {
        Self::Or(box self, box other)
    }

    pub const fn and(self, other: Self) -> Self {
        Self::And(box self, box other)
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
pub struct Range(usize, usize);

