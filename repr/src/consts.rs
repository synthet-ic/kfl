use std::sync::LazyLock;
use crate::{
    p, u,
    pat::Pat
};

/// `\0`
pub const NUL: LazyLock<Pat> = LazyLock::new(|| u!(0000));
/// `\t`
pub const HT: LazyLock<Pat> = LazyLock::new(|| u!(0009));
/// `\n`
pub const LF: LazyLock<Pat> = LazyLock::new(|| u!(000A));
/// `\v`
pub const VT: LazyLock<Pat> = LazyLock::new(|| u!(000B));
/// `\r`
pub const CR: LazyLock<Pat> = LazyLock::new(|| u!(000D));
/// ` `
pub const SP: LazyLock<Pat> = LazyLock::new(|| u!(0020));

pub const EMPTY: LazyLock<Pat> = LazyLock::new(|| Pat::empty());
pub const DIGIT: LazyLock<Pat> = LazyLock::new(|| p!(0..9));
pub const SPACE: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ' ');
pub const WORD: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & p!(A..Z) | p!(a..z));
