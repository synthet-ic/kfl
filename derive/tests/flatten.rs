mod common;

use kfl::{Decode, DecodePartial};

#[derive(Decode, Debug, PartialEq)]
struct Child1(#[kfl(argument)] String);

#[derive(Decode, Debug, PartialEq)]
struct Child2(#[kfl(argument)] String);

#[derive(DecodePartial, Debug, Default, PartialEq)]
struct Intermediate {
    #[kfl(child, default)]
    child1: Option<Child1>,
    #[kfl(children, default)]
    children2: Vec<Child2>
}

#[derive(Decode, Debug, PartialEq)]
struct Parent {
    #[kfl(flatten)]
    intermediate: Intermediate,
}

#[test]
fn parse_flatten() {
    #[derive(Decode, Debug, PartialEq)]
    struct Child1(#[kfl(argument)] String);
    #[derive(Decode, Debug, PartialEq)]
    struct Child2(#[kfl(argument)] String);
    #[derive(DecodePartial, Debug, Default, PartialEq)]
    struct Intermediate {
        #[kfl(child, default)]
        child1: Option<Child1>,
        #[kfl(children, default)]
        children2: Vec<Child2>
    }
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(flatten)]
        intermediate: Intermediate,
    }
    assert_decode!(
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
    assert_decode_error!(Parent,
        r#"something "world""#,
        "unexpected node `something`");
}

#[test]
fn parse_flatten_flatten() {
    #[derive(Decode, Debug, PartialEq)]
    struct Child3(#[kfl(argument)] String);
    #[derive(Decode, Debug, PartialEq)]
    struct Child4(#[kfl(argument)] String);
    #[derive(DecodePartial, Debug, Default, PartialEq)]
    struct Intermediate2 {
        #[kfl(child, default)]
        child3: Option<Child3>,
        #[kfl(children, default)]
        children4: Vec<Child4>,
    }
    #[derive(Decode, Debug, PartialEq)]
    struct Child1(#[kfl(argument)] String);
    #[derive(Decode, Debug, PartialEq)]
    struct Child2(#[kfl(argument)] String);
    #[derive(DecodePartial, Debug, Default, PartialEq)]
    struct Intermediate1 {
        #[kfl(child, default)]
        child1: Option<Child1>,
        #[kfl(children, default)]
        children2: Vec<Child2>,
        #[kfl(flatten)]
        intermediate: Intermediate2
    }
    #[derive(Decode, Debug, PartialEq)]
    struct Parent {
        #[kfl(flatten)]
        intermediate: Intermediate1,
    }
    assert_decode!(
        r#"parent {
            child2 "v2"
            child1 "v1"
            child2 "v3"
            child3 "v4"
            child4 "v5"
            child2 "v6"
        }"#,
        Parent {
            intermediate: Intermediate1 {
                child1: Some(Child1("v1".into())),
                children2: vec![
                    Child2("v2".into()),
                    Child2("v3".into()),
                    Child2("v6".into())
                ],
                intermediate: Intermediate2 {
                    child3: Some(Child3("v4".into())),
                    children4: vec![Child4("v5".into())]
                }
            }
        });
    assert_decode_error!(Parent,
        r#"something "world""#,
        "unexpected node `something`");
}
