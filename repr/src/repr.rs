use alloc::boxed::Box;

#[derive(Clone, Debug)]
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
