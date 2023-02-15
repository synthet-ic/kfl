use std::fmt::Debug;
use kfl::{
    Decode, DecodeChildren,
    span::Span
};
use miette::Diagnostic;

pub fn parse<T: Decode<Span>>(input: &str) -> T {
    let mut nodes: Vec<T> = kfl::parse("<test>", input).unwrap();
    assert_eq!(nodes.len(), 1);
    nodes.remove(0)
}

fn hint_same_type<T: PartialEq>(lhs: &T, rhs: &T) {}

#[macro_export]
macro_rules! assert_parse {
    ($input:literal, $output:expr) => {
        let mut node = kfl::decode("<test>", $input).unwrap();
        common::hint_same_type(&node, &$output);
        assert!(node == $output);
    }
}

pub fn parse_err<T: Decode<Span> + Debug>(input: &str) -> String {
    let err = kfl::parse::<Vec<T>>("<test>", input).unwrap_err();
    err.related().unwrap()
        .map(|e| e.to_string()).collect::<Vec<_>>()
        .join("\n")
}

pub fn assert_parse_err<T>(input: &str, output: &str)
    where T: Decode<Span> + Debug + PartialEq
{
    let err = kfl::parse::<Vec<T>>("<test>", input).unwrap_err()
        .related().unwrap()
        .map(|e| e.to_string()).collect::<Vec<_>>()
        .join("\n");
    assert_eq!(err, output);
}

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
