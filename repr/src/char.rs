// use core::{
//     ops::{
//         BitOr, BitAnd, Range, RangeFrom, RangeTo, Mul, RangeBounds, Bound, Try,
//         ControlFlow, FromResidual, RangeFull
//     },
// };
use crate::pat::Pat;

pub trait CharExt: Into<Pat> {
    fn and(self, rhs: Self) -> Pat {
        self.into() & rhs.into()
    }

    fn or(self, rhs: Self) -> Pat {
        self.into() | rhs.into()
    }
}

impl CharExt for char {}

// impl<T: CharExt> BitAnd<T> for T {
//     type Output = Pat;

//     fn bitand(self, rhs: T) -> Self::Output {
//         self.and(rhs)
//     }
// }

// impl<T: CharExt> BitOr<T> for T {
//     type Output = Pat;
    
//     fn bitor(self, rhs: T) -> Self::Output {
//         self.or(rhs)
//     }
// }
