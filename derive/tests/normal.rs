mod common;

use std::{
    collections::BTreeMap,
    default::Default
};
use kfl::Decode;

use common::assert_parse_err;

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Arg1 {
//     #[kfl(argument)]
//     name: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Arg1RawIdent {
//     #[kfl(argument)]
//     r#type: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct ArgDefOptValue {
//     #[kfl(argument, default = Some("unnamed".into()))]
//     name: Option<String>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct OptArg {
//     #[kfl(argument)]
//     name: Option<String>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Extra {
//     field: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq, Default)]
// struct Prop1 {
//     #[kfl(property)]
//     label: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq, Default)]
// struct Prop1RawIdent {
//     #[kfl(property)]
//     r#type: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct PropDefValue {
//     #[kfl(property, default="unknown".into())]
//     label: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct PropDefOptValue {
//     #[kfl(property, default=Some("unknown".into()))]
//     label: Option<String>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct FilteredChildren {
//     #[kfl(children)]
//     left: Vec<OptArg>,
//     #[kfl(children)]
//     right: Vec<OptArg>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Child {
//     #[kfl(child)]
//     main: Prop1,
//     #[kfl(child, default)]
//     extra: Option<Prop1>,
//     #[kfl(child)]
//     flag: bool,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Bytes {
//     #[kfl(argument, bytes)]
//     data: Vec<u8>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct OptBytes {
//     #[kfl(property, bytes)]
//     data: Option<Vec<u8>>,
// }

#[test]
fn parse_argument_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]
        name: String,
    }

    assert_parse!(
        r#"node "hello""#,
        Node { name: "hello".into() });
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument");
    // assert_parse_err::<Node>(
    //     r#"(some)node "hello""#,
    //     "no type name expected for this node");
    assert_parse_err::<Node>(
        r#"node"#,
        "additional argument `name` is required");
}

#[test]
fn parse_argument_unnamed() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node(
        #[kfl(argument)]
        String
    );
    assert_parse!(
        r#"node "hello""#,
        Node("hello".into()));
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument");
    // assert_parse_err::<Node>(
    //     r#"(some)node "hello""#,
    //     "no type name expected for this node");
    assert_parse_err::<Node>(
        r#"node"#,
        "additional argument is required");
}

// #[test]
// fn parse_arg1_raw_ident() {
//     assert_eq!(parse::<Arg1RawIdent>(r#"node "hello""#),
//                Arg1RawIdent { r#type: "hello".into() } );
//     assert_eq!(parse_err::<Arg1RawIdent>(r#"node "hello" "world""#),
//                "unexpected argument");
//     assert_eq!(parse_err::<Arg1RawIdent>(r#"(some)node "hello""#),
//                "no type name expected for this node");
//     assert_eq!(parse_err::<Arg1RawIdent>(r#"node"#),
//                "additional argument `type` is required");
// }


#[test]
fn parse_argument_default_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default)]
        name: String,
    }
    assert_parse!(
        r#"node "hello""#,
        Node { name: "hello".into() });
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_parse!(
        r#"node"#,
        Node { name: "".into() });
}

#[test]
fn parse_argument_default_unnamed() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node(
        #[kfl(argument, default)]
        String,
    );
    assert_parse!(
        r#"node "hello""#,
        Node("hello".into())
    );
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument"
    );
    assert_parse!(
        r#"node"#,
        Node("".into())
    );
}

#[test]
fn parse_argument_default_value_named() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument, default = "unnamed".into())]
        name: String,
    }
    assert_parse!(
        r#"node "hello""#,
        Node { name: "hello".into() }
    );
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument"
    );
    assert_parse!(
        r#"node"#,
        Node { name: "unnamed".into() }
    );

    // assert_eq!(parse::<ArgDefOptValue>(r#"node "hello""#),
    //            ArgDefOptValue { name: Some("hello".into()) } );
    // assert_eq!(parse_err::<ArgDefValue>(r#"node "hello" "world""#),
    //     "unexpected argument");
    // assert_eq!(parse::<ArgDefOptValue>(r#"node"#),
    //            ArgDefOptValue { name: Some("unnamed".into()) } );
    // assert_eq!(parse::<ArgDefOptValue>(r#"node null"#),
    //            ArgDefOptValue { name: None } );
}

#[test]
fn parse_property_named() {
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Node {
        #[kfl(property)]
        name: String,
    }
    assert_parse!(
        r#"node name="hello""#,
        Node { name: "hello".into() });
    assert_parse_err::<Node>(
        r#"node name="hello" y="world""#,
        "unexpected property `y`");
    assert_parse_err::<Node>(
        r#"node"#,
        "property `name` is required");
}

#[test]
fn parse_property_unnamed() {
    #[derive(Decode, Debug, PartialEq, Default)]
    struct Node(
        #[kfl(property(name = "name"))]
        String,
    );
    assert_parse!(
        r#"node name="hello""#,
        Node("hello".into()));
    assert_parse_err::<Node>(
        r#"node name="hello" y="world""#,
        "unexpected property `y`");
    assert_parse_err::<Node>(
        r#"node"#,
        "property `name` is required");
}

// #[test]
// fn parse_prop_raw_ident() {
//     assert_eq!(parse::<Prop1RawIdent>(r#"node type="hello""#),
//                Prop1RawIdent { r#type: "hello".into() } );
//     assert_eq!(parse_err::<Prop1RawIdent>(r#"node type="hello" y="world""#),
//                "unexpected property `y`");
//     assert_eq!(parse_err::<Prop1RawIdent>(r#"node"#),
//                "property `type` is required");
// }

#[test]
fn parse_property_default() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default)]
        name: String,
    }
    assert_parse!(r#"node name="hello""#,
                  Node { name: "hello".into() });
    assert_parse!(r#"node"#,
                  Node { name: "".into() });
}

// #[test]
// fn parse_prop_def_value() {
//     assert_eq!(parse::<PropDefValue>(r#"node label="hello""#),
//                PropDefValue { label: "hello".into() } );
//     assert_eq!(parse::<PropDefValue>(r#"node"#),
//                PropDefValue { label: "unknown".into() });

//     assert_eq!(parse::<PropDefOptValue>(r#"node label="hello""#),
//                PropDefOptValue { label: Some("hello".into()) } );
//     assert_eq!(parse::<PropDefOptValue>(r#"node"#),
//                PropDefOptValue { label: Some("unknown".into()) });
//     assert_eq!(parse::<PropDefOptValue>(r#"node label=null"#),
//                PropDefOptValue { label: None });
// }

#[test]
fn parse_property_name() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property(name = "x"))]
        name: String,
    }
    assert_parse!(r#"node x="hello""#,
                  Node { name: "hello".into() } );
    assert_parse_err::<Node>(r#"node label="hello" y="world""#),
        "unexpected property `label`");
    assert_parse_err::<Node>(r#"node"#),
        "property `x` is required");
}

#[test]
fn parse_option_property() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(property, default)]  /* TODO test without default */
        name: Option<String>,
    }
    assert_parse!(r#"node name="hello""#,
                  Node { name: Some("hello".into()) } );
    assert_parse!(r#"node"#,
                  Node { name: None } );
    assert_parse!(r#"node name=null"#,
                  Node { name: None } );
}

#[test]
fn parse_var_arguments() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(arguments)]
        params: Vec<u64>,
    }
    assert_parse!(r#"node 1 2 3"#,
                  Node { params: vec![1, 2, 3] } );
    assert_parse!(r#"node"#,
                  Node { params: vec![] } );
}

#[test]
fn parse_var_properties() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(properties)]
        scores: BTreeMap<String, u64>,
    }
    let mut scores = BTreeMap::new();
    scores.insert("john".into(), 13);
    scores.insert("jack".into(), 7);
    assert_parse!(r#"node john=13 jack=7"#,
                  Node { scores });
    assert_parse!(r#"node"#,
                  Node { scores: BTreeMap::new() });
}

#[test]
fn parse_children() {
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
    assert_parse!(
        r#"parent { child "val1"; child "val2"; }"#,
        Parent { children: vec![
            Child { name: "val1".into() },
            Child { name: "val2".into() },
        ]}
    );
    assert_parse!(
        r#"parent"#,
        Parent { children: vec![]});

    // assert_eq!(parse_doc::<Parent>(r#"- "val1"; - "val2""#),
    //            Parent { children: vec! [
    //                Child { name: "val1".into() },
    //                Child { name: "val2".into() },
    //            ]} );
    // assert_eq!(parse_doc::<Parent>(r#""#),
    //            Parent { children: Vec::new() } );
}

#[test]
fn parse_filtered_children() {
    #[derive(Decode, Debug, PartialEq)]
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
    assert_parse!(
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
    assert_parse!(
        r#"parent { right; left; }"#,
        Parent {
            lefts: vec![Left { name: None }],
            rights: vec![Right { name: None }]
        }
    );
    assert_parse_err::<Parent>(
        r#"some"#,
        "unexpected node `some`");

    // assert_eq!(parse_doc::<FilteredChildren>(
    //                r#"left "v1"; right "v2"; left "v3""#),
    //            FilteredChildren {
    //                left: vec![
    //                    OptArg { name: Some("v1".into()) },
    //                    OptArg { name: Some("v3".into()) },
    //                ],
    //                right: vec![
    //                    OptArg { name: Some("v2".into()) },
    //                ]
    //            });
    // assert_eq!(parse_doc::<FilteredChildren>(r#"right; left"#),
    //            FilteredChildren {
    //                left: vec![
    //                    OptArg { name: None },
    //                ],
    //                right: vec![
    //                    OptArg { name: None },
    //                ]
    //            });
}

#[test]
fn parse_child() {
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(child)]
        child1: Child1,
        #[kfl(child, default)]
        child2: Option<Child2>,
        // #[kfl(child)]
        // flag: bool,
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
    assert_parse!(
        r#"parent { child1 name="val1"; }"#,
        Parent {
            child1: Child1 { name: "val1".into() },
            child2: None,
            // flag: false,
        });
//     assert_eq!(parse::<Child>(r#"parent {
//                     main label="primary";
//                     extra label="replica";
//                  }"#),
//                Child {
//                    main: Prop1 { label: "primary".into() },
//                    extra: Some(Prop1 { label: "replica".into() }),
//                    flag: false,
//                });
//     assert_eq!(parse_err::<Child>(r#"parent { something; }"#),
//                "unexpected node `something`\n\
//                 child node `main` is required");
//     assert_eq!(parse_err::<Child>(r#"parent"#),
//                "child node `main` is required");

//     assert_eq!(parse_doc::<Child>(r#"main label="val1""#),
//                Child {
//                    main: Prop1 { label: "val1".into() },
//                    extra: None,
//                    flag: false,
//                });
//     assert_eq!(parse_doc::<Child>(r#"
//                     main label="primary"
//                     extra label="replica"
//                     flag
//                  "#),
//                Child {
//                    main: Prop1 { label: "primary".into() },
//                    extra: Some(Prop1 { label: "replica".into() }),
//                    flag: true,
//                });
//     assert_eq!(parse_doc_err::<Child>(r#"something"#),
//                "unexpected node `something`\n\
//                 child node `main` is required");
//     assert_eq!(parse_doc_err::<Child>(r#""#),
//                "child node `main` is required");
}

#[test]
fn parse_child_default() {
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
    assert_parse!(
        r#"parent { child name="val1"; }"#,
        Parent { child: Child { name: "val1".into() } });
    assert_parse!(
        r#"parent"#,
        Parent { child: Child { name: "".into() } });
}

#[test]
fn parse_child_default_value() {
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
    assert_parse!(r#"parent { child label="val1"; }"#,
        Parent { main: Child { label: "val1".into() },
               });
    assert_parse!(r#"parent"#,
        Parent { main: Child { label: "prop1".into() },
               });
}

// #[test]
// fn parse_enum() {
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
//     assert_eq!(parse_err::<Variant>(r#"something"#),
//         "expected one of `arg1`, `prop1`");
// }

#[test]
fn parse_str() {
    #[derive(Decode, Debug, PartialEq)]
    struct Node {
        #[kfl(argument)]  /* str */
        listen: std::net::SocketAddr,
    }
    assert_parse!(r#"node "127.0.0.1:8080""#,
               Node { listen: "127.0.0.1:8080".parse().unwrap() });
    assert_parse_err::<Node>(r#"node "2/3""#,
               "invalid socket address syntax");
}

#[test]
fn parse_option_str() {
    #[derive(Decode, Debug, PartialEq)]
    struct Server {
        #[kfl(property, default)]  /* str */
        listen: Option<std::net::SocketAddr>,  /* TODO strip std::net:: */
    }
    assert_parse!(r#"server listen="127.0.0.1:8080""#,
               Server { listen: Some("127.0.0.1:8080".parse().unwrap()) });
    assert_parse_err::<Server>(r#"server listen="2/3""#,
               "invalid socket address syntax");
    assert_parse!(r#"server listen=null"#,
               Server { listen: None });
}

// #[test]
// fn parse_bytes() {
//     assert_eq!(parse::<Bytes>(r#"bytes (base64)"aGVsbG8=""#),
//                Bytes { data: b"hello".to_vec() });
//     assert_eq!(parse::<Bytes>(r#"bytes "world""#),
//                Bytes { data: b"world".to_vec() });
//     assert_eq!(parse_err::<Bytes>(r#"bytes (base64)"2/3""#),
//         "Invalid padding");

//     assert_eq!(parse::<OptBytes>(r#"node data=(base64)"aGVsbG8=""#),
//                OptBytes { data: Some(b"hello".to_vec()) });
//     assert_eq!(parse::<OptBytes>(r#"node data="world""#),
//                OptBytes { data: Some(b"world".to_vec()) });
//     assert_eq!(parse::<OptBytes>(r#"node data=null"#),
//                OptBytes { data: None });
// }

// #[test]
// fn parse_extra() {
//     assert_eq!(parse::<Extra>(r#"data"#),
//                Extra { field: "".into() });
//     assert_eq!(parse_err::<Extra>(r#"data x=1"#),
//         "unexpected property `x`");
// }
