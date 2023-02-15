use crate::{
    ast::{Node, SpannedNode, TypeName, Literal, Value},
    decode::Context,
    errors::DecodeError,
    span::Spanned,
    traits::{Decode, DecodeScalar, DecodeSpan, Span}
};

impl<S, T> Decode<S> for Node<T>
    where S: Span,
          T: DecodeSpan<S>
{
    fn decode(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        Ok(Node {
            type_name: node.type_name.as_ref().map(|n| n.clone_as(ctx)),
            node_name: node.node_name.clone_as(ctx),
            arguments: node.arguments.iter()
                .map(|v| DecodeScalar::decode(v, ctx))
                .collect::<Result<_, _>>()?,
            properties: node.properties.iter()
                .map(|(k, v)| {
                    Ok((k.clone_as(ctx), DecodeScalar::decode(v, ctx)?))
                })
                .collect::<Result<_, _>>()?,
            children: node.children.as_ref().map(|sc| {
                Ok(Spanned {
                    span: DecodeSpan::decode_span(&sc.span, ctx),
                    value: sc.iter()
                        .map(|node| Ok(Spanned {
                            span: DecodeSpan::decode_span(&node.span, ctx),
                            value: Decode::decode(node, ctx)?,
                        }))
                        .collect::<Result<_, _>>()?,
                })
            }).transpose()?,
        })
    }
}

impl<S, T> Decode<S> for SpannedNode<T>
    where S: Span,
          T: DecodeSpan<S>
{
    fn decode(node: &SpannedNode<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        Ok(Spanned {
            span: DecodeSpan::decode_span(&node.span, ctx),
            value: Decode::decode(node, ctx)?,
        })
    }
}

impl<S, T> DecodeScalar<S> for Value<T>
    where S: Span,
          T: DecodeSpan<S>
{
    fn type_check(_type_name: &Option<Spanned<TypeName, S>>,
                  _ctx: &mut Context<S>)
    {
    }
    fn raw_decode(_value: &Spanned<Literal, S>, _ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        panic!("called `raw_decode` directly on the `Value`");
    }
    fn decode(value: &Value<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        Ok(Value {
            type_name: value.type_name.as_ref().map(|n| n.clone_as(ctx)),
            literal: value.literal.clone_as(ctx),
        })
    }
}

impl<S> DecodeScalar<S> for Literal
    where S: Span,
{
    fn type_check(_type_name: &Option<Spanned<TypeName, S>>,
                  _ctx: &mut Context<S>)
    {
    }
    fn raw_decode(value: &Spanned<Literal, S>, _ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        Ok((**value).clone())
    }
}
