mod common;

use std::fmt::Debug;
use kfl::Decode;
// use miette::Diagnostic;

#[test]
fn parse_unit() {
    #[derive(Debug, Decode, PartialEq)]
    struct Node;
    assert_decode!(r#"node"#, Node);
    assert_decode_error!(Node,
        r#"node something="world""#,
        "unexpected property `something`");
}

#[test]
fn parse_argument() {
    #[derive(Debug, Decode, PartialEq)]
    struct Node(#[kfl(argument)] u32);
    assert_decode!(r#"node 123"#, Node(123));
    assert_decode_error!(Node,
        r#"node something="world""#,
        "additional argument is required");
}

#[test]
fn parse_option_argument() {
    #[derive(Debug, Decode, PartialEq)]
    struct Node(#[kfl(argument, default)] Option<u32>);
    assert_decode!(r#"node 123"#, Node(Some(123)));
    assert_decode!(r#"node"#, Node(None));
    assert_decode_error!(Node,
        r#"node something="world""#,
        "unexpected property `something`");
}

#[test]
fn parse_extra() {
    #[derive(Debug, Decode, PartialEq)]
    struct Node(#[kfl(argument, default)] Option<String>, u32);
    assert_decode!(r#"node "123""#,
                   Node(Some("123".into()), 0));
    assert_decode!(r#"node"#,
                   Node(None, 0));
    assert_decode_error!(Node,
        r#"node "123" 456"#,
        "unexpected argument");
}

#[test]
fn parse_enum() {
    #[derive(Debug, Decode, PartialEq)]
    enum Enum {
        Unit,
        Arg(#[kfl(argument)] u32),
        // Opt(Option<Arg>),
        Extra(#[kfl(argument, default)] Option<String>, u32),
    }
    assert_decode!(r#"unit"#, Enum::Unit);
    assert_decode!(r#"arg 123"#, Enum::Arg(123));
    // assert_decode!(r#"opt 123"#, Enum::Opt(Some(Arg(123))));
    // assert_decode!(r#"opt"#, Enum::Opt(None));
    assert_decode!(r#"extra"#, Enum::Extra(None, 0));
    assert_decode_error!(Enum,
        r#"unit something="world""#,
        "unexpected property `something`");
    assert_decode_error!(Enum,
        r#"other something="world""#,
        "expected one of `unit`, `arg`, `extra`");
    assert_decode_error!(Enum,
        r#"extra "hello" "world""#,
        "unexpected argument");
}
