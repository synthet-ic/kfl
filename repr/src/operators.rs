use core::ops::{BitOr, BitAnd, BitXor, Range, Mul, RangeBounds};

use crate::repr::{Repr, Seq, Integral, Iterator};

impl<I: ~const Integral> const BitAnd<I> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitand(self, rhs: I) -> Repr<S, I> {
        self.and(rhs)
    }
}

impl<S, I: ~const Integral<S>> const BitAnd<S> for Repr<S, I>
{
    type Output = Repr<S, I>;

    fn bitand(self, rhs: T) -> Repr<S, I> {
        self.and(rhs)
    }
}

impl BitAnd<Repr<char>> for &str {
    type Output = Repr<char>;

    fn bitand(self, rhs: Repr<char>) -> Repr<char> {
        rhs.clone().and(self)
    }
}

impl<I: ~const Integral> BitAnd<Repr<S, I>> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitand(self, rhs: Self) -> Repr<S, I> {
        self.and(rhs)
    }
}

// impl BitAnd<Range<u8>> for Repr<char> {
//     type Output = Self;

//     fn bitand(self, rhs: Range<u8>) -> Repr<char> {
//         self.and(rhs)
//     }
// }

impl<I: ~const Integral> BitAnd<Range<I>> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitand(self, rhs: Range<I>) -> Repr<S, I> {
        self.and(rhs)
    }
}

impl<T: Into<Repr<char>>> BitAnd<[T; 1]> for Repr<char> {
    type Output = Repr<char>;

    fn bitand(self, rhs: [T; 1]) -> Repr<char> {
        self.and(Repr::from(rhs) * ..)
    }
}

impl<I: ~const Integral> BitOr<I> for Repr<S, I> {
    type Output = Repr<S, I>;
    
    fn bitor(self, rhs: I) -> Repr<S, I> {
        self.or(rhs)
    }
}

impl BitOr<&str> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: &str) -> Repr<char> {
        self.or(rhs)
    }
}

impl BitOr<Repr<char>> for &str {
    type Output = Repr<char>;

    fn bitor(self, rhs: Repr<char>) -> Repr<char> {
        rhs.or(self)
    }
}

impl<I: ~const Integral> BitOr<Repr<S, I>> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitor(self, rhs: Self) -> Repr<S, I> {
        self.or(rhs)
    }
}

impl<I: ~const Integral> BitOr<Range<I>> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn bitor(self, rhs: Range<I>) -> Repr<S, I> {
        self.or(rhs)
    }
}

impl<T: Into<Repr<char>>> BitOr<[T; 1]> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: [T; 1]) -> Repr<char> {
        self.or(Repr::from(rhs) * ..)
    }
}

impl<R: RangeBounds<usize>, I: ~const Integral> const Mul<R> for Repr<S, I> {
    type Output = Repr<S, I>;

    fn mul(self, rhs: R) -> Repr<S, I> {
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
