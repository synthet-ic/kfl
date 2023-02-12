// use std::fmt;
// use std::collections::BTreeMap;
// use std::default::Default;

// use miette::Diagnostic;

// use kfl::{Decode, span::Span};
// use kfl::traits::DecodeChildren;

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Unwrap {
//     #[kfl(child, unwrap(argument))]
//     label: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct UnwrapRawIdent {
//     #[kfl(child, unwrap(argument))]
//     r#type: String,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct UnwrapFiltChildren {
//     #[kfl(children, unwrap(arguments))]
//     labels: Vec<Vec<String>>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct UnwrapChildren {
//     #[kfl(children, unwrap(arguments))]
//     labels: Vec<Vec<String>>,
// }

// #[derive(kfl_derive::Decode, Debug, PartialEq)]
// struct Parse {
//     #[kfl(child, unwrap(argument, str))]
//     listen: std::net::SocketAddr,
// }

// #[test]
// fn parse_unwrap() {
//     assert_eq!(parse::<Unwrap>(r#"node { label "hello"; }"#),
//                Unwrap { label: "hello".into() } );
//     assert_eq!(parse_err::<Unwrap>(r#"node label="hello""#),
//         "unexpected property `label`");
//     assert_eq!(parse_err::<Unwrap>(r#"node"#),
//         "child node `label` is required");
//     assert_eq!(parse_doc::<Unwrap>(r#"label "hello""#),
//                Unwrap { label: "hello".into() } );
// }

// #[test]
// fn parse_unwrap_raw_ident() {
//     assert_eq!(parse::<UnwrapRawIdent>(r#"node { type "hello"; }"#),
//                UnwrapRawIdent { r#type: "hello".into() } );
//     assert_eq!(parse_err::<UnwrapRawIdent>(r#"node type="hello""#),
//                "unexpected property `type`");
//     assert_eq!(parse_err::<UnwrapRawIdent>(r#"node"#),
//                "child node `type` is required");
//     assert_eq!(parse_doc::<UnwrapRawIdent>(r#"type "hello""#),
//                UnwrapRawIdent { r#type: "hello".into() } );
// }

// #[test]
// fn parse_unwrap_filtered_children() {
//     assert_eq!(parse::<UnwrapFiltChildren>(
//        r#"node { labels "hello" "world"; labels "oh" "my"; }"#),
//        UnwrapFiltChildren { labels: vec![
//            vec!["hello".into(), "world".into()],
//            vec!["oh".into(), "my".into()],
//        ]},
//     );
// }

// #[test]
// fn parse_unwrap_children() {
//     assert_eq!(parse::<UnwrapChildren>(
//        r#"node { some "hello" "world"; other "oh" "my"; }"#),
//        UnwrapChildren { labels: vec![
//            vec!["hello".into(), "world".into()],
//            vec!["oh".into(), "my".into()],
//        ]},
//     );
// }
