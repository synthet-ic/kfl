use chumsky::Parser;
use miette::NamedSource;

use crate::{
    ast::Document,
    decode::Context,
    errors::Error,
    grammar,
    span::Span,
    traits::{self, DecodeChildren, EncodeChildren}
};

/// Parse KDL text and return AST
pub fn parse_ast<S: traits::Span>(file_name: &str, text: &str)
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

/// Parse KDL text and decode Rust object
pub fn parse<T>(file_name: &str, text: &str) -> Result<T, Error>
    where T: DecodeChildren<Span>,
{
    parse_with_context(file_name, text, |_| {})
}

/// Parse KDL text and decode Rust object providing extra context for the
/// decoder
pub fn parse_with_context<T, S, F>(file_name: &str, text: &str, set_ctx: F)
    -> Result<T, Error>
    where F: FnOnce(&mut Context<S>),
          T: DecodeChildren<S>,
          S: traits::Span,
{
    let ast = parse_ast(file_name, text)?;

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

/*
/// Encode Rust object and print KDL text
pub fn print<T>(file_name: &str, document: T) -> Result<String, Error>
    where T: EncodeChildren<Span, _>,
{
    print_with_context(file_name, document, |_| {})
}

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
    let doc = parse_ast::<Span>("embedded.kdl", r#"node "hello""#).unwrap();
    assert_eq!(doc.nodes.len(), 1);
    assert_eq!(&**doc.nodes[0].node_name, "node");
}
