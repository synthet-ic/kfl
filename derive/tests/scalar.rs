mod common;

use kfl::{Decode, DecodeScalar};
use common::{assert_parse, assert_parse_err};

// #[derive(Decode, Default, Debug, PartialEq)]
// struct Prop1 {
//     #[kfl(property)]
//     label: Option<String>,
// }

// #[derive(Decode, Debug, PartialEq)]
// struct FlatProp {
//     #[kfl(flatten(property))]
//     props: Prop1,
// }

// #[test]
// fn parse_flat_prop() {
//     assert_eq!(parse::<FlatProp>(r#"node label="hello""#),
//         FlatProp { props: Prop1 { label: Some("hello".into()) } } );
//     assert_eq!(parse_err::<FlatProp>(r#"node something="world""#),
//         "unexpected property `something`");
// }

#[derive(DecodeScalar, Debug, PartialEq)]
enum SomeScalar {
    First,
    AnotherOption,
}

#[derive(Decode, Debug, PartialEq)]
struct Node {
    #[kfl(argument)]
    value: SomeScalar,
}

#[test]
fn parse_some_scalar() {
    assert_parse::<Node>(
        r#"node "first""#,
        Node { value: SomeScalar::First });
    assert_parse::<Node>(
        r#"node "another-option""#,
        Node { value: SomeScalar::AnotherOption });
    assert_parse_err::<Node>(
        r#"node "test""#,
        "expected one of `first`, `another-option`");
}
