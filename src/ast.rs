//! Structures that represent abstract syntax tree (AST) of the KDL document
//!
//! All of these types are parameterised by the `S` type which is a span type
//! (perhaps implements [`Span`](crate::traits::Span). The idea is that most of
//! the time spans are used for errors (either at parsing time, or at runtime),
//! and original source is somewhere around to show in error snippets. So it's
//! faster to only track byte offsets and calculate line number and column when
//! printing code snippet. So use [`span::Span`](crate::traits::Span).
//!
//! But sometimes you will not have KDL source around, or performance of
//! priting matters (i.e. you log source spans). In that case, span should
//! contain line and column numbers for things, use
//! [`LineSpan`](crate::span::LineSpan) for that.

use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
    vec::Vec
};
use core::fmt::{self, Debug, Pointer};

use crate::own;

/// Single node of the KDL document
#[derive(Debug, Clone)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct Node {
    /// A type name if specified in parenthesis
    #[cfg_attr(feature = "minicbor", n(0))]
    pub type_name: Option<Box<str>>,
    /// A node name
    #[cfg_attr(feature = "minicbor", n(1))]
    pub node_name: Box<str>,
    /// Positional arguments
    #[cfg_attr(feature = "minicbor", n(2))]
    pub arguments: Vec<Scalar>,
    /// Named properties
    #[cfg_attr(feature = "minicbor", n(3))]
    pub properties: BTreeMap<Box<str>, Scalar>,
    /// Node's children. This field is not none if there are braces `{..}`
    #[cfg_attr(feature = "minicbor", n(4))]
    pub children: Option<Vec<Node>>,
}

/// Possibly typed KDL scalar value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct Scalar {
    /// A type name if specified in parenthesis
    #[cfg_attr(feature = "minicbor", n(0))]
    pub type_name: Option<Box<str>>,
    /// The actual value literal
    #[cfg_attr(feature = "minicbor", n(1))]
    pub literal: Box<str>,
}

impl Node {
    /// TODO(rnarkk) document
    pub fn new(name: &str) -> Self {
        Self {
            type_name: None,
            node_name: own!(name),
            arguments: Vec::new(),
            properties: BTreeMap::new(),
            children: None,
        }
    }
    /// Returns node children
    pub fn children(&self)
        -> impl Iterator<Item = &Node> +
                ExactSizeIterator
    {
        self.children.as_ref().map(|c| c.iter()).unwrap_or_else(|| [].iter())
    }
}

impl Scalar {
    ///
    pub fn new(type_name: Box<str>, literal: Box<str>) -> Self {
        Self { type_name: Some(type_name), literal }
    }
}

impl From<Box<str>> for Scalar {
    fn from(value: Box<str>) -> Self {
        Scalar { type_name: None, literal: value }
    }
}

macro_rules! impl_pointer {
    ($ty:ty) => {
        impl Pointer for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let ptr = self as *const Self;
                Pointer::fmt(&ptr, f)
            }
        }
    };
}

impl_pointer!(Node);
impl_pointer!(Scalar);
