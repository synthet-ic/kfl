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
