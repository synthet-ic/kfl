use chumsky::zero_copy::Parser;
use miette::NamedSource;

use crate::{
    ast::Node,
    context::Context,
    errors::Error,
    grammar,
    traits::{Decode, DecodeChildren, Encode},
};

/// Parse KDL text and return AST
pub fn parse(ctx: &mut Context, input: &'static str)
    -> Result<Vec<Node>, Error>
{
    grammar::document()
    .parse_with_state(&input, ctx).into_result();
    .map_err(|errors| {
        Error {
            source_code: NamedSource::new(ctx.get::<&str>().unwrap(), input.to_string()),
            errors: errors.into_iter().map(Into::into).collect(),
        }
    })
}

/// Parse KDL text and decode it into Rust object
pub fn decode<T>(file_name: &'static str, input: &'static str) -> Result<T, Error>
    where T: Decode,
{
    let mut ctx = Context::new();
    let nodes = parse(&mut ctx, &input)?;
    ctx.set::<String>(file_name.to_string());
    Decode::decode(&nodes[0], &mut ctx).map_err(|error| {
        Error {
            source_code: NamedSource::new(file_name, input.to_string()),
            errors: vec![error.into()],
        }
    })
}

// /// Parse single KDL node from AST
// pub fn decode_node<T, S>(ast: &Node) -> Result<T, Vec<DecodeError>>
//     where T: Decode,
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
pub fn decode_children<T>(file_name: &'static str, input: &'static str) -> Result<T, Error>
    where T: DecodeChildren,
{
    decode_with_context(file_name, input, |_| {})
}

/// Parse KDL text and decode Rust object providing extra context for the
/// decoder
pub fn decode_with_context<T, F>(file_name: &'static str, input: &'static str, set_ctx: F)
    -> Result<T, Error>
    where F: FnOnce(&mut Context),
          T: DecodeChildren,
{
    let mut ctx = Context::new();
    let nodes = parse(&mut ctx, &input)?;
    set_ctx(&mut ctx);
    let errors = match <T as DecodeChildren>
        ::decode_children(&nodes, &mut ctx)
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
        source_code: NamedSource::new(file_name, input.to_string()),
        errors: errors.into_iter().map(Into::into).collect(),
    })
}

/// Print ast and return KDL text
#[allow(unused)]
pub fn print(file_name: &str, node: Node) -> Result<String, Error> {
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
    where T: Encode + std::fmt::Debug,
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

// #[test]
// fn normal() {
//     let ctx = Rc::new(RefCell::new(Context::new()));
//     let nodes = parse::<Span>(ctx.clone(), r#"node "hello""#).unwrap();
//     assert_eq!(nodes.len(), 1);
//     assert_eq!(&*nodes[0].node_name, "node");
// }
