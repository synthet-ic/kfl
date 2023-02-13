mod common;

use kfl::{Decode, DecodeScalar};
use common::{assert_parse, assert_parse_err};

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
