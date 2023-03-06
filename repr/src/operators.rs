use core::ops::{BitOr, BitAnd, BitXor, Range, Mul, RangeBounds};

use crate::repr::{Repr, Seq, Integral};

impl<S, I: ~const Integral<S>> const BitAnd<I> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitand(self, rhs: I) -> Repr<S, I> {
        self.and(rhs)
    }
}

// impl<S, I: ~const Integral<S>> const BitAnd<S> for Repr<S, I>
// {
//     type Output = Repr<S, I>;

//     fn bitand(self, rhs: T) -> Repr<S, I> {
//         self.and(rhs)
//     }
// }

impl BitAnd<Repr<&'static str, char>> for &str {
    type Output = Repr<&'static str, char>;

    fn bitand(self, rhs: Repr<&'static str, char>) -> Self::Output {
        rhs.clone().and(self)
    }
}

impl<S, I: ~const Integral<S>> BitAnd<Repr<S, I>> for Repr<S, I> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.and(rhs)
    }
}

// impl BitAnd<Range<u8>> for Repr<char> {
//     type Output = Self;

//     fn bitand(self, rhs: Range<u8>) -> Repr<char> {
//         self.and(rhs)
//     }
// }

impl<S, I: ~const Integral<S>> BitAnd<Range<I>> for Repr<S, I> {
    type Output = Self;

    fn bitand(self, rhs: Range<I>) -> Self::Output {
        self.and(rhs)
    }
}

// impl<S, I: ~const Integral<S>, T: Into<Self>> BitAnd<[T; 1]> for Repr<S, I> {
//     type Output = Self;

//     fn bitand(self, rhs: [T; 1]) -> Self::Output {
//         self.and(Repr::from(rhs) * ..)
//     }
// }

impl<S, I: ~const Integral<S>> BitOr<I> for Repr<S, I> {
    type Output = Repr<S, I>;
    
    fn bitor(self, rhs: I) -> Repr<S, I> {
        self.or(rhs)
    }
}

impl BitOr<&str> for Repr<&'static str, char> {
    type Output = Repr<&'static str, char>;

    fn bitor(self, rhs: &str) -> Repr<&'static str, char> {
        self.or(Self::One(rhs))
    }
}

impl BitOr<Repr<&'static str, char>> for &str {
    type Output = Repr<&'static str, char>;

    fn bitor(self, rhs: Repr<&'static str, char>) -> Repr<&'static str, char> {
        Self::One(rhs).or(self)
    }
}

impl<S, I: ~const Integral<S>> BitOr<Repr<S, I>> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitor(self, rhs: Self) -> Repr<S, I> {
        self.or(rhs)
    }
}

impl<S, I: ~const Integral<S>> BitOr<Range<I>> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitor(self, rhs: Range<I>) -> Repr<S, I> {
        self.or(rhs.into())
    }
}

// impl<S, I: ~const Integral<S>, T: Into<Repr<S, I>>> BitOr<[T; 1]> for Repr<S, I> {
//     type Output = Self;

//     fn bitor(self, rhs: [T; 1]) -> Self {
//         self.or(Repr::from(rhs) * ..)
//     }
// }

impl<R: RangeBounds<usize>, S, I: ~const Integral<S>> const Mul<R> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn mul(self, rhs: R) -> Repr<S, I> {
        Self::Mul(box self, rhs.into())
    }
}

impl<S, I: ~const Integral<S>> const BitAnd<Seq<S, I>> for Seq<S, I> {
    type Output = Seq<S, I>;

    fn bitand(self, rhs: Seq<S, I>) -> Seq<S, I> {
        self.and(&rhs)
    }
}

impl<S, I: ~const Integral<S>> const BitOr<Seq<S, I>> for Seq<S, I> {
    type Output = Seq<S, I>;

    fn bitor(self, rhs: Seq<S, I>) -> Seq<S, I> {
        self.or(&rhs)
    }
}

impl<S, I: ~const Integral<S>> const BitXor<Seq<S, I>> for Seq<S, I> {
    type Output = Seq<S, I>;

    fn bitxor(self, rhs: Seq<S, I>) -> Seq<S, I> {
        self.xor(&rhs)
    }
}
