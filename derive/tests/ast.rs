// use kfl::{
//     Decode,
//     ast::SpannedNode,
//     span::Span,
// };

// #[derive(Decode, Debug)]
// #[kfl(span_type = Span)]
// struct AstChildren {
//     #[kfl(children)]
//     children: Vec<SpannedNode<Span>>,
// }

// fn parse<T: Decode<Span>>(text: &str) -> T {
//     let mut nodes: Vec<T> = kfl::parse("<test>", text).unwrap();
//     assert_eq!(nodes.len(), 1);
//     nodes.remove(0)
// }

// #[test]
// fn parse_node_span() {
//     let item = parse::<AstChildren>(r#"node {a; b;}"#);
//     assert_eq!(item.children.len(), 2);
// }
