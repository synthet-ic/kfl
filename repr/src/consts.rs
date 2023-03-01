use std::sync::LazyLock;
use crate::pat::Pat;

pub const EMPTY: LazyLock<Pat> = LazyLock::new(|| Pat::empty());
pub const DIGIT: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ('0'..'9'));
pub const SPACE: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ' ');
pub const WORD: LazyLock<Pat> = LazyLock::new(|| Pat::empty() & ('A'..'Z') | ('a'..'z'));
