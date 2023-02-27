mod common;

use std::{
    collections::BTreeMap,
    default::Default,
    net::SocketAddr
};
use kfl::{Decode, DecodePartial};

#[test]
fn decode_argument_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        name: String,
    }
    assert_decode!(
        r#"node "hello""#,
        Node { name: "hello".into() });
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode_error!(Node,
        r#"(some)node "hello""#,
        "no type name expected for this node");
    assert_decode_error!(Node,
        r#"node"#,
        "additional argument `name` is required");
}

#[test]
fn decode_argument_unnamed() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node(
        #[kfl(argument)]
        String
    );
    assert_decode!(
        r#"node "hello""#,
        Node("hello".into()));
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode_error!(Node,
        r#"(some)node "hello""#,
        "no type name expected for this node");
    assert_decode_error!(Node,
        r#"node"#,
        "additional argument is required");
}

#[test]
fn decode_argument_raw_ident() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        r#type: String,
    }
    assert_decode!(r#"node "hello""#,
                   Node { r#type: "hello".into() });
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode_error!(Node,
        r#"(some)node "hello""#,
        "no type name expected for this node");
    assert_decode_error!(Node,
        r#"node"#,
        "additional argument `type` is required");
}

#[test]
fn decode_argument_default_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default)]
        name: String,
    }
    assert_decode!(r#"node "hello""#,
                   Node { name: "hello".into() });
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode!(r#"node"#,
                   Node { name: "".into() });
}

#[test]
fn decode_argument_default_unnamed() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node(
        #[kfl(argument, default)]
        String,
    );
    assert_decode!(
        r#"node "hello""#,
        Node("hello".into()));
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode!(
        r#"node"#,
        Node("".into()));
}

#[test]
fn decode_argument_default_value_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default = "unnamed".into())]
        name: String,
    }
    assert_decode!(
        r#"node "hello""#,
        Node { name: "hello".into() });
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode!(
        r#"node"#,
        Node { name: "unnamed".into() });
}

#[test]
fn decode_argument_default_option_value_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default = Some("unnamed".into()))]
        name: Option<String>,
    }
    assert_decode!(r#"node "hello""#,
                   Node { name: Some("hello".into()) });
    assert_decode_error!(Node,
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_decode!(r#"node"#,
                   Node { name: Some("unnamed".into()) });
    assert_decode!(r#"node null"#,
                   Node { name: None } );
}

#[test]
fn decode_property_named() {
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Node {
        #[kfl(property)]
        name: String,
    }
    assert_decode!(
        r#"node name="hello""#,
        Node { name: "hello".into() });
    assert_decode_error!(Node,
        r#"node name="hello" y="world""#,
        "unexpected property `y`");
    assert_decode_error!(Node,
        r#"node"#,
        "property `name` is required");
}

#[test]
fn decode_property_unnamed() {
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Node(
        #[kfl(property(name = "name"))]
        String,
    );
    assert_decode!(
        r#"node name="hello""#,
        Node("hello".into()));
    assert_decode_error!(Node,
        r#"node name="hello" y="world""#,
        "unexpected property `y`");
    assert_decode_error!(Node,
        r#"node"#,
        "property `name` is required");
}

#[test]
fn decode_property_raw_ident() {
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Node {
        #[kfl(property)]
        r#type: String,
    }
    assert_decode!(r#"node type="hello""#,
                   Node { r#type: "hello".into() });
    assert_decode_error!(Node,
        r#"node type="hello" y="world""#,
        "unexpected property `y`");
    assert_decode_error!(Node,
        r#"node"#,
        "property `type` is required");
}

#[test]
fn decode_property_default() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default)]
        name: String,
    }
    assert_decode!(r#"node name="hello""#,
                   Node { name: "hello".into() });
    assert_decode!(r#"node"#,
                   Node { name: "".into() });
}

#[test]
fn decode_property_default_value() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default="unknown".into())]
        label: String,
    }
    assert_decode!(r#"node label="hello""#,
                   Node { label: "hello".into() } );
    assert_decode!(r#"node"#,
                   Node { label: "unknown".into() });
}

#[test]
fn decode_property_default_option_value() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default = Some("unknown".into()))]
        label: Option<String>,
    }
    assert_decode!(r#"node label="hello""#,
                   Node { label: Some("hello".into()) } );
    assert_decode!(r#"node"#,
                   Node { label: Some("unknown".into()) });
    assert_decode!(r#"node label=null"#,
                   Node { label: None });
}

#[test]
fn decode_property_name() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property(name = "x"))]
        name: String,
    }
    assert_decode!(r#"node x="hello""#,
                   Node { name: "hello".into() });
    assert_decode_error!(Node,
        r#"node label="hello" y="world""#,
        "unexpected property `label`");
    assert_decode_error!(Node,
        r#"node"#,
        "property `x` is required");
}

#[test]
fn decode_option_property() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default)]  /* TODO test without default */
        name: Option<String>,
    }
    assert_decode!(r#"node name="hello""#,
                   Node { name: Some("hello".into()) });
    assert_decode!(r#"node"#,
                   Node { name: None });
    assert_decode!(r#"node name=null"#,
                   Node { name: None });
}

#[test]
fn decode_var_arguments() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(arguments)]
        params: Vec<u64>,
    }
    assert_decode!(r#"node 1 2 3"#,
                   Node { params: vec![1, 2, 3] });
    assert_decode!(r#"node"#,
                   Node { params: vec![] });
}

#[test]
fn decode_var_properties() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(properties)]
        scores: BTreeMap<String, u64>,
    }
    let mut scores = BTreeMap::new();
    scores.insert("john".into(), 13);
    scores.insert("jack".into(), 7);
    assert_decode!(r#"node john=13 jack=7"#,
                   Node { scores });
    assert_decode!(r#"node"#,
                   Node { scores: BTreeMap::new() });
}

#[test]
fn decode_children() {
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(children)]
        children: Vec<Child>,
    }
    #[derive(Decode, Debug, PartialEq)]
    struct Child {
        #[kfl(argument)]
        name: String,
    }
    assert_decode!(
        r#"parent { child "val1"; child "val2"; }"#,
        Parent { children: vec![
            Child { name: "val1".into() },
            Child { name: "val2".into() },
        ]}
    );
    assert_decode!(
        r#"parent"#,
        Parent { children: vec![]});

    // assert_eq!(decode_doc::<Parent>(r#"- "val1"; - "val2""#),
    //            Parent { children: vec! [
    //                Child { name: "val1".into() },
    //                Child { name: "val2".into() },
    //            ]} );
    // assert_eq!(decode_doc::<Parent>(r#""#),
    //            Parent { children: Vec::new() } );
}

#[test]
fn decode_filtered_children() {
    #[derive(DecodePartial, Default, Debug, PartialEq)]
    struct Parent {
        #[kfl(children)]
        lefts: Vec<Left>,
        #[kfl(children)]
        rights: Vec<Right>,
    }
    #[derive(Decode, Debug, PartialEq)]
    struct Left {
        #[kfl(argument, default)]
        name: Option<String>,
    }
    #[derive(Decode, Debug, PartialEq)]
    struct Right {
        #[kfl(argument, default)]
        name: Option<String>,
    }
    assert_decode!(
        r#"parent { left "v1"; right "v2"; left "v3"; }"#,
        Parent {
            lefts: vec![
                Left { name: Some("v1".into()) },
                Left { name: Some("v3".into()) },
            ],
            rights: vec![
                Right { name: Some("v2".into()) },
            ]
        }
    );
    assert_decode_children!(
        r#"left "v1"; right "v2"; left "v3""#,
        Parent {
            lefts: vec![
                Left { name: Some("v1".into()) },
                Left { name: Some("v3".into()) },
            ],
            rights: vec![
                Right { name: Some("v2".into()) },
            ]
        }
    );
    assert_decode!(
        r#"parent { right; left; }"#,
        Parent {
            lefts: vec![Left { name: None }],
            rights: vec![Right { name: None }]
        }
    );
    assert_decode_children!(
        r#"right; left"#,
        Parent {
            lefts: vec![Left { name: None }],
            rights: vec![Right { name: None }]
        }
    );
    assert_decode_error!(Parent,
        r#"some"#,
        "unexpected node `some`");
}

#[test]
fn decode_child() {
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(child)]
        child1: Child1,
        #[kfl(child, default)]
        child2: Option<Child2>,
    }
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Child1 {
        #[kfl(property)]
        name: String,
    }
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Child2 {
        #[kfl(property)]
        name: String,
    }
    #[derive(DecodePartial, Default, Debug, PartialEq)]
    struct ParentPartial {
        #[kfl(child)]
        child1: Option<Child1>,
        #[kfl(child, default)]
        child2: Option<Child2>,
    }
    assert_decode!(
        r#"parent { child1 name="val1"; }"#,
        Parent {
            child1: Child1 { name: "val1".into() },
            child2: None,
        });
    assert_decode!(
        r#"parent {
            child1 name="primary";
            child2 name="replica";
         }"#,
         Parent {
            child1: Child1 { name: "primary".into() },
            child2: Some(Child2 { name: "replica".into() }),
        });
    // TODO(rnarkk)
    // assert_decode_error!(Parent,
    //     r#"parent { something; }"#,
    //     "unexpected node `something`\n\
    //     child node for struct field `child1` is required");
    assert_decode_error!(Parent,
        r#"parent"#,
        "child node for struct field `child1` is required");
    assert_decode_children!(
        r#"child1 name="val1""#,
        ParentPartial {
            child1: Some(Child1 { name: "val1".into() }),
            child2: None,
        });
    assert_decode_children!(
        r#"child1 name="primary"
        child2 name="replica""#,
        ParentPartial {
            child1: Some(Child1 { name: "primary".into() }),
            child2: Some(Child2 { name: "replica".into() }),
        });
    // TODO(rnarkk)
    // assert_decode_children_error!(ParentPartial,
    //     r#"something"#,
    //     "unexpected node `something`\n\
    //     child node for struct field `child1` is required");
    // assert_decode_children_error!(ParentPartial,
    //     r#""#,
    //     "child node for struct field `child1` is required");
}

#[test]
fn decode_child_default() {
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(child, default)]
        child: Child,
    }
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Child {
        #[kfl(property)]
        name: String,
    }
    assert_decode!(
        r#"parent { child name="val1"; }"#,
        Parent { child: Child { name: "val1".into() } });
    assert_decode!(
        r#"parent"#,
        Parent { child: Child { name: "".into() } });
}

#[test]
fn decode_child_default_value() {
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(child, default = Child { label: String::from("prop1") })]
        main: Child,
    }
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Child {
        #[kfl(property)]
        label: String,
    }
    assert_decode!(r#"parent { child label="val1"; }"#,
        Parent { main: Child { label: "val1".into() } });
    assert_decode!(r#"parent"#,
        Parent { main: Child { label: "prop1".into() } });
}

#[test]
fn decode_enum_named() {
    #[derive(Decode, Debug, PartialEq)]
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
    assert_decode!(r#"var0"#, Enum::Var0);
    assert_decode!(r#"var1 "hello""#,
        Enum::Var1 { name: "hello".into() });
    assert_decode!(r#"var2 name="hello""#,
        Enum::Var2 { name: "hello".into() });
    assert_decode_error!(Enum,
        r#"something"#,
        "expected one of `var0`, `var1`, `var2`");
}

#[test]
fn decode_enum_unnamed() {
    #[derive(Decode, Debug, PartialEq)]
    enum Enum {
        Var0,
        Var1(#[kfl(argument)] String),
        Var2(#[kfl(property(name = "name"))] String),
        #[kfl(skip)]
        #[allow(dead_code)]
        Var3(u32),
    }
    assert_decode!(r#"var0"#, Enum::Var0);
    assert_decode!(r#"var1 "hello""#,
        Enum::Var1("hello".into()));
    assert_decode!(r#"var2 name="hello""#,
        Enum::Var2("hello".into()));
    assert_decode_error!(Enum,
        r#"something"#,
        "expected one of `var0`, `var1`, `var2`");
}

// #[test]
// fn decode_enum() {
//     #[derive(Decode, Debug, PartialEq)]
//     enum Variant {
//         Arg1(Arg1),
//         Prop1(Prop1),
//         #[kfl(skip)]
//         #[allow(dead_code)]
//         Var3(u32),
//     }
//     #[derive(Decode, Debug, PartialEq)]
//     struct Arg1 {
//         #[kfl(argument)]
//         name: String,
//     }
//     #[derive(Decode, Debug, PartialEq, Default)]
//     struct Prop1 {
//         #[kfl(property)]
//         label: String,
//     }
//     assert_eq!(parse::<Variant>(r#"arg1 "hello""#),
//                Variant::Arg1(Arg1 { name: "hello".into() }));
//     assert_eq!(parse::<Variant>(r#"prop1 label="hello""#),
//                Variant::Prop1(Prop1 { label: "hello".into() }));
//     assert_eq!(decode_err::<Variant>(r#"something"#),
//         "expected one of `arg1`, `prop1`");
// }

#[test]
fn decode_str() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]  /* str */
        listen: SocketAddr,
    }
    assert_decode!(r#"node 127.0.0.1:8080"#,
               Node { listen: "127.0.0.1:8080".parse().unwrap() });
    assert_decode_error!(Node,
        r#"node "2/3""#,
        "invalid socket address syntax");
}

#[test]
fn decode_option_str() {
    #[derive(Decode, Debug, PartialEq)]
    struct Server {
        #[kfl(property, default)]  /* str */
        listen: Option<SocketAddr>,
    }
    assert_decode!(r#"server listen=127.0.0.1:8080"#,
                   Server { listen: Some("127.0.0.1:8080".parse().unwrap()) });
    assert_decode_error!(Server,
        r#"server listen="2/3""#,
        "invalid socket address syntax");
    assert_decode!(r#"server listen=null"#,
                   Server { listen: None });
}

#[test]
fn decode_bytes() {
    #[derive(Decode, Debug, PartialEq)]
    struct Bytes {
        #[kfl(argument)]  /* bytes */
        data: Vec<u8>,
    }
    assert_decode!(
        r#"bytes (base64)"aGVsbG8=""#,
        Bytes { data: b"hello".to_vec() });
    assert_decode!(
        r#"bytes "world""#,
        Bytes { data: b"world".to_vec() });
    assert_decode_error!(Bytes,
        r#"bytes (base64)"2/3""#,
        "Invalid padding");
}

#[test]
fn decode_option_bytes() {
    #[derive(Decode, Debug, PartialEq)]
    struct Bytes {
        #[kfl(property)]  /* bytes */
        data: Option<Vec<u8>>,
    }
    assert_decode!(
        r#"bytes data=(base64)"aGVsbG8=""#,
        Bytes { data: Some(b"hello".to_vec()) });
    assert_decode!(
        r#"bytes data="world""#,
        Bytes { data: Some(b"world".to_vec()) });
    assert_decode!(
        r#"bytes data=null"#,
        Bytes { data: None });
}

#[test]
fn decode_extra() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        field: String,
    }
    assert_decode!(
        r#"node"#,
        Node { field: "".into() });
    assert_decode_error!(Node,
        r#"node x=1"#,
        "unexpected property `x`");
}
