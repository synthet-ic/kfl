use kfl::Decode;

#[test]
fn parse_node_span() {
    #[derive(Decode, Debug)]
    struct Node {
        #[kfl(children)]
        children: Vec<kfl::ast::Node>,
    }
    let node = kfl::decode::<Node>("<test>", r#"node { a; b; }"#).unwrap();
    assert_eq!(node.children.len(), 2);
}
