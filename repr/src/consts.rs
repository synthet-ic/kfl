use crate::{
    u,
    repr::Repr
};

/// `\0`
pub const NUL: Repr<char> = u!(0000);
/// `\t`
pub const HT: Repr<char> = u!(0009);
/// `\n`
pub const LF: Repr<char> = u!(000A);
/// `\v`
pub const VT: Repr<char> = u!(000B);
/// `\r`
pub const CR: Repr<char> = u!(000D);
/// ` `
pub const SP: Repr<char> = u!(0020);

pub const DIGIT: Repr<char> = Repr::from('0'..'9');
pub const SPACE: Repr<char> = SP;
pub const WORD: Repr<char> = Repr::from('A'..'Z') | ('a'..'z');
