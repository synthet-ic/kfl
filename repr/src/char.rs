// use core::{
//     ops::{
//         BitOr, BitAnd, Range, RangeFrom, RangeTo, Mul, RangeBounds, Bound, Try,
//         ControlFlow, FromResidual, RangeFull
//     },
// };
use crate::repr::Repr;

pub trait CharExt: Into<Repr<char>> {
    fn and(self, rhs: Self) -> Repr<char> {
        self.into() & rhs.into()
    }

    fn or(self, rhs: Self) -> Repr<char> {
        self.into() | rhs.into()
    }
}

impl CharExt for char {}

// impl<T: CharExt> BitAnd<T> for T {
//     type Output = Repr<char>;

//     fn bitand(self, rhs: T) -> Self::Output {
//         self.and(rhs)
//     }
// }

// impl<T: CharExt> BitOr<T> for T {
//     type Output = Repr<char>;
    
//     fn bitor(self, rhs: T) -> Self::Output {
//         self.or(rhs)
//     }
// }
