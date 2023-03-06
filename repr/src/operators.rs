use core::ops::{BitOr, BitAnd, BitXor, Range, Mul, RangeBounds};

use crate::repr::{Repr, Seq, Integral};

impl<I: ~const Integral> const BitAnd<I> for Repr<I> {
    type Output = Repr<I>;

    fn bitand(self, rhs: I) -> Repr<I> {
        self.and(rhs)
    }
}

// impl<I: ~const Integral> const BitAnd<S> for Repr<I>
// {
//     type Output = Repr<I>;

//     fn bitand(self, rhs: T) -> Repr<I> {
//         self.and(rhs)
//     }
// }

impl BitAnd<Repr<char>> for &str {
    type Output = Repr<char>;

    fn bitand(self, rhs: Repr<char>) -> Self::Output {
        rhs.clone().and(self)
    }
}

impl<I: ~const Integral> BitAnd<Repr<I>> for Repr<I> {
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

impl<I: ~const Integral> BitAnd<Range<I>> for Repr<I> {
    type Output = Self;

    fn bitand(self, rhs: Range<I>) -> Self::Output {
        self.and(rhs)
    }
}

// impl<S, I: ~const Integral, T: Into<Self>> BitAnd<[T; 1]> for Repr<I> {
//     type Output = Self;

//     fn bitand(self, rhs: [T; 1]) -> Self::Output {
//         self.and(Repr::from(rhs) * ..)
//     }
// }

impl<I: ~const Integral> BitOr<I> for Repr<I> {
    type Output = Repr<I>;
    
    fn bitor(self, rhs: I) -> Repr<I> {
        self.or(rhs)
    }
}

impl BitOr<&str> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: &str) -> Repr<char> {
        self.or(Self::One(rhs))
    }
}

impl BitOr<Repr<char>> for &str {
    type Output = Repr<char>;

    fn bitor(self, rhs: Repr<char>) -> Repr<char> {
        Self::One(rhs).or(self)
    }
}

impl<I: ~const Integral> BitOr<Repr<I>> for Repr<I> {
    type Output = Repr<I>;

    fn bitor(self, rhs: Self) -> Repr<I> {
        self.or(rhs)
    }
}

impl<I: ~const Integral> BitOr<Range<I>> for Repr<I> {
    type Output = Repr<I>;

    fn bitor(self, rhs: Range<I>) -> Repr<I> {
        self.or(rhs.into())
    }
}

// impl<S, I: ~const Integral, T: Into<Repr<I>>> BitOr<[T; 1]> for Repr<I> {
//     type Output = Self;

//     fn bitor(self, rhs: [T; 1]) -> Self {
//         self.or(Repr::from(rhs) * ..)
//     }
// }

impl<R: RangeBounds<usize>, I: ~const Integral> const Mul<R> for Repr<I> {
    type Output = Repr<I>;

    fn mul(self, rhs: R) -> Repr<I> {
        Self::Mul(box self, rhs.into())
    }
}

impl<I: ~const Integral> const BitAnd<Seq<I>> for Seq<I> {
    type Output = Seq<I>;

    fn bitand(self, rhs: Seq<I>) -> Seq<I> {
        self.and(&rhs)
    }
}

impl<I: ~const Integral> const BitOr<Seq<I>> for Seq<I> {
    type Output = Seq<I>;

    fn bitor(self, rhs: Seq<I>) -> Seq<I> {
        self.or(&rhs)
    }
}

impl<I: ~const Integral> const BitXor<Seq<I>> for Seq<I> {
    type Output = Seq<I>;

    fn bitxor(self, rhs: Seq<I>) -> Seq<I> {
        self.xor(&rhs)
    }
}
