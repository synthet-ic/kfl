//! Decode support stuff
//!
//! Mostly useful for manual implementation of various `Decode*` traits.
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    ast::Literal,
    errors::DecodeError,
    traits::ErrorSpan
};

/// Context is passed through all the decode operations and can be used for:
///
/// 1. To emit error and proceed (so multiple errors presented to user)
/// 2. To store and retrieve data in decoders of nodes, scalars and spans
#[derive(Debug)]
pub struct Context<S: ErrorSpan> {
    spans: HashMap<&str, S>,
    errors: Vec<DecodeError<S>>,
    extensions: HashMap<TypeId, Box<dyn Any>>,
}

impl<S: ErrorSpan> Context<S> {
    pub(crate) fn new() -> Context<S> {
        Context {
            errors: Vec::new(),
            extensions: HashMap::new(),
        }
    }
    ///
    pub(crate) fn set_span<P: Pointer>(&mut self, pointer: &P, span: S) {
        self.spans.insert(format!("{:p}", pointer).as_str(), span);
    }
    ///
    pub fn span(&self, pointer: &P) -> S {
        self[format!("{:p}", pointer).as_str()]
    }
    /// Add error
    ///
    /// This fails decoding operation similarly to just returning error value.
    /// But unlike result allows returning some dummy value and allows decoder
    /// to proceed so multiple errors are presented to user at the same time.
    pub fn emit_error(&mut self, err: impl Into<DecodeError<S>>) {
        self.errors.push(err.into());
    }
    /// Returns `true` if any errors was emitted into the context
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub(crate) fn into_errors(self) -> Vec<DecodeError<S>> {
        self.errors
    }
    /// Set context value
    ///
    /// These values aren't used by the kfl itself. But can be used by
    /// user-defined decoders to get some value. Each type can have a single but
    /// separate value set. So users are encouraged to use [new type idiom
    /// ](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
    /// to avoid conflicts with other libraries.
    ///
    /// It's also discourated to use `set` in the decoder. It's expeced that
    /// context will be filled in using
    /// [`parse_with_context`](crate::parse_with_context) function.
    pub fn set<T: 'static>(&mut self, value: T) {
        self.extensions.insert(TypeId::of::<T>(), Box::new(value));
    }
    /// Get context value
    ///
    /// Returns a value previously set in context
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.extensions.get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }
}

impl Literal {
    /// Returns the string representation of `Literal`
    ///
    /// This is currently used in error messages.
    pub const fn as_str(&self) -> &'static str {
        use Literal::*;
        match self {
            Int(_) => "integer",
            Decimal(_) => "decimal",
            String(_) => "string",
            Bool(_) => "boolean",
            Null => "null",
        }
    }
}
