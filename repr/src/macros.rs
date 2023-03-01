//! Macro definitions.

use crate::pat::Pat;

/// Pattern
#[macro_export]
macro_rules! p {
    ($lhs:tt | $rhs:tt) => {
        Pat::from(stringify!($lhs)) | stringify!($rhs);
    };
    ($lhs:tt .. $rhs:tt) => {
    };
}

/// Unicode
#[macro_export]
macro_rules! u {
    ($tt:tt) => {
        Pat::from(char::from_u32(u32::from_str_radix(stringify!($tt), 16)).unwrap());
    }
}

/// Escaped character
#[macro_export]
macro_rules! e {
    ($tt:tt) => {
        Pat::from(format!(r"\{}", stringify!($tt)));
    }
}

/// Ignore pattern
#[macro_export]
macro_rules! i {
    ($tt:tt) => {
        
    }
}
