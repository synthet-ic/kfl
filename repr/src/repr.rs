pub enum Repr<I> {
    Empty,
    Range(I),
    Not(Repr<I>),
    Or(Repr<I>, Repr<I>),
    And(Repr<I>, Repr<I>),
    Xor(Repr<I>, Repr<I>),
    Add(Repr<I>, I),
    Sub(Repr<I>, I)
}
