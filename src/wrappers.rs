use alloc::{
    borrow::ToOwned,
    format,
    string::String,
    vec,
    vec::Vec
};
use core::fmt::{Debug, Write};
use miette::NamedSource;

use crate::{
    ast::Node,
    context::Context,
    errors::Error,
    grammar,
    traits::{Decode, DecodePartial, Encode, EncodePartial},
};

/// Parse KDL text and return AST
pub fn parse(ctx: &mut Context, input: &str) -> Result<Vec<Node>, Error> {
    grammar::document()
    .parse_with_state(&input, ctx).into_result()
    .map_err(|errors| {
        Error {
            source_code: NamedSource::new(ctx.get::<&str>().unwrap(), input.to_owned()),
            errors: errors.into_iter().map(Into::into).collect(),
        }
    })
}

/// Parse KDL text and decode it into Rust object
pub fn decode<T>(file_name: &'static str, input: &str) -> Result<T, Error>
    where T: Decode,
{
    let mut ctx = Context::new();
    let nodes = parse(&mut ctx, &input)?;
    ctx.set::<String>(file_name.to_owned());
    Decode::decode(&nodes[0], &mut ctx).map_err(|error| {
        Error {
            source_code: NamedSource::new(file_name, input.to_owned()),
            errors: vec![error.into()],
        }
    })
}

// /// Parse single KDL node from AST
// pub fn decode_node<T>(ast: &Node) -> Result<T, Vec<DecodeError>>
//     where T: Decode,
// {
//     let mut ctx = Context::new();
//     match Decode::decode(ast, &mut ctx) {
//         Ok(_) if ctx.has_errors() => {
//             Err(ctx.into_errors())
//         }
//         Err(e) => {
//             ctx.emit_error(e);
//             Err(ctx.into_errors())
//         }
//         Ok(v) => Ok(v)
//     }
// }

/// Parse KDL text and decode Rust object
pub fn decode_children<T>(file_name: &str, input: &str) -> Result<T, Error>
    where T: DecodePartial,
{
    decode_with_context(file_name, input, |_| {})
}

/// Parse KDL text and decode Rust object providing extra context for the
/// decoder
pub fn decode_with_context<T, F>(file_name: &str, input: &str, set_ctx: F)
    -> Result<T, Error>
    where F: FnOnce(&mut Context),
          T: DecodePartial,
{
    let mut ctx = Context::new();
    let nodes = parse(&mut ctx, &input)?;
    set_ctx(&mut ctx);
    let mut output = <T as Default>::default();
    for node in nodes {
        output.decode_partial(&node, &mut ctx).map_err(|error| {
            Error {
                source_code: NamedSource::new(file_name, input.to_owned()),
                errors: vec![error.into()],
            }
        })?;
    }
    Ok(output)
}

/// Print ast and return KDL text
pub fn print(_ctx: &mut Context, node: Node) -> Result<String, Error> {
    let mut output = String::new();
    write!(output, "{}", node).unwrap();
    Ok(output)
    // .map_err(|errors| {
    //     Error {
    //         source_code: NamedSource::new(file_name, text.to_string()),
    //         errors: errors.into_iter().map(Into::into).collect(),
    //     }
    // })
}

/// Encode Rust object and print it into KDL text
pub fn encode<T>(file_name: &str, t: &T) -> Result<String, Error>
    where T: Encode + Debug,
{
    let mut ctx = Context::new();
    ctx.set::<String>(file_name.to_owned());
    let node = t.encode(&mut ctx).map_err(|error| {
        Error {
            source_code: NamedSource::new(file_name, format!("{:?}", &t)),
            errors: vec![error.into()],
        }
    })?;
    print(&mut ctx, node)
}

/// Parse KDL text and decode Rust object
pub fn encode_children<T>(file_name: &str, t: &T) -> Result<String, Error>
    where T: EncodePartial + Debug,
{
    encode_with_context(file_name, t, |_| {})
}

/// Parse KDL text and decode Rust object providing extra context for the
/// decoder
pub fn encode_with_context<T, F>(file_name: &str, t: &T, set_ctx: F)
    -> Result<String, Error>
    where F: FnOnce(&mut Context),
          T: EncodePartial + Debug,
{
    let mut ctx = Context::new();
    // let nodes = print(&mut ctx, &t)?;
    set_ctx(&mut ctx);
    let mut node = Node::new("-");
    t.encode_partial(&mut node, &mut ctx).map_err(|error| {
        Error {
            source_code: NamedSource::new(file_name, format!("{:?}", &t)),
            errors: vec![error.into()],
        }
    })?;
    Ok(print(&mut ctx, node)?)
}

#[test]
fn normal() {
    let mut ctx = Context::new();
    let nodes = parse(&mut ctx, r#"node "hello""#).unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(&*nodes[0].node_name, "node");
}
