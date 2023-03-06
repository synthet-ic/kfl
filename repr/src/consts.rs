use crate::repr::Repr;

/// `\0`
pub const NUL: Repr<char> = Repr::from('\0');
/// `\t`
pub const HT: Repr<char> = Repr::from('\t');
/// `\n`
pub const LF: Repr<char> = Repr::from('\n');
/// `\v`
pub const VT: Repr<char> = Repr::from('\u{000B}');
/// `\r`
pub const CR: Repr<char> = Repr::from('\r');
/// ` `
pub const SP: Repr<char> = Repr::from(' ');

pub const DIGIT: Repr<char> = Repr::from('0'..'9');
pub const SPACE: Repr<char> = SP;
pub const WORD: Repr<char> = Repr::from('A'..'Z') | ('a'..'z');
