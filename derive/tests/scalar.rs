mod common;

use kfl::{Decode, DecodeScalar};

// #[test]
// fn parse_struct_scalar_properties() {
//     #[derive(Decode, Debug, PartialEq)]
//     struct Node {
//         #[kfl(flatten)]
//         props: Props
//     }
//     #[derive(Decode, Default, Debug, PartialEq)]
//     struct Props {
//         #[kfl(property)]
//         label: Option<String>,
//     }
//     assert_decode!(r#"node label="hello""#,
//                    Node { props: Prop1 { label: Some("hello".into()) } } );
//     assert_decode_error!(Node, r#"node something="world""#,
//                          "unexpected property `something`");
// }

#[test]
fn parse_enum_scalar() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        value: SomeScalar,
    }
    #[derive(Clone, DecodeScalar, Debug, PartialEq)]
    enum SomeScalar {
        First,
        AnotherOption,
    }
    assert_decode!(
        r#"node "first""#,
        Node { value: SomeScalar::First });
    assert_decode!(
        r#"node "another-option""#,
        Node { value: SomeScalar::AnotherOption });
    assert_decode_error!(Node,
        r#"node "test""#,
        "expected one of `first`, `another-option`");
}

#[test]
fn parse_option_argument() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        name: Option<String>,
    }
    assert_decode!(
        r#"node "hello""#,
        Node { name: Some("hello".into()) });
    // TODO(rnarkk) should fail since no `default` directive
    // assert_decode!(
    //     r#"node"#,
    //     Node { name: None });
    assert_decode!(
        r#"node null"#,
        Node { name: None });
}
