//! Used by derive macro.

use alloc::format;
use crate::{
    ast::Node,
    context::Context,
    errors::DecodeError
};

///
pub fn check_type(ident: &str, node: &Node, ctx: &Context)
    -> Result<(), DecodeError>
{
    if node.type_name.is_some() {
        return Err(DecodeError::unexpected(
                   ctx.span(&node), "type name",
                   "no type name expected for this node"));
    }
    if node.node_name.as_ref() != ident {
        return Err(DecodeError::unexpected(ctx.span(&node),
                   "node", format!("unexpected node `{}`",
                   node.node_name.as_ref())));
    }
    Ok(())
}
