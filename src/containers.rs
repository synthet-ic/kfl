use std::{
    sync::Arc,
    rc::Rc,
};

use crate::{
    ast::{Node, Literal, BuiltinType},
    decode::Context,
    errors::{DecodeError, ExpectedType},
    traits::{Decode, DecodePartial, DecodeChildren, DecodeScalar},
    traits::ErrorSpan
};

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Box<T> {
    fn decode(node: &Node, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode(node, ctx).map(Box::new)
    }
}

impl<S: ErrorSpan, T: DecodeChildren<S>> DecodeChildren<S> for Box<T> {
    fn decode_children(nodes: &[Node], ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeChildren<S>>::decode_children(nodes, ctx).map(Box::new)
    }
}

impl<S: ErrorSpan, T: DecodePartial<S>> DecodePartial<S> for Box<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        (**self).decode_partial(node, ctx)
    }
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context<S>)
    //     -> Result<bool, DecodeError<S>>
    // {
    //     (**self).insert_property(name, value, ctx)
    // }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Box<T> {
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeScalar<S>>::decode(scalar, ctx).map(Box::new)
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Arc<T> {
    fn decode(node: &Node, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode(node, ctx).map(Arc::new)
    }
}

impl<S: ErrorSpan, T: DecodeChildren<S>> DecodeChildren<S> for Arc<T> {
    fn decode_children(nodes: &[Node], ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeChildren<S>>::decode_children(nodes, ctx).map(Arc::new)
    }
}

impl<S: ErrorSpan, T: DecodePartial<S>> DecodePartial<S> for Arc<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        Arc::get_mut(self).expect("no Arc clone yet")
            .decode_partial(node, ctx)
    }
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context<S>)
    //     -> Result<bool, DecodeError<S>>
    // {
    //     Arc::get_mut(self).expect("no Arc clone yet")
    //         .insert_property(name, value, ctx)
    // }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Arc<T> {
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeScalar<S>>::decode(scalar, ctx).map(Arc::new)
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Rc<T> {
    fn decode(node: &Node, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode(node, ctx).map(Rc::new)
    }
}

impl<S: ErrorSpan, T: DecodeChildren<S>> DecodeChildren<S> for Rc<T> {
    fn decode_children(nodes: &[Node], ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeChildren<S>>::decode_children(nodes, ctx).map(Rc::new)
    }
}

impl<S: ErrorSpan, T: DecodePartial<S>> DecodePartial<S> for Rc<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        Rc::get_mut(self).expect("no Rc clone yet")
            .decode_partial(node, ctx)
    }
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context<S>)
    //     -> Result<bool, DecodeError<S>>
    // {
    //     Rc::get_mut(self).expect("no Rc clone yet")
    //         .insert_property(name, value, ctx)
    // }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Rc<T> {
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeScalar<S>>::decode(scalar, ctx).map(Rc::new)
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Vec<T> {
    fn decode(node: &Node, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode(node, ctx).map(|node| vec![node])
    }
}

impl<S: ErrorSpan, T: Decode<S>> DecodePartial<S> for Vec<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        match <T as Decode<S>>::decode(node, ctx) {
            Ok(value) => {
                self.push(value);
                Ok(true)
            }
            Err(e) => Err(e)
        }
    }
}

impl<S: ErrorSpan, T: Decode<S>> DecodeChildren<S> for Vec<T> {
    fn decode_children(nodes: &[Node], ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        let mut result = Vec::with_capacity(nodes.len());
        for node in nodes {
            match <T as Decode<S>>::decode(node, ctx) {
                Ok(node) => result.push(node),
                Err(e) => ctx.emit_error(e),
            }
        }
        Ok(result)
    }
}

impl<S: ErrorSpan> DecodeScalar<S> for Vec<u8> {
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        let is_base64 = if let Some(ty) = scalar.type_name.as_ref() {
            match ty.as_builtin() {
                Some(&BuiltinType::Base64) => true,
                _ => {
                    return Err(DecodeError::TypeName {
                        span: ctx.span(&ty),
                        found: Some(ty.clone()),
                        expected: ExpectedType::optional(BuiltinType::Base64),
                        rust_type: "bytes",
                    });
                }
            }
        } else { false };
        match &scalar.literal {
            Literal::String(ref s) => {
                if is_base64 {
                    #[cfg(feature = "base64")] {
                        use base64::{Engine as _,
                                     engine::general_purpose::STANDARD};
                        match STANDARD.decode(s.as_bytes()) {
                            Ok(vec) => Ok(vec),
                            Err(e) => {
                                Err(DecodeError::conversion(ctx.span(&scalar), e))
                            }
                        }
                    }
                    #[cfg(not(feature = "base64"))] {
                        Err(DecodeError::unsupported(ctx.span(&value),
                            "base64 support is not compiled in"))
                    }
                } else {
                    Ok(s.as_bytes().to_vec())
                }
            }
            _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "string",
                                              &scalar.literal))
        }
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Option<T> {
    fn decode(node: &Node, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode(node, ctx).map(|node| Some(node))
    }
}

impl<S: ErrorSpan, T: Decode<S>> DecodePartial<S> for Option<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        let slf = std::mem::take(self);  /* (1) */
        let result = <Self as Decode<S>>::decode(node, ctx);
        match (slf, result) {
            (None, Ok(None)) => Ok(true),  /* no-op */
            (None, Ok(value)) => {
                *self = value;
                Ok(true)
            }
            (slf, Err(_)) => {
                *self = slf;  /* TODO improve this with line (1) */
                Ok(false)
            },
            (_, _) => {
                let dup_err = format!("duplicate node `{}`, single node expected", node.node_name.as_ref());
                Err(DecodeError::unexpected(ctx.span(&node.node_name), "node",
                    dup_err))
            }
        }
    }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Option<T> {
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        match &scalar.literal {
            Literal::Null => Ok(None),
            _ => <T as DecodeScalar<S>>::decode(scalar, ctx).map(Some),
        }
    }
}
