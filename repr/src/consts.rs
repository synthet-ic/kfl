use crate::{
    u, seq,
    repr::Repr
};

/// `\0`
pub const NUL: Repr<&'static str, char> = u!(0000);
/// `\t`
pub const HT: Repr<&'static str, char> = u!(0009);
/// `\n`
pub const LF: Repr<&'static str, char> = u!(000A);
/// `\v`
pub const VT: Repr<&'static str, char> = u!(000B);
/// `\r`
pub const CR: Repr<&'static str, char> = u!(000D);
/// ` `
pub const SP: Repr<&'static str, char> = u!(0020);

pub const DIGIT: Repr<&'static str, char> = seq!(0..9);
pub const SPACE: Repr<&'static str, char> = SP;
pub const WORD: Repr<&'static str, char> = seq!(A..Z) | seq!(a..z);
