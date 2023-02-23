mod common;

use std::{
    collections::BTreeMap,
//     default::Default,
//     net::SocketAddr
};
use kfl::{Decode, Encode};

#[test]
fn print_argument_named() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        name: String,
    }
    assert_encode!(
        Node { name: "hello".into() },
        r#"node "hello""#);
    // assert_encode_error!(Node,
    //     r#"node "hello" "world""#,
    //     "unexpected argument"
    // );
    // assert_encode_error!(Node,
    //     r#"(some)node "hello""#,
    //     "no type name expected for this node"
    // );
    // assert_encode_error!(Node,
    //     r#"node"#,
    //     "additional argument `name` is required"
    // );
}

#[test]
fn print_argument_unnamed() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node(
        #[kfl(argument)]
        String
    );
    assert_encode!(
        Node("hello".into()),
        r#"node "hello""#);
//     assert_encode_error!(Node,
//         r#"node "hello" "world""#,
//         "unexpected argument");
//     assert_encode_error!(Node,
//         r#"(some)node "hello""#,
//         "no type name expected for this node");
//     assert_encode_error!(Node,
//         r#"node"#,
//         "additional argument is required");
}

#[test]
fn print_argument_raw_ident() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        r#type: String,
    }
    assert_encode!(Node { r#type: "hello".into() },
                   r#"node "hello""#);
//     assert_encode_error!(Node,
//         r#"node "hello" "world""#,
//         "unexpected argument");
//     assert_encode_error!(Node,
//         r#"(some)node "hello""#,
//         "no type name expected for this node");
//     assert_encode_error!(Node,
//         r#"node"#,
//         "additional argument `type` is required");
}

#[test]
fn print_argument_default_named() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default)]
        name: String,
    }
    assert_encode!(Node { name: "hello".into() },
                   r#"node "hello""#);
//     assert_encode_error!(Node,
//         r#"node "hello" "world""#,
//         "unexpected argument");
//     assert_encode!(r#"node"#,
//                    Node { name: "".into() });
}

#[test]
fn print_argument_default_unnamed() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node(
        #[kfl(argument, default)]
        String,
    );
    assert_encode!(
        Node("hello".into()),
        r#"node "hello""#);
//     assert_encode_error!(Node,
//         r#"node "hello" "world""#,
//         "unexpected argument");
    assert_encode!(
        Node("".into()),
        r#"node"#);
}

#[test]
fn print_argument_default_value_named() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default = "unnamed".into())]
        name: String,
    }
    assert_encode!(
        Node { name: "hello".into() },
        r#"node "hello""#);
//     assert_encode_error!(Node,
//         r#"node "hello" "world""#,
//         "unexpected argument");
    assert_encode!(
        Node { name: "unnamed".into() },
        r#"node"#);
}

#[test]
fn print_argument_default_option_value_named() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default = Some("unnamed".into()))]
        name: Option<String>,
    }
    assert_encode!(Node { name: Some("hello".into()) },
                   r#"node "hello""#);
//     assert_encode_error!(Node,
//         r#"node "hello" "world""#,
//         "unexpected argument");
    assert_encode!(Node { name: Some("unnamed".into()) },
                   r#"node"#);
    assert_encode!(Node { name: None },
                   r#"node null"#);
}

#[test]
fn print_property_named() {
    #[derive(Decode, Encode, Debug, PartialEq, Default)]
    struct Node {
        #[kfl(property)]
        name: String,
    }
    assert_encode!(
        Node { name: "hello".into() },
        r#"node name="hello""#);
//     assert_encode_error!(Node,
//         r#"node name="hello" y="world""#,
//         "unexpected property `y`");
//     assert_encode_error!(Node,
//         r#"node"#,
//         "property `name` is required");
}

#[test]
fn print_property_unnamed() {
    #[derive(Decode, Encode, Debug, PartialEq, Default)]
    struct Node(
        #[kfl(property(name = "name"))]
        String,
    );
    assert_encode!(
        Node("hello".into()),
        r#"node name="hello""#);
//     assert_encode_error!(Node,
//         r#"node name="hello" y="world""#,
//         "unexpected property `y`");
//     assert_encode_error!(Node,
//         r#"node"#,
//         "property `name` is required");
}

#[test]
fn print_property_raw_ident() {
    #[derive(Decode, Encode, Debug, PartialEq, Default)]
    struct Node {
        #[kfl(property)]
        r#type: String,
    }
    assert_encode!(Node { r#type: "hello".into() },
                   r#"node type="hello""#);
//     assert_encode_error!(Node,
//         r#"node type="hello" y="world""#,
//         "unexpected property `y`");
//     assert_encode_error!(Node,
//         r#"node"#,
//         "property `type` is required");
}

#[test]
fn print_property_default() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default)]
        name: String,
    }
    assert_encode!(Node { name: "hello".into() },
                   r#"node name="hello""#);
    assert_encode!(Node { name: "".into() },
                   r#"node"#);
}

#[test]
fn print_property_default_value() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default="unknown".into())]
        label: String,
    }
    assert_encode!(Node { label: "hello".into() },
                   r#"node label="hello""#);
    assert_encode!(Node { label: "unknown".into() },
                   r#"node"#);
}

#[test]
fn print_property_default_option_value() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default = Some("unknown".into()))]
        label: Option<String>,
    }
    assert_encode!(Node { label: Some("hello".into()) },
                   r#"node label="hello""#);
    assert_encode!(Node { label: Some("unknown".into()) },
                   r#"node"#);
    assert_encode!(Node { label: None },
                   r#"node label=null"#);
}

#[test]
fn print_property_name() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(property(name = "x"))]
        name: String,
    }
    assert_encode!(Node { name: "hello".into() },
                   r#"node x="hello""#);
//     assert_encode_error!(Node,
//         r#"node label="hello" y="world""#,
//         "unexpected property `label`");
//     assert_encode_error!(Node,
//         r#"node"#,
//         "property `x` is required");
}

#[test]
fn print_option_property() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default)]  /* TODO test without default */
        name: Option<String>,
    }
    assert_encode!(Node { name: Some("hello".into()) },
                   r#"node name="hello""#);
    assert_encode!(Node { name: None },
                   r#"node"#);
    // assert_encode!(Node { name: None },
    //                r#"node name=null"#);
}

#[test]
fn print_var_arguments() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(arguments)]
        params: Vec<u64>,
    }
    assert_encode!(Node { params: vec![1, 2, 3] },
                   r#"node 1 2 3"#);
    assert_encode!(Node { params: vec![] },
                   r#"node"#);
}

#[test]
fn print_var_properties() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Node {
        #[kfl(properties)]
        scores: BTreeMap<String, u64>,
    }
    // let mut scores = BTreeMap::new();
    // scores.insert("john".into(), 13);
    // scores.insert("jack".into(), 7);
    // assert_encode!(Node { scores },
    //                r#"node john=13 jack=7"#);
    assert_encode!(Node { scores: BTreeMap::new() },
                   r#"node"#);
}

#[test]
fn print_children() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Parent {
        #[kfl(children)]
        children: Vec<Child>,
    }
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Child {
        #[kfl(argument)]
        name: String,
    }
    // assert_encode!(
    //     Parent { children: vec![
    //         Child { name: "val1".into() },
    //         Child { name: "val2".into() },
    //     ]},
    //     r#"parent { child "val1"; child "val2"; }"#
    // );
    assert_encode!(
        Parent { children: vec![]},
        r#"parent"#);

//     // assert_eq!(parse_doc::<Parent>(r#"- "val1"; - "val2""#),
//     //            Parent { children: vec! [
//     //                Child { name: "val1".into() },
//     //                Child { name: "val2".into() },
//     //            ]} );
//     // assert_eq!(parse_doc::<Parent>(r#""#),
//     //            Parent { children: Vec::new() } );
}

// #[test]
// fn print_filtered_children() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Parent {
//         #[kfl(children)]
//         lefts: Vec<Left>,
//         #[kfl(children)]
//         rights: Vec<Right>,
//     }
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Left {
//         #[kfl(argument, default)]
//         name: Option<String>,
//     }
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Right {
//         #[kfl(argument, default)]
//         name: Option<String>,
//     }
//     assert_encode!(
//         r#"parent { left "v1"; right "v2"; left "v3"; }"#,
//         Parent {
//             lefts: vec![
//                 Left { name: Some("v1".into()) },
//                 Left { name: Some("v3".into()) },
//             ],
//             rights: vec![
//                 Right { name: Some("v2".into()) },
//             ]
//         }
//     );
//     assert_decode_children!(
//         r#"left "v1"; right "v2"; left "v3""#,
//         Parent {
//             lefts: vec![
//                 Left { name: Some("v1".into()) },
//                 Left { name: Some("v3".into()) },
//             ],
//             rights: vec![
//                 Right { name: Some("v2".into()) },
//             ]
//         }
//     );
//     assert_encode!(
//         r#"parent { right; left; }"#,
//         Parent {
//             lefts: vec![Left { name: None }],
//             rights: vec![Right { name: None }]
//         }
//     );
//     assert_decode_children!(
//         r#"right; left"#,
//         Parent {
//             lefts: vec![Left { name: None }],
//             rights: vec![Right { name: None }]
//         }
//     );
//     assert_encode_error!(Parent,
//         r#"some"#,
//         "unexpected node `some`");
// }

// #[test]
// fn print_child() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Parent {
//         #[kfl(child)]
//         child1: Child1,
//         #[kfl(child, default)]
//         child2: Option<Child2>,
//     }
//     #[derive(Decode, Encode, Debug, PartialEq, Default)]
//     struct Child1 {
//         #[kfl(property)]
//         name: String,
//     }
//     #[derive(Decode, Encode, Debug, PartialEq, Default)]
//     struct Child2 {
//         #[kfl(property)]
//         name: String,
//     }
//     assert_encode!(
//         r#"parent { child1 name="val1"; }"#,
//         Parent {
//             child1: Child1 { name: "val1".into() },
//             child2: None,
//         });
//     assert_encode!(
//         r#"parent {
//             child1 name="primary";
//             child2 name="replica";
//          }"#,
//          Parent {
//             child1: Child1 { name: "primary".into() },
//             child2: Some(Child2 { name: "replica".into() }),
//         });
//     // TODO(rnarkk)
//     // assert_encode_error!(Parent,
//     //     r#"parent { something; }"#,
//     //     "unexpected node `something`\n\
//     //     child node for struct field `child1` is required");
//     assert_encode_error!(Parent,
//         r#"parent"#,
//         "child node for struct field `child1` is required");
//     assert_decode_children!(
//         r#"child1 name="val1""#,
//         Parent {
//             child1: Child1 { name: "val1".into() },
//             child2: None,
//         });
//     assert_decode_children!(
//         r#"child1 name="primary"
//         child2 name="replica""#,
//         Parent {
//             child1: Child1 { name: "primary".into() },
//             child2: Some(Child2 { name: "replica".into() }),
//         });
//     assert_decode_children_error!(Parent,
//         r#"something"#,
//         "unexpected node `something`\n\
//         child node for struct field `child1` is required");
//     assert_decode_children_error!(Parent,
//         r#""#,
//         "child node for struct field `child1` is required");
// }

#[test]
fn print_child_default() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Parent {
        #[kfl(child, default)]
        child: Child,
    }
    #[derive(Decode, Encode, Debug, PartialEq, Default)]
    struct Child {
        #[kfl(property)]
        name: String,
    }
//     assert_encode!(
//         r#"parent { child name="val1"; }"#,
//         Parent { child: Child { name: "val1".into() } });
    assert_encode!(
        Parent { child: Child { name: "".into() } },
        r#"parent"#);
}

#[test]
fn print_child_default_value() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    struct Parent {
        #[kfl(child, default = Child { label: String::from("prop1") })]
        main: Child,
    }
    #[derive(Decode, Encode, Debug, PartialEq, Default)]
    struct Child {
        #[kfl(property)]
        label: String,
    }
//     assert_encode!(r#"parent { child label="val1"; }"#,
//         Parent { main: Child { label: "val1".into() } });
    assert_encode!(
        Parent { main: Child { label: "prop1".into() } },
        r#"parent"#);
}

#[test]
fn print_enum_named() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    enum Enum {
        Var0,
        Var1 {
            #[kfl(argument)]
            name: String,
        },
        Var2 {
            #[kfl(property)]
            name: String,
        },
        #[kfl(skip)]
        #[allow(dead_code)]
        Var3(u32),
    }
    assert_encode!(Enum::Var0, r#"var0"#);
    assert_encode!(Enum::Var1 { name: "hello".into() },
                   r#"var1 "hello""#);
    assert_encode!(Enum::Var2 { name: "hello".into() },
                   r#"var2 name="hello""#);
//     assert_encode_error!(Enum,
//         r#"something"#,
//         "expected one of `var0`, `var1`, `var2`");
}

#[test]
fn print_enum_unnamed() {
    #[derive(Decode, Encode, Debug, PartialEq)]
    enum Enum {
        Var0,
        Var1(#[kfl(argument)] String),
        Var2(#[kfl(property(name = "name"))] String),
        #[kfl(skip)]
        #[allow(dead_code)]
        Var3(u32),
    }
    assert_encode!(Enum::Var0, r#"var0"#);
    assert_encode!(Enum::Var1("hello".into()),
                   r#"var1 "hello""#);
    assert_encode!(Enum::Var2("hello".into()),
                   r#"var2 name="hello""#);
    // assert_encode_error!(Enum,
    //     r#"something"#,
    //     "expected one of `var0`, `var1`, `var2`");
}

// // #[test]
// // fn print_enum() {
// //     #[derive(Decode, Encode, Debug, PartialEq)]
// //     enum Variant {
// //         Arg1(Arg1),
// //         Prop1(Prop1),
// //         #[kfl(skip)]
// //         #[allow(dead_code)]
// //         Var3(u32),
// //     }
// //     #[derive(Decode, Encode, Debug, PartialEq)]
// //     struct Arg1 {
// //         #[kfl(argument)]
// //         name: String,
// //     }
// //     #[derive(Decode, Encode, Debug, PartialEq, Default)]
// //     struct Prop1 {
// //         #[kfl(property)]
// //         label: String,
// //     }
// //     assert_eq!(parse::<Variant>(r#"arg1 "hello""#),
// //                Variant::Arg1(Arg1 { name: "hello".into() }));
// //     assert_eq!(parse::<Variant>(r#"prop1 label="hello""#),
// //                Variant::Prop1(Prop1 { label: "hello".into() }));
// //     assert_eq!(parse_err::<Variant>(r#"something"#),
// //         "expected one of `arg1`, `prop1`");
// // }

// #[test]
// fn print_str() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Node {
//         #[kfl(argument)]  /* str */
//         listen: SocketAddr,
//     }
//     assert_encode!(r#"node "127.0.0.1:8080""#,
//                Node { listen: "127.0.0.1:8080".parse().unwrap() });
//     assert_encode_error!(Node,
//         r#"node "2/3""#,
//         "invalid socket address syntax");
// }

// #[test]
// fn print_option_str() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Server {
//         #[kfl(property, default)]  /* str */
//         listen: Option<SocketAddr>,
//     }
//     assert_encode!(r#"server listen="127.0.0.1:8080""#,
//                    Server { listen: Some("127.0.0.1:8080".parse().unwrap()) });
//     assert_encode_error!(Server,
//         r#"server listen="2/3""#,
//         "invalid socket address syntax");
//     assert_encode!(r#"server listen=null"#,
//                    Server { listen: None });
// }

// #[test]
// fn print_bytes() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Bytes {
//         #[kfl(argument)]  /* bytes */
//         data: Vec<u8>,
//     }
//     assert_encode!(
//         r#"bytes (base64)"aGVsbG8=""#,
//         Bytes { data: b"hello".to_vec() });
//     assert_encode!(
//         r#"bytes "world""#,
//         Bytes { data: b"world".to_vec() });
//     assert_encode_error!(Bytes,
//         r#"bytes (base64)"2/3""#,
//         "Invalid padding");
// }

// #[test]
// fn print_option_bytes() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Bytes {
//         #[kfl(property)]  /* bytes */
//         data: Option<Vec<u8>>,
//     }
//     assert_encode!(
//         r#"bytes data=(base64)"aGVsbG8=""#,
//         Bytes { data: Some(b"hello".to_vec()) });
//     assert_encode!(
//         r#"bytes data="world""#,
//         Bytes { data: Some(b"world".to_vec()) });
//     assert_encode!(
//         r#"bytes data=null"#,
//         Bytes { data: None });
// }

// #[test]
// fn print_extra() {
//     #[derive(Decode, Encode, Debug, PartialEq)]
//     struct Node {
//         field: String,
//     }
//     assert_encode!(
//         r#"node"#,
//         Node { field: "".into() });
//     assert_encode_error!(Node,
//         r#"node x=1"#,
//         "unexpected property `x`");
// }
