//! Traits used for the library
//!
//! Most users will never implement these manually. See
//! [`Decode`](derive@crate::Decode)` and
//! [`DecodeScalar`](derive@crate::DecodeScalar) for a
//! documentation of the derives to implement these traits.

use crate::{
    ast::{Node, Scalar},
    errors::{DecodeError, EncodeError},
    context::Context
};

/// Trait to decode KDL node from the AST
pub trait Decode: Sized {
    /// Decodes the node from the ast
    fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError>;
}

/// The trait is implemented for structures that can be used as part of other
/// structs
///
/// The type of field that `#[kfl(flatten)]` is used for should implement
/// this trait. It is automatically implemented by `#[derive(Decode)]`
/// by structures that have only optional properties and children (no
/// arguments).
pub trait DecodePartial: Sized + Default {
    /// The method is called when unknown child is encountered by parent
    /// structure
    ///
    /// Returns `Ok(true)` if the child is "consumed" (i.e. stored in this
    /// structure).
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context)
        -> Result<bool, DecodeError>;
    // /// The method is called when unknown property is encountered by parent
    // /// structure
    // ///
    // /// Returns `Ok(true)` if the property is "consumed" (i.e. stored in this
    // /// structure).
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context)
    //     -> Result<bool, DecodeError>;
}

/// The trait that decodes scalar value and checks its type
pub trait DecodeScalar: Sized + Clone {
    /// Decode the value and typecheck
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError>;
}

/// Trait to encode the ast into KDL node
pub trait Encode: Decode {
    /// Encodes the ast from the node
    fn encode(&self, ctx: &mut Context) -> Result<Node, EncodeError>;
}

/// TODO(rnarkk)
pub trait EncodePartial: DecodePartial {
    /// TODO(rnarkk)
    fn encode_partial(&self, node: &mut Node, ctx: &mut Context)
        -> Result<(), EncodeError>;
}

/// The trait that encodes scalar value and checks its type
pub trait EncodeScalar: DecodeScalar {
    /// TODO(rnarkk)
    fn encode(&self, ctx: &mut Context) -> Result<Scalar, EncodeError>;
}
