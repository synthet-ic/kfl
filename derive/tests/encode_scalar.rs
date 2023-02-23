mod common;

use kfl::{Decode, DecodeScalar, Encode, EncodeScalar};

// #[test]
// fn print_struct_scalar_properties() {
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
fn print_enum_scalar() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        value: SomeScalar,
    }
    #[derive(DecodeScalar, EncodeScalar, Debug, PartialEq)]
    enum SomeScalar {
        First,
        AnotherOption,
    }
    assert_encode!(Node { value: SomeScalar::First },
                   r#"node "first""#);
    assert_encode!(Node { value: SomeScalar::AnotherOption },
                   r#"node "another-option""#);
    // assert_encode_error!(Node,
    //     r#"node "test""#,
    //     "expected one of `first`, `another-option`");
}

#[test]
fn print_option_argument() {
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
