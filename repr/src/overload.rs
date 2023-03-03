use core::ops::{
    BitOr, BitAnd, Range, RangeFrom, RangeTo, Mul, RangeBounds, Bound, Try,
    ControlFlow, FromResidual, RangeFull
};

use crate::repr::Repr;

impl BitAnd<char> for Repr<char> {
    type Output = Repr<char>;

    fn bitand(self, rhs: char) -> Repr<char> {
        self.and(rhs)
    }
}

impl BitAnd<&str> for Repr<char> {
    type Output = Repr<char>;

    fn bitand(self, rhs: &str) -> Repr<char> {
        self.and(rhs)
    }
}

impl BitAnd<Repr> for &str {
    type Output = Repr<char>;

    fn bitand(self, rhs: Repr<char>) -> Repr<char> {
        rhs.clone().and(self)
    }
}

impl BitAnd<Repr> for Repr<char> {
    type Output = Repr<char>;

    fn bitand(self, rhs: Self) -> Repr<char> {
        self.and(rhs)
    }
}

// impl BitAnd<Range<u8>> for Repr<char> {
//     type Output = Self;

//     fn bitand(self, rhs: Range<u8>) -> Repr<char> {
//         self.and(rhs)
//     }
// }

impl BitAnd<Range<char>> for Repr<char> {
    type Output = Repr<char>;

    fn bitand(self, rhs: Range<char>) -> Repr<char> {
        self.and(rhs)
    }
}

impl<T: Into<Repr>> BitAnd<[T; 1]> for Repr<char> {
    type Output = Repr<char>;

    fn bitand(self, rhs: [T; 1]) -> Repr<char> {
        self.and(Repr::from(rhs) * ..)
    }
}

impl BitOr<char> for Repr<char> {
    type Output = Repr<char>;
    
    fn bitor(self, rhs: char) -> Repr<char> {
        self.or(rhs)
    }
}

impl BitOr<&str> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: &str) -> Repr<char> {
        self.or(rhs)
    }
}

impl BitOr<Repr> for &str {
    type Output = Repr<char>;

    fn bitor(self, rhs: Repr<char>) -> Repr<char> {
        rhs.or(self)
    }
}

impl BitOr<Repr> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: Self) -> Repr<char> {
        self.or(rhs)
    }
}

impl BitOr<Range<char>> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: Range<char>) -> Repr<char> {
        self.or(rhs)
    }
}

impl<T: Into<Repr>> BitOr<[T; 1]> for Repr<char> {
    type Output = Repr<char>;

    fn bitor(self, rhs: [T; 1]) -> Repr<char> {
        self.or(Repr::from(rhs) * ..)
    }
}

impl Mul<u32> for Repr<char> {
    type Output = Repr<char>;

    fn mul(self, rhs: u32) -> Repr<char> {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Exactly(rhs)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<RangeFull> for Repr<char> {
    type Output = Repr<char>;

    fn mul(self, _: RangeFull) -> Repr<char> {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<Range<u32>> for Repr<char> {
    type Output = Repr<char>;

    fn mul(self, rhs: Range<u32>) -> Repr<char> {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(rhs.start, rhs.end)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<RangeFrom<u32>> for Repr<char> {
    type Output = Repr<char>;

    fn mul(self, rhs: RangeFrom<u32>) -> Repr<char> {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::AtLeast(rhs.start)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<RangeTo<u32>> for Repr<char> {
    type Output = Repr<char>;

    fn mul(self, rhs: RangeTo<u32>) -> Repr<char> {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(0, rhs.end)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Try for Repr<char> {
    type Output = Repr<char>;
    type Residual = Repr;

    fn from_output(output: Repr) -> Self {
        output * (0..1)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Repr> {
        match self.0.kind() {
            HirKind::Repetition(rep) => {
                let rep = Repetition {
                    kind: rep.kind.clone(),
                    greedy: false,
                    hir: rep.hir.clone()
                };
                ControlFlow::Continue(Self(Hir::repetition(rep)))
            },
            _ => ControlFlow::Continue(self * (0..1))
        }
    }
}

impl FromResidual for Repr<char> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}
