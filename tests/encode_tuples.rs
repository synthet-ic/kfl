mod common;

use kfl::{Decode, Encode};

#[test]
fn print_unit() {
    #[derive(Debug, Decode, Encode, PartialEq)]
    struct Node;
    assert_encode!(Node, r#"node"#);
    // assert_encode_error!(Node,
    //     r#"node something="world""#,
    //     "unexpected property `something`");
}

#[test]
fn print_argument() {
    #[derive(Debug, Decode, Encode, PartialEq)]
    struct Node(#[kfl(argument)] u32);
    assert_encode!(Node(123), r#"node 123"#);
    // assert_encode_error!(Node,
    //     r#"node something="world""#,
    //     "additional argument is required");
}

#[test]
fn print_option_argument() {
    #[derive(Debug, Decode, Encode, PartialEq)]
    struct Node(#[kfl(argument, default)] Option<u32>);
    assert_encode!(Node(Some(123)), r#"node 123"#);
    assert_encode!(Node(None), r#"node"#);
    // assert_encode_error!(Node,
    //     r#"node something="world""#,
    //     "unexpected property `something`");
}

#[test]
fn print_extra() {
    #[derive(Debug, Decode, Encode, PartialEq)]
    struct Node(#[kfl(argument, default)] Option<String>, u32);
    assert_encode!(Node(Some("123".into()), 0),
                   r#"node "123""#);
    assert_encode!(Node(None, 0),
                   r#"node"#);
    // assert_encode_error!(Node,
    //     r#"node "123" 456"#,
    //     "unexpected argument");
}

#[test]
fn print_enum() {
    #[derive(Debug, Decode, Encode, PartialEq)]
    enum Enum {
        Unit,
        Arg(#[kfl(argument)] u32),
        // Opt(Option<Arg>),
        Extra(#[kfl(argument, default)] Option<String>, u32),
    }
    assert_encode!(Enum::Unit, r#"unit"#);
    assert_encode!(Enum::Arg(123), r#"arg 123"#);
    // assert_encode!(r#"opt 123"#, Enum::Opt(Some(Arg(123))));
    // assert_encode!(r#"opt"#, Enum::Opt(None));
    assert_encode!(Enum::Extra(None, 0), r#"extra"#);
    // assert_encode_error!(Enum,
    //     r#"unit something="world""#,
    //     "unexpected property `something`");
    // assert_encode_error!(Enum,
    //     r#"other something="world""#,
    //     "expected one of `unit`, `arg`, `extra`");
    // assert_encode_error!(Enum,
    //     r#"extra "hello" "world""#,
    //     "unexpected argument");
}
