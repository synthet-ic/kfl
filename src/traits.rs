//! Traits used for the library
//!
//! Most users will never implement these manually. See
//! [`Decode`](derive@crate::Decode)` and
//! [`DecodeScalar`](derive@crate::DecodeScalar) for a
//! documentation of the derives to implement these traits.
use std::fmt::Debug;

use crate::{
    ast::{Node, Scalar},
    errors::{DecodeError, EncodeError},
    context::Context
};

/// Trait to decode KDL node from the AST
pub trait Decode<S: ErrorSpan>: Sized {
    /// Decodes the node from the ast
    fn decode(node: &Node, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>;
}

/// Trait to decode children of the KDL node, mostly used for root document
pub trait DecodeChildren<S: ErrorSpan>: Sized {
    /// Decodes from a list of chidren ASTs
    fn decode_children(nodes: &[Node], ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>;
}

/// The trait is implemented for structures that can be used as part of other
/// structs
///
/// The type of field that `#[kfl(flatten)]` is used for should implement
/// this trait. It is automatically implemented by `#[derive(Decode)]`
/// by structures that have only optional properties and children (no
/// arguments).
pub trait DecodePartial<S: ErrorSpan>: Sized + Default {
    /// The method is called when unknown child is encountered by parent
    /// structure
    ///
    /// Returns `Ok(true)` if the child is "consumed" (i.e. stored in this
    /// structure).
    fn decode_partial(&mut self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, DecodeError<S>>;
    // /// The method is called when unknown property is encountered by parent
    // /// structure
    // ///
    // /// Returns `Ok(true)` if the property is "consumed" (i.e. stored in this
    // /// structure).
    // fn insert_property(&mut self,
    //                    name: &Box<str>, scalar: &Scalar,
    //                    ctx: &mut Context<S>)
    //     -> Result<bool, DecodeError<S>>;
}

/// The trait that decodes scalar value and checks its type
pub trait DecodeScalar<S: ErrorSpan>: Sized {
    // /// Typecheck the value
    // ///
    // /// This method can only emit errors to the context in type mismatch case.
    // /// Errors emitted to the context are considered fatal once the whole data
    // /// is processed but non fatal when encountered. So even if there is a type
    // /// in type name we can proceed and try parsing actual value.
    // #[allow(unused)]
    // fn type_check(type_name: &Option<TypeName>,
    //               ctx: &mut Context<S>) {}
    // /// Decode value without typecheck
    // ///
    // /// This can be used by wrappers to parse some known value but use a
    // /// different typename (kinda emulated subclassing)
    // fn raw_decode(value: &Literal, ctx: &mut Context<S>)
    //     -> Result<Self, DecodeError<S>>;
    /// Decode the value and typecheck
    ///
    /// This should not be overriden and uses `type_check` in combination with
    /// `raw_decode`.
    fn decode(scalar: &Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>;
}

/// Span must implement this trait to be used in the error messages
///
/// Custom span types can be used for this unlike for [`Span`]
pub trait ErrorSpan: Into<miette::SourceSpan>
                     + Clone + Debug + Send + Sync + 'static {}
impl<T> ErrorSpan for T
    where T: Into<miette::SourceSpan>,
          T: Clone + Debug + Send + Sync + 'static,
{}

/// Span trait used for parsing source code
///
/// It's sealed because needs some tight interoperation with the parser. Use
/// [`DecodeSpan`] to convert spans whenever needed.
pub trait Span: sealed::Sealed + chumsky::Span + ErrorSpan {}

/// Trait to encode the ast into KDL node
pub trait Encode<S: ErrorSpan>: Decode<S> {
    /// Encodes the ast from the node
    fn encode(&self, ctx: &mut Context<S>)
        -> Result<Node, EncodeError<S>>;
}

///
pub trait EncodePartial<S: ErrorSpan>: DecodePartial<S> {
    ///
    fn encode_partial(&self, node: &Node, ctx: &mut Context<S>)
        -> Result<bool, EncodeError<S>>;
}

///
pub trait EncodeChildren<S: ErrorSpan, T: DecodeChildren<S>> {
    ///
    fn encode_children(nodes: &[T], ctx: &mut Context<S>)
        -> Result<Vec<Node>, EncodeError<S>>;
}

/// The trait that encodes scalar value and checks its type
pub trait EncodeScalar<S: ErrorSpan>: DecodeScalar<S> {
    ///
    fn encode(&self, ctx: &mut Context<S>)
        -> Result<Scalar, EncodeError<S>>;
}

#[allow(missing_debug_implementations)]
pub(crate) mod sealed {
    pub type Stream<'a, S, T> = chumsky::Stream<
        'a, char, S, Map<std::str::Chars<'a>, T>
    >;

    pub struct Map<I, F>(pub(crate) I, pub(crate) F);

    pub trait SpanTracker {
        type Span;
        fn next_span(&mut self, c: char) -> Self::Span;
    }

    impl<I, T> Iterator for Map<I, T>
         where I: Iterator<Item = char>,
               T: SpanTracker,
    {
        type Item = (char, T::Span);
        fn next(&mut self) -> Option<(char, T::Span)> {
            self.0.next().map(|c| (c, self.1.next_span(c)))
        }
    }

    pub trait Sealed {
        type Tracker: SpanTracker<Span = Self>;
        /// Note assuming ascii, single-width, non-newline chars here
        fn at_start(&self, chars: usize) -> Self;
        fn at_end(&self) -> Self;
        /// Note assuming ascii, single-width, non-newline chars here
        fn before_start(&self, chars: usize) -> Self;
        fn length(&self) -> usize;

        fn stream(s: &str) -> Stream<'_, Self, Self::Tracker>
            where Self: chumsky::Span;
    }
}
