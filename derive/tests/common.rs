use kfl::{Decode, DecodeChildren, span::Span};

// fn parse<T: Decode<Span>>(text: &str) -> T {
//     let mut nodes: Vec<T> = kfl::parse("<test>", text).unwrap();
//     assert_eq!(nodes.len(), 1);
//     nodes.remove(0)
// }

pub fn assert_parse<T>(input: &str, output: T)
    where T: Decode<Span> + fmt::Debug + PartialEq
{
    let mut nodes: Vec<T> = kfl::parse("<test>", input).unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes.remove(0), output);
}

// fn parse_err<T: Decode<Span>+fmt::Debug>(text: &str) -> String {
//     let err = kfl::parse::<Vec<T>>("<test>", text).unwrap_err();
//     err.related().unwrap()
//         .map(|e| e.to_string()).collect::<Vec<_>>()
//         .join("\n")
// }

pub fn assert_parse_err<T>(input: &str, output: &str)
    where T: Decode<Span> + fmt::Debug + PartialEq
{
    let err = kfl::parse::<Vec<T>>("<test>", input).unwrap_err()
        .related().unwrap()
        .map(|e| e.to_string()).collect::<Vec<_>>()
        .join("\n");
    assert_eq!(err, output);
}

pub fn parse_doc<T: DecodeChildren<Span>>(text: &str) -> T {
    kfl::parse("<test>", text).unwrap()
}

pub fn parse_doc_err<T: DecodeChildren<Span>+fmt::Debug>(text: &str) -> String {
    let err = kfl::parse::<T>("<test>", text).unwrap_err();
    err.related().unwrap()
        .map(|e| e.to_string()).collect::<Vec<_>>()
        .join("\n")
}
