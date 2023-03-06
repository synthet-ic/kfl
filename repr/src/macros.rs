//! Macro definitions.

/// Pi
#[macro_export]
macro_rules! and {
    [$one:expr] => {
        ::repr::Repr::from($one)
    };
    [$one:expr, $two:expr] => {
        ::repr::Repr::from($one) & ::repr::Repr::from($two)
    };
    [$one:expr, $($repr:expr),*] => {
        ::repr::Repr::from($one) & and![$($repr:expr),*]
    }
}

/// Sigma
#[macro_export]
macro_rules! or {
    [$one:expr] => {
        ::repr::Repr::from($one)
    };
    [$one:expr, $two:expr] => {
        ::repr::Repr::from($one) | ::repr::Repr::from($two)
    };
    [$one:expr, $($repr:expr),*] => {
        ::repr::Repr::from($one) | or![$($repr:expr),*]
    }
}

/// Delta
#[macro_export]
macro_rules! xor {
    [$one:expr] => {
        ::repr::Repr::from($one)
    };
    [$one:expr, $two:expr] => {
        ::repr::Repr::from($one) ^ ::repr::Repr::from($two)
    };
    [$one:expr, $($repr:expr),*] => {
        ::repr::Repr::from($one) ^ xor![$($repr:expr),*]
    }
}

// /// Seq
// #[macro_export]
// macro_rules! seq {
//     ($lhs:tt .. $rhs:tt) => {
//         Repr::Seq((stringify!($lhs).chars().nth(0).unwrap()..stringify!($rhs).chars().nth(0).unwrap()).into())
//     };
// }

/// Pattern
#[macro_export]
macro_rules! p {
    ($lhs:tt | $rhs:tt) => {
        Repr::from(stringify!($lhs)) | stringify!($rhs)
    };
}

/// Pattern from a char
#[macro_export]
macro_rules! c {
    ($tt:literal) => {
        Repr::from($tt)
    };
}

// /// Unicode
// #[macro_export]
// macro_rules! u {
//     ($tt:tt) => {
//         Repr::from(char::from_u32(u32::from_str_radix(stringify!($tt), 16).unwrap()).unwrap())
//     }
// }

/// Escaped character
#[macro_export]
macro_rules! e {
    ($tt:tt) => {
        Repr::from(format!(r"\{}", stringify!($tt)));
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
        Repr::new(r"\(") & $tt & Repr::new(r"\)")
    };
    {$tt:expr} => {
        Repr::new(r"\{") & $tt & Repr::new(r"\}")
    };
    [$tt:expr] => {
        Repr::new(r"\[") & $tt & Repr::new(r"\]")
    };
}

// #[test]
// fn test() {
//     use crate::pat::Repr<char>;
//     use super::{c, delimit};
//     assert_eq!(delimit!(c!('a')), Repr::from(r"\(a\)"));
//     assert_eq!(delimit!(c!{'a'}), Repr::from(r"\{a\}"));
// }
