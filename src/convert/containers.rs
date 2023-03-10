//! Convert container types.

use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    sync::Arc,
    vec,
    vec::Vec
};
use core::mem;

use crate::{
    ast::{Node, Scalar},
    context::Context,
    errors::{DecodeError, ExpectedType, EncodeError},
    traits::{Decode, DecodePartial, DecodeScalar},
    traits::{Encode, EncodePartial, EncodeScalar},
};

impl<T: Decode> Decode for Box<T> {
    fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as Decode>::decode(node, ctx).map(Box::new)
    }
}

impl<T: DecodePartial> DecodePartial for Box<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context)
        -> Result<bool, DecodeError>
    {
        (**self).decode_partial(node, ctx)
    }
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context)
    //     -> Result<bool, DecodeError>
    // {
    //     (**self).insert_property(name, value, ctx)
    // }
}

impl<T: DecodeScalar> DecodeScalar for Box<T> {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as DecodeScalar>::decode(scalar, ctx).map(Box::new)
    }
}

impl<T: Decode> Decode for Arc<T> {
    fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as Decode>::decode(node, ctx).map(Arc::new)
    }
}

impl<T: DecodePartial> DecodePartial for Arc<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context)
        -> Result<bool, DecodeError>
    {
        Arc::get_mut(self).expect("no Arc clone yet")
            .decode_partial(node, ctx)
    }
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context)
    //     -> Result<bool, DecodeError>
    // {
    //     Arc::get_mut(self).expect("no Arc clone yet")
    //         .insert_property(name, value, ctx)
    // }
}

impl<T: DecodeScalar> DecodeScalar for Arc<T> {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as DecodeScalar>::decode(scalar, ctx).map(Arc::new)
    }
}

impl<T: Decode> Decode for Rc<T> {
    fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as Decode>::decode(node, ctx).map(Rc::new)
    }
}

impl<T: DecodePartial> DecodePartial for Rc<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context)
        -> Result<bool, DecodeError>
    {
        Rc::get_mut(self).expect("no Rc clone yet")
            .decode_partial(node, ctx)
    }
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context)
    //     -> Result<bool, DecodeError>
    // {
    //     Rc::get_mut(self).expect("no Rc clone yet")
    //         .insert_property(name, value, ctx)
    // }
}

impl<T: DecodeScalar> DecodeScalar for Rc<T> {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as DecodeScalar>::decode(scalar, ctx).map(Rc::new)
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as Decode>::decode(node, ctx).map(|node| Some(node))
    }
}

impl<T: Decode> DecodePartial for Option<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context)
        -> Result<bool, DecodeError>
    {
        let slf = mem::take(self);  /* (1) */
        let result = <Self as Decode>::decode(node, ctx);
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

impl<T: DecodeScalar> DecodeScalar for Option<T> {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        match scalar.literal.as_ref() {
            "null" => Ok(None),
            _ => T::decode(scalar, ctx).map(Some),
        }
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, ctx: &mut Context) -> Result<Node, EncodeError> {
        match self {
            None => panic!(),
            Some(t) => <T as Encode>::encode(t, ctx)
        }
    }
}

impl<T: Encode> EncodePartial for Option<T> {
    fn encode_partial(&self, node: &mut Node, ctx: &mut Context)
        -> Result<(), EncodeError>
    {
        let mut children = match mem::take(&mut node.children) {
            None => Vec::new(),
            Some(children) => children
        };
        match self {
            None => panic!(),
            Some(t) => {
                let child = <T as Encode>::encode(t, ctx)?;
                children.push(child);
            }
        };
        let _ = mem::replace(&mut node.children, Some(children));
        Ok(())
    }
}

impl<T: EncodeScalar> EncodeScalar for Option<T> {
    fn encode(&self, ctx: &mut Context) -> Result<Scalar, EncodeError> {
        match &self {
            None => Ok(Scalar { type_name: None, literal: "null".into() }),
            Some(scalar) => <T as EncodeScalar>::encode(&scalar, ctx),
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError> {
        <T as Decode>::decode(node, ctx).map(|node| vec![node])
    }
}

impl<T: Decode> DecodePartial for Vec<T> {
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context)
        -> Result<bool, DecodeError>
    {
        match <T as Decode>::decode(node, ctx) {
            Ok(value) => {
                self.push(value);
                Ok(true)
            }
            Err(e) => Err(e)
        }
    }
}

// impl<T: Decode> DecodeChildren for Vec<T> {
//     fn decode_children(nodes: &[Node], ctx: &mut Context)
//         -> Result<Self, DecodeError>
//     {
//         let mut result = Vec::with_capacity(nodes.len());
//         for node in nodes {
//             match <T as Decode>::decode(node, ctx) {
//                 Ok(node) => result.push(node),
//                 Err(e) => ctx.emit_error(e),
//             }
//         }
//         Ok(result)
//     }
// }

impl<T: Encode> EncodePartial for Vec<T> {
    fn encode_partial(&self, node: &mut Node, ctx: &mut Context)
        -> Result<(), EncodeError>
    {
        let mut children = match mem::take(&mut node.children) {
            None => Vec::new(),
            Some(children) => children
        };
        for item in self.iter() {
            let child = <T as Encode>::encode(item, ctx)?;
            children.push(child);
        }
        let _ = mem::replace(&mut node.children, Some(children));
        Ok(())
    }
}

impl DecodeScalar for Vec<u8> {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        let is_base64 = if let Some(ty) = scalar.type_name.as_ref() {
            match ty.as_ref() {
                "base64" => true,
                _ => {
                    return Err(DecodeError::TypeName {
                        span: ctx.span(&ty),
                        found: Some(ty.clone()),
                        expected: ExpectedType::optional(ty.clone()),
                        rust_type: "bytes",
                    });
                }
            }
        } else { false };
        if is_base64 {
            #[cfg(feature = "base64")] {
                use base64::{Engine as _,
                             engine::general_purpose::STANDARD};
                match STANDARD.decode(scalar.literal.as_bytes()) {
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
            Ok(scalar.literal.as_bytes().to_vec())
        }
    }
}
