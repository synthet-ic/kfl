//! Literal conversion. Makes Node and Scalar transparent objects.

mod containers;
mod scalars;

use crate::{
    ast::{Node, Scalar},
    context::Context,
    errors::DecodeError,
    traits::{Decode, DecodeScalar}
};

impl Decode for Node {
    fn decode(node: &Node, ctx: &mut Context)
        -> Result<Self, DecodeError>
    {
        Ok(Node {
            type_name: node.type_name.as_ref().map(|n| n.clone()),
            node_name: node.node_name.clone(),
            arguments: node.arguments.iter()
                .map(|v| DecodeScalar::decode(v, ctx))
                .collect::<Result<_, _>>()?,
            properties: node.properties.iter()
                .map(|(k, v)| {
                    Ok((k.clone(), DecodeScalar::decode(v, ctx)?))
                })
                .collect::<Result<_, _>>()?,
            children: node.children.as_ref().map(|sc| {
                Ok(sc.iter()
                    .map(|node| Ok(Decode::decode(node, ctx)?))
                    .collect::<Result<_, _>>()?)
            }).transpose()?,
        })
    }
}

impl DecodeScalar for Scalar {
    fn decode(scalar: &Scalar, _: &mut Context)
        -> Result<Self, DecodeError>
    {
        Ok(Scalar {
            type_name: scalar.type_name.as_ref().map(|n| n.clone()),
            literal: scalar.literal.clone(),
        })
    }
}

// TODO(rnarkk) 
// impl DecodeScalar for Literal {
//     fn decode(scalar: &Scalar, _: &mut Context)
//         -> Result<Self, DecodeError>
//     {
//         Ok(scalar.literal.clone())
//     }
// }
