//! Decode support stuff
//!
//! Mostly useful for manual implementation of various `Decode*` traits.

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    vec::Vec
};
use core::{
    any::{Any, TypeId}, 
    fmt::{Pointer, Debug}
};

use crate::{
    errors::DecodeError,
    span::Span,
};

/// Context is passed through all the decode operations and can be used for:
///
/// 1. To emit error and proceed (so multiple errors presented to user)
/// 2. To store and retrieve data in decoders of nodes, scalars and spans
#[derive(Debug, Default)]
pub struct Context {
    ///
    pub spans: BTreeMap<Box<str>, Span>,
    /// 
    pub errors: Vec<DecodeError>,
    extensions: BTreeMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub(crate) fn new() -> Context {
        Context {
            spans: BTreeMap::new(),
            errors: Vec::new(),
            extensions: BTreeMap::new(),
        }
    }
    ///
    pub(crate) fn set_span<P: Pointer + Debug>(&mut self, pointer: &P, span: Span) {
        // println!("SET {0:?} {0:p}", pointer);
        self.spans.insert(format!("{:p}", pointer).into_boxed_str(), span);
        // println!("{:#?}", &self.spans);
    }
    ///
    #[allow(unused_variables)]
    pub fn span<P: Pointer + Debug>(&self, pointer: &P) -> Span {
        // println!("GET {0:?} {0:p}", pointer);
        Span(0, 0)
        // self.spans[&format!("{:p}", pointer).into_boxed_str()].clone()
    }
    /// Add error
    ///
    /// This fails decoding operation similarly to just returning error value.
    /// But unlike result allows returning some dummy value and allows decoder
    /// to proceed so multiple errors are presented to user at the same time.
    pub fn emit_error(&mut self, err: impl Into<DecodeError>) {
        self.errors.push(err.into());
    }
    /// Returns `true` if any errors was emitted into the context
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
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
