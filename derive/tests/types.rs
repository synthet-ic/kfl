mod common;

use std::path::PathBuf;
use kfl::Decode;

#[derive(Decode, Debug, PartialEq)]
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

#[test]
fn decode_types() {
    assert_decode!(r#"
        scalars \
            "hello" \
            1234 \
            1.234 \
            /hello/world \
            true \
    "#,
        Scalars {
            str: "hello".into(),
            u64: 1234,
            f64: 1.234,
            path: PathBuf::from("/hello/world"),
            boolean: true,
        }
    );
}
