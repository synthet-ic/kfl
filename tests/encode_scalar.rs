mod common;

use kfl::{Decode, DecodeScalar, Encode, EncodeScalar};

// #[test]
// fn encode_struct_scalar_properties() {
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
//     assert_encode!(r#"node label="hello""#,
//                    Node { props: Prop1 { label: Some("hello".into()) } } );
//     assert_encode_error!(Node, r#"node something="world""#,
//                          "unexpected property `something`");
// }

#[test]
fn encode_enum_scalar() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        value: SomeScalar,
    }
    #[derive(Clone, DecodeScalar, EncodeScalar, Debug, PartialEq)]
    enum SomeScalar {
        First,
        AnotherOption,
    }
    assert_encode!(Node { value: SomeScalar::First },
                   r#"node first"#);
    assert_encode!(Node { value: SomeScalar::AnotherOption },
                   r#"node another-option"#);
    // assert_encode_error!(Node,
    //     r#"node "test""#,
    //     "expected one of `first`, `another-option`");
}

#[test]
fn encode_option_argument() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        name: Option<String>,
    }
    assert_encode!(Node { name: Some("hello".into()) },
                   r#"node "hello""#);
    // TODO(rnarkk) should fail since no `default` directive
    // assert_encode!(
    //     r#"node"#,
    //     Node { name: None });
    assert_encode!(Node { name: None },
                   r#"node null"#);
}
