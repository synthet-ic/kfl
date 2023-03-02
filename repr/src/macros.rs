//! Macro definitions.

/// Pattern
#[macro_export]
macro_rules! p {
    ($lhs:tt | $rhs:tt) => {
        Pat::from(stringify!($lhs)) | stringify!($rhs)
    };
    ($lhs:tt .. $rhs:tt) => {
        Pat::from(stringify!($lhs).chars().nth(0).unwrap()..stringify!($rhs).chars().nth(0).unwrap())
    };
}

/// Pattern from a char
#[macro_export]
macro_rules! c {
    ($tt:literal) => {
        Pat::from($tt)
    };
}

/// Unicode
#[macro_export]
macro_rules! u {
    ($tt:tt) => {
        Pat::from(char::from_u32(u32::from_str_radix(stringify!($tt), 16).unwrap()).unwrap())
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

/// Delimit pattern by 
#[macro_export]
macro_rules! delimit {
    ($tt:expr) => {
        Pat::new(r"\(") & $tt & Pat::new(r"\)")
    };
    {$tt:expr} => {
        Pat::new(r"\{") & $tt & Pat::new(r"\}")
    };
    [$tt:expr] => {
        Pat::new(r"\[") & $tt & Pat::new(r"\]")
    };
}

// #[test]
// fn test() {
//     use crate::pat::Pat;
//     use super::{c, delimit};
//     assert_eq!(delimit!(c!('a')), Pat::from(r"\(a\)"));
//     assert_eq!(delimit!(c!{'a'}), Pat::from(r"\{a\}"));
// }