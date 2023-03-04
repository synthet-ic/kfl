use crate::repr::Repr;

pub trait CharExt: Into<Repr<&'static str, char>> {
    fn and(self, rhs: Self) -> Repr<&'static str, char> {
        self.into() & rhs.into()
    }

    fn or(self, rhs: Self) -> Repr<&'static str, char> {
        self.into() | rhs.into()
    }
}

impl CharExt for char {}
