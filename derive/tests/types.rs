use std::path::PathBuf;

use kfl::{
    Decode,
    span::Span,
    traits::DecodeChildren
};

#[derive(knuffel_derive::Decode, Debug, PartialEq)]
struct Scalars {
    #[kfl(argument)]
    str: String,
    #[kfl(argument)]
    u64: u64,
    #[kfl(argument)]
    f64: f64,
    #[kfl(argument)]
    path: PathBuf,
    #[kfl(argument)]
    boolean: bool,
}

fn parse<T: DecodeChildren<Span>>(text: &str) -> T {
    kfl::parse("<test>", text).unwrap()
}

#[test]
fn parse_enum() {
    assert_eq!(
        parse::<Scalars>(r#"
            scalars \
                "hello" \
                1234 \
                1.234 \
                "/hello/world" \
                true \
        "#),
        Scalars {
            str: "hello".into(),
            u64: 1234,
            f64: 1.234,
            path: PathBuf::from("/hello/world"),
            boolean: true,
        });
}
