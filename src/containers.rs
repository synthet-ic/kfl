use std::{
    sync::Arc,
    rc::Rc
};

use crate::{
    ast::{SpannedNode, Literal, Value, TypeName},
    decode::Context,
    errors::DecodeError,
    span::Spanned,
    traits::{Decode, DecodeChildren, DecodeScalar, DecodePartial},
    traits::{ErrorSpan, DecodeSpan, Span}
};

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Box<T> {
    fn decode_node(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode_node(node, ctx).map(Box::new)
    }
}

// impl<S: ErrorSpan, T: DecodeChildren<S>> DecodeChildren<S> for Box<T> {
//     fn decode_children(nodes: &[SpannedNode<S>], ctx: &mut Context<S>)
//         -> Result<Self, DecodeError<S>>
//     {
//         DecodeChildren::decode_children(nodes, ctx).map(Box::new)
//     }
// }

impl<S: ErrorSpan, T: DecodePartial<S>> DecodePartial<S> for Box<T> {
    fn insert_child(&mut self, node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        (**self).insert_child(node, ctx)
    }
    fn insert_property(&mut self,
                       name: &Spanned<Box<str>, S>, value: &Value<S>,
                       ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        (**self).insert_property(name, value, ctx)
    }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Box<T> {
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>) {
        T::type_check(type_name, ctx)
    }
    fn raw_decode(value: &Spanned<Literal, S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeScalar<S>>::raw_decode(value, ctx).map(Box::new)
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Arc<T> {
    fn decode_node(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode_node(node, ctx).map(Arc::new)
    }
}

// impl<S: ErrorSpan, T: DecodeChildren<S>> DecodeChildren<S> for Arc<T> {
//     fn decode_children(nodes: &[SpannedNode<S>], ctx: &mut Context<S>)
//         -> Result<Self, DecodeError<S>>
//     {
//         DecodeChildren::decode_children(nodes, ctx).map(Arc::new)
//     }
// }

impl<S: ErrorSpan, T: DecodePartial<S>> DecodePartial<S> for Arc<T> {
    fn insert_child(&mut self, node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        Arc::get_mut(self).expect("no Arc clone yet")
            .insert_child(node, ctx)
    }
    fn insert_property(&mut self,
                       name: &Spanned<Box<str>, S>, value: &Value<S>,
                       ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        Arc::get_mut(self).expect("no Arc clone yet")
            .insert_property(name, value, ctx)
    }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Arc<T> {
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>)
    {
        T::type_check(type_name, ctx)
    }
    fn raw_decode(value: &Spanned<Literal, S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeScalar<S>>::raw_decode(value, ctx).map(Arc::new)
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Rc<T> {
    fn decode_node(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode_node(node, ctx).map(Rc::new)
    }
}

// impl<S: ErrorSpan, T: DecodeChildren<S>> DecodeChildren<S> for Rc<T> {
//     fn decode_children(nodes: &[SpannedNode<S>], ctx: &mut Context<S>)
//         -> Result<Self, DecodeError<S>>
//     {
//         DecodeChildren::decode_children(nodes, ctx).map(Rc::new)
//     }
// }

impl<S: ErrorSpan, T: DecodePartial<S>> DecodePartial<S> for Rc<T> {
    fn insert_child(&mut self, node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        Rc::get_mut(self).expect("no Rc clone yet")
            .insert_child(node, ctx)
    }
    fn insert_property(&mut self,
                       name: &Spanned<Box<str>, S>, value: &Value<S>,
                       ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>
    {
        Rc::get_mut(self).expect("no Rc clone yet")
            .insert_property(name, value, ctx)
    }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Rc<T> {
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>)
    {
        T::type_check(type_name, ctx)
    }
    fn raw_decode(value: &Spanned<Literal, S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as DecodeScalar<S>>::raw_decode(value, ctx).map(Rc::new)
    }
}

// impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Vec<T> {
//     fn decode_node(node: &SpannedNode<S>, ctx: &mut Context<S>)
//         -> Result<Self, DecodeError<S>>
//     {
//         <T as Decode>::decode_node(node, ctx)
//     }
// }

impl<S: ErrorSpan, T: Decode<S>> DecodeChildren<S> for Vec<T> {
    type Item = T;

    fn decode_children(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self::Item, DecodeError<S>>
    {
        <Self::Item as Decode<S>>::decode_node(node, ctx)
    }
}

impl<S: ErrorSpan, T: Decode<S>> Decode<S> for Option<T> {
    fn decode_node(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        <T as Decode<S>>::decode_node(node, ctx).map(|node| Some(node))
    }
}

impl<S: ErrorSpan, T: DecodeScalar<S>> DecodeScalar<S> for Option<T> {
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>) {
        T::type_check(type_name, ctx)
    }
    fn raw_decode(value: &Spanned<Literal, S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        match &**value {
            Literal::Null => Ok(None),
            _ => DecodeScalar::raw_decode(value, ctx).map(Some),
        }
    }
}

// impl<S: ErrorSpan, T: Decode<S>> DecodeChildren<S> for Option<T> {
//     fn decode_children(nodes: &[SpannedNode<S>], ctx: &mut Context<S>)
//         -> Result<Self, DecodeError<S>>
//     {
//         match nodes.len() {
//             0 => Ok(None),
//             1 => Decode::decode_node(&nodes[0], ctx),
//             _ => {
//                 Err(DecodeError::unexpected(
//                     &nodes[1],
//                     "kind",
//                     "Option"
//                 ))
//             }
//         }
//     }
// }

impl<T: DecodeScalar<S>, S, Q> DecodeScalar<S> for Spanned<T, Q>
    where S: Span,
          Q: DecodeSpan<S>
{
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>)
    {
        T::type_check(type_name, ctx)
    }
    fn raw_decode(value: &Spanned<Literal, S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        let decoded = T::raw_decode(value, ctx)?;
        Ok(Spanned {
            span: DecodeSpan::decode_span(&value.span, ctx),
            value: decoded,
        })
    }
}
