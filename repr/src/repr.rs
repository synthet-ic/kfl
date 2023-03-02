use alloc::boxed::Box;

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
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn or(self, other: Self) -> Self {
        Self::Or(box self, box other)
    }

    pub fn and(self, other: Self) -> Self {
        let hir = Hir::concat(vec![self.0, pat.into().0]);
        Self::And(box self, box other)
    }
    
    pub fn xor(self, other: Self) -> Self {
        let hir = Hir::concat(vec![self.0, pat.into().0]);
        Self::Xor(box self, box other)
    }
}
