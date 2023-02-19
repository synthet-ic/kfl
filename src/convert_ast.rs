use crate::{
    ast::{Node, SpannedNode, Literal, Scalar},
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

impl<S: Span, T: DecodeSpan<S>> DecodeScalar<S> for Scalar<T> {
    fn decode(scalar: &Scalar<S>, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        Ok(Scalar {
            type_name: scalar.type_name.as_ref().map(|n| n.clone_as(ctx)),
            literal: scalar.literal.clone_as(ctx),
        })
    }
}

impl<S: Span> DecodeScalar<S> for Literal {
    fn decode(scalar: &Scalar<S>, _: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        Ok((*scalar.literal).clone())
    }
}
