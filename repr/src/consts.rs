use std::sync::LazyLock;
use crate::pat::Pat;

/// `\t`
pub const TAB: LazyLock<Pat> = LazyLock::new(|| u!(0009));
/// `\n`
pub const LF: LazyLock<Pat> = LazyLock::new(|| u!(000A));
/// `\r`
pub const CR: LazyLock<Pat> = LazyLock::new(|| u!(000D));

pub const EMPTY: LazyLock<Pat> = LazyLock::new(|| Pat::empty());
pub const DIGIT: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ('0'..'9'));
pub const SPACE: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ' ');
pub const WORD: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ('A'..'Z') | ('a'..'z'));
