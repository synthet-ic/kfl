//! Traits used for the library
//!
//! Most users will never implement these manually. See
//! [`Decode`](derive@crate::Decode)` and
//! [`DecodeScalar`](derive@crate::DecodeScalar) for a
//! documentation of the derives to implement these traits.

use alloc::vec::Vec;
use crate::{
    ast::{Node, Scalar},
    errors::{DecodeError, EncodeError},
    context::Context
};

/// Trait to decode KDL node from the AST
pub trait Decode: Sized {
    /// Decodes the node from the ast
    fn decode(node: &Node, ctx: &mut Context)
        -> Result<Self, DecodeError>;
}

/// Trait to decode children of the KDL node, mostly used for root document
pub trait DecodeChildren: Sized {
    /// Decodes from a list of chidren ASTs
    fn decode_children(nodes: &[Node], ctx: &mut Context)
        -> Result<Self, DecodeError>;
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
pub trait DecodeScalar: Sized {
    // /// Typecheck the value
    // ///
    // /// This method can only emit errors to the context in type mismatch case.
    // /// Errors emitted to the context are considered fatal once the whole data
    // /// is processed but non fatal when encountered. So even if there is a type
    // /// in type name we can proceed and try parsing actual value.
    // #[allow(unused)]
    // fn type_check(type_name: &Option<TypeName>,
    //               ctx: &mut Context) {}
    // /// Decode value without typecheck
    // ///
    // /// This can be used by wrappers to parse some known value but use a
    // /// different typename (kinda emulated subclassing)
    // fn raw_decode(value: &Literal, ctx: &mut Context)
    //     -> Result<Self, DecodeError>;
    /// Decode the value and typecheck
    ///
    /// This should not be overriden and uses `type_check` in combination with
    /// `raw_decode`.
    fn decode(scalar: &Scalar, ctx: &mut Context)
        -> Result<Self, DecodeError>;
}

/// Trait to encode the ast into KDL node
pub trait Encode: Decode {
    /// Encodes the ast from the node
    fn encode(&self, ctx: &mut Context)
        -> Result<Node, EncodeError>;
}

/// TODO(rnarkk)
pub trait EncodePartial: DecodePartial {
    /// TODO(rnarkk)
    fn encode_partial(&self, node: &mut Node, ctx: &mut Context)
        -> Result<(), EncodeError>;
}

/// TODO(rnarkk)
pub trait EncodeChildren: DecodeChildren {
    /// TODO(rnarkk)
    fn encode_children(&self, ctx: &mut Context)
        -> Result<Vec<Node>, EncodeError>;
}

/// The trait that encodes scalar value and checks its type
pub trait EncodeScalar: DecodeScalar {
    /// TODO(rnarkk)
    fn encode(&self, ctx: &mut Context)
        -> Result<Scalar, EncodeError>;
}
