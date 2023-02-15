// use std::fmt::Debug;
// use kfl::{
//     Decode, DecodeChildren,
//     span::Span
// };
// use miette::Diagnostic;

pub fn hint_same_type<T>(_lhs: &T, _rhs: &T) {}

#[macro_export]
macro_rules! assert_decode {
    ($input:literal, $output:expr) => {
        let node = kfl::decode("<test>", $input).unwrap();
        let output = $output;
        common::hint_same_type(&node, &output);
        assert!(node == output);
    }
}

#[macro_export]
macro_rules! assert_decode_error {
    ($ty:ty, $input:literal, $output:literal) => {
        let err = kfl::decode::<$ty>("<test>", $input).unwrap_err().to_string();
        assert!(err == $output);
    }
}

/*
pub fn parse_doc<T>(input: &str) -> T
    where T: DecodeChildren<Span>
{
    kfl::parse("<test>", input).unwrap()
}

pub fn assert_parse_doc<T>(input: &str, output: T)
    where T: DecodeChildren<Span> + Debug + PartialEq
{
    let node = parse_doc::<T>(input);
    assert_eq!(node, output);
}

pub fn parse_doc_err<T: DecodeChildren<Span> + Debug>(text: &str) -> String {
    let err = kfl::parse::<T>("<test>", text).unwrap_err();
    err.related().unwrap()
        .map(|e| e.to_string()).collect::<Vec<_>>()
        .join("\n")
}
*/
