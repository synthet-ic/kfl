use chumsky::Parser;
use miette::NamedSource;

use crate::{
    ast::{Document, SpannedNode},
    decode::Context,
    errors::Error,
    grammar,
    span::Span,
    traits::{self, Decode, DecodeChildren, Encode},
};

/// Parse KDL text and return AST
pub fn parse<S: traits::Span>(file_name: &str, text: &str)
    -> Result<Document<S>, Error>
{
    grammar::document()
    .parse(S::stream(text))
    .map_err(|errors| {
        Error {
            source_code: NamedSource::new(file_name, text.to_string()),
            errors: errors.into_iter().map(Into::into).collect(),
        }
    })
}

/// Parse KDL text and decode it into Rust object
pub fn decode<T>(file_name: &str, text: &str) -> Result<T, Error>
    where T: Decode<Span>,
{
    let ast = parse(file_name, text)?;
    let mut ctx = Context::new();
    Decode::decode(&ast.nodes[0], &mut ctx).map_err(|error| {
        Error {
            source_code: NamedSource::new(file_name, text.to_string()),
            errors: vec![error.into()],
        }
    })
}

// /// Parse single KDL node from AST
// pub fn decode_node<T, S>(ast: &SpannedNode<S>) -> Result<T, Vec<DecodeError<S>>>
//     where T: Decode<S>,
//           S: ErrorSpan,
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
pub fn decode_children<T>(file_name: &str, text: &str) -> Result<T, Error>
    where T: DecodeChildren<Span>,
{
    decode_with_context(file_name, text, |_| {})
}

/// Parse KDL text and decode Rust object providing extra context for the
/// decoder
pub fn decode_with_context<T, S, F>(file_name: &str, text: &str, set_ctx: F)
    -> Result<T, Error>
    where F: FnOnce(&mut Context<S>),
          T: DecodeChildren<S>,
          S: traits::Span,
{
    let ast = parse(file_name, text)?;

    let mut ctx = Context::new();
    set_ctx(&mut ctx);
    let errors = match <T as DecodeChildren<S>>
        ::decode_children(&ast.nodes, &mut ctx)
    {
        Ok(_) if ctx.has_errors() => {
            ctx.into_errors()
        }
        Err(e) => {
            ctx.emit_error(e);
            ctx.into_errors()
        }
        Ok(v) => return Ok(v)
    };
    Err(Error {
        source_code: NamedSource::new(file_name, text.to_string()),
        errors: errors.into_iter().map(Into::into).collect(),
    })
}

/// Print ast and return KDL text
pub fn print<S: traits::Span>(file_name: &str, node: SpannedNode<S>)
    -> Result<String, Error>
{
    Ok("".into())
    // grammar::document()
    // .parse(S::stream(text))
    // .map_err(|errors| {
    //     Error {
    //         source_code: NamedSource::new(file_name, text.to_string()),
    //         errors: errors.into_iter().map(Into::into).collect(),
    //     }
    // })
}

/// Encode Rust object and print it into KDL text
pub fn encode<T>(file_name: &str, t: &T) -> Result<String, Error>
    where T: Encode<Span> + std::fmt::Debug,
{
    let mut ctx = Context::new();
    let node = t.encode(&mut ctx).map_err(|error| {
        Error {
            source_code: NamedSource::new(file_name, format!("{:?}", &t)),
            errors: vec![error.into()],
        }
    })?;
    print(file_name, node)
}

/*
/// Decode Rust object providing extra context for the
/// decoder and print KDL text
pub fn print_with_context<T, S, F>(file_name: &str, document: T, set_ctx: F)
    -> Result<String, Error>
    where F: FnOnce(&mut Context<S>),
          T: EncodeChildren<S, _>,
          S: traits::Span,
{
    Err(Error::)
}
*/

#[test]
fn normal() {
    let doc = parse::<Span>("embedded.kdl", r#"node "hello""#).unwrap();
    assert_eq!(doc.nodes.len(), 1);
    assert_eq!(&**doc.nodes[0].node_name, "node");
}
