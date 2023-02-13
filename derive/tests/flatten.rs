mod common;

use kfl::Decode;

use common::{assert_parse, assert_parse_err};

// #[derive(Decode, Default, Debug, PartialEq)]
// struct Prop1 {
//     #[kfl(property)]
//     label: Option<String>,
// }

// #[derive(Decode, Debug, PartialEq)]
// struct FlatProp {
//     #[kfl(flatten(property))]
//     props: Prop1,
// }

#[derive(Decode, Debug, PartialEq)]
struct Child1(#[kfl(argument)] String);

#[derive(Decode, Debug, PartialEq)]
struct Child2(#[kfl(argument)] String);

#[derive(Decode, Debug, PartialEq)]
struct Intermediate {
    #[kfl(child)]
    child1: Option<Child1>,
    #[kfl(children)]
    children2: Vec<Child2>
}

#[derive(Decode, Debug, PartialEq)]
struct Parent {
    #[kfl(flatten)]
    intermediate: Intermediate,
}

// #[test]
// fn parse_flat_prop() {
//     assert_eq!(parse::<FlatProp>(r#"node label="hello""#),
//         FlatProp { props: Prop1 { label: Some("hello".into()) } } );
//     assert_eq!(parse_err::<FlatProp>(r#"node something="world""#),
//         "unexpected property `something`");
// }

#[test]
fn parse_flat_child() {
    assert_parse::<Parent>(
        r#"parent {
            child2 "v2"
            child1 "v1"
            child2 "v3"
        }"#,
        Parent {
            intermediate: Intermediate {
                child1: Some(Child1("v1".into())),
                children2: vec![Child2("v2".into()), Child2("v3".into())]
            }
        });
    assert_parse_err::<Parent>(
        r#"something "world""#,
        "unexpected node `something`");
}
