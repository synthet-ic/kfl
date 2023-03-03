use crate::{
    p, u,
    repr::Repr
};

/// `\0`
pub const NUL: Repr = u!(0000);
/// `\t`
pub const HT: Repr = u!(0009);
/// `\n`
pub const LF: Repr = u!(000A);
/// `\v`
pub const VT: Repr = u!(000B);
/// `\r`
pub const CR: Repr = u!(000D);
/// ` `
pub const SP: Repr = u!(0020);

pub const DIGIT: Repr = p!(0..9);
pub const SPACE: Repr = Repr::empty() & ' ';
pub const WORD: Repr = Repr::empty() & p!(A..Z) | p!(a..z);
