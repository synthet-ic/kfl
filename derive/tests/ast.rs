use kfl::{
    Decode,
    ast::SpannedNode,
    span::Span,
};

#[test]
fn parse_node_span() {
    #[derive(Decode, Debug)]
    #[kfl(span_type = Span)]
    struct Node {
        #[kfl(children)]
        children: Vec<SpannedNode<Span>>,
    }
    let node = kfl::decode::<Node>("<test>", r#"node { a; b; }"#).unwrap();
    assert_eq!(node.children.len(), 2);
}
