mod common;

use std::{
    // collections::BTreeMap,
    default::Default
};
use kfl::Decode;

use common::{assert_parse, assert_parse_err};

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

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct VarArg {
//     #[kfl(arguments)]
//     params: Vec<u64>,
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
// struct PropDef {
//     #[kfl(property, default)]
//     label: String,
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
// struct PropNamed {
//     #[kfl(property(name="x"))]
//     label: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct OptProp {
//     #[kfl(property)]
//     label: Option<String>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct VarProp {
//     #[kfl(properties)]
//     scores: BTreeMap<String, u64>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct FilteredChildren {
//     #[kfl(children)]
//     left: Vec<OptArg>,
//     #[kfl(children)]
//     right: Vec<OptArg>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// enum Variant {
//     Arg1(Arg1),
//     Prop1(Prop1),
//     #[kfl(skip)]
//     #[allow(dead_code)]
//     Var3(u32),
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
// struct ChildDef {
//     #[kfl(child, default)]
//     main: Prop1,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct ChildDefValue {
//     #[kfl(child, default=Prop1 { label: String::from("prop1") })]
//     main: Prop1,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct ParseOpt {
//     #[kfl(property, str)]
//     listen: Option<std::net::SocketAddr>,
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

    assert_parse::<Node>(
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

    assert_parse::<Node>(
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

    assert_parse::<Node>(
        r#"node "hello""#,
        Node { name: "hello".into() });
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument");
    assert_parse::<Node>(
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

    assert_parse::<Node>(
        r#"node "hello""#,
        Node("hello".into())
    );
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument"
    );
    assert_parse::<Node>(
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

    assert_parse::<Node>(
        r#"node "hello""#,
        Node { name: "hello".into() }
    );
    assert_parse_err::<Node>(
        r#"node "hello" "world""#,
        "unexpected argument"
    );
    assert_parse::<Node>(
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

    assert_parse::<Node>(
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

    assert_parse::<Node>(
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

// #[test]
// fn parse_prop_default() {
//     assert_eq!(parse::<PropDef>(r#"node label="hello""#),
//                PropDef { label: "hello".into() } );
//     assert_eq!(parse::<PropDef>(r#"node"#),
//                PropDef { label: "".into() });
// }

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

// #[test]
// fn parse_prop_named() {
//     assert_eq!(parse::<PropNamed>(r#"node x="hello""#),
//                PropNamed { label: "hello".into() } );
//     assert_eq!(parse_err::<PropNamed>(r#"node label="hello" y="world""#),
//         "unexpected property `label`");
//     assert_eq!(parse_err::<PropNamed>(r#"node"#),
//         "property `x` is required");
// }

// #[test]
// fn parse_opt_prop() {
//     assert_eq!(parse::<OptProp>(r#"node label="hello""#),
//                OptProp { label: Some("hello".into()) } );
//     assert_eq!(parse::<OptProp>(r#"node"#),
//                OptProp { label: None } );
//     assert_eq!(parse::<OptProp>(r#"node label=null"#),
//                OptProp { label: None } );
// }

// #[test]
// fn parse_var_arg() {
//     assert_eq!(parse::<VarArg>(r#"sum 1 2 3"#),
//                VarArg { params: vec![1, 2, 3] } );
//     assert_eq!(parse::<VarArg>(r#"sum"#),
//                VarArg { params: vec![] } );
// }

// #[test]
// fn parse_var_prop() {
//     let mut scores = BTreeMap::new();
//     scores.insert("john".into(), 13);
//     scores.insert("jack".into(), 7);
//     assert_eq!(parse::<VarProp>(r#"scores john=13 jack=7"#),
//                VarProp { scores } );
//     assert_eq!(parse::<VarProp>(r#"scores"#),
//                VarProp { scores: BTreeMap::new() } );
// }

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

    assert_parse::<Parent>(
        r#"parent { child "val1"; child "val2"; }"#,
        Parent { children: vec![
            Child { name: "val1".into() },
            Child { name: "val2".into() },
        ]}
    );
    assert_parse::<Parent>(
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

    assert_parse::<Parent>(
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
    assert_parse::<Parent>(
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

    assert_parse::<Parent>(
        r#"parent { child1 name="val1"; }"#,
        Parent {
            child1: Child1 { name: "val1".into() },
            child2: None,
            // flag: false,
        }
    );
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

// #[test]
// fn parse_child_def() {
//     assert_eq!(parse::<ChildDef>(r#"parent { main label="val1"; }"#),
//                ChildDef {
//                    main: Prop1 { label: "val1".into() },
//                });
//     assert_eq!(parse::<ChildDef>(r#"parent"#),
//                ChildDef {
//                    main: Prop1 { label: "".into() },
//                });
// }

// #[test]
// fn parse_child_def_value() {
//     assert_eq!(parse::<ChildDefValue>(r#"parent { main label="val1"; }"#),
//                ChildDefValue {
//                    main: Prop1 { label: "val1".into() },
//                });
//     assert_eq!(parse::<ChildDefValue>(r#"parent"#),
//                ChildDefValue {
//                    main: Prop1 { label: "prop1".into() },
//                });
// }

// #[test]
// fn parse_enum() {
//     assert_eq!(parse::<Variant>(r#"arg1 "hello""#),
//                Variant::Arg1(Arg1 { name: "hello".into() }));
//     assert_eq!(parse::<Variant>(r#"prop1 label="hello""#),
//                Variant::Prop1(Prop1 { label: "hello".into() }));
//     assert_eq!(parse_err::<Variant>(r#"something"#),
//         "expected one of `arg1`, `prop1`");
// }

// #[test]
// fn parse_str() {
//     assert_eq!(parse_doc::<Parse>(r#"listen "127.0.0.1:8080""#),
//                Parse { listen: "127.0.0.1:8080".parse().unwrap() });
//     assert_eq!(parse_doc_err::<Parse>(r#"listen "2/3""#),
//         "invalid socket address syntax");

//     assert_eq!(parse::<ParseOpt>(r#"server listen="127.0.0.1:8080""#),
//                ParseOpt { listen: Some("127.0.0.1:8080".parse().unwrap()) });
//     assert_eq!(parse_err::<ParseOpt>(r#"server listen="2/3""#),
//         "invalid socket address syntax");
//     assert_eq!(parse::<ParseOpt>(r#"server listen=null"#),
//                ParseOpt { listen: None });
// }

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
