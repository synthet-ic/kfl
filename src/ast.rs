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

use std::{
    collections::BTreeMap,
    convert::Infallible,
    fmt::{Debug, Display, Pointer},
    ops::Deref,
    str::FromStr
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[cfg_attr(feature = "minicbor", cbor(index_only))]
pub(crate) enum Radix {
    #[cfg_attr(feature = "minicbor", n(2))]
    Bin,
    #[cfg_attr(feature = "minicbor", n(16))]
    Hex,
    #[cfg_attr(feature = "minicbor", n(8))]
    Oct,
    #[cfg_attr(feature = "minicbor", n(10))]
    Dec,
}

/// Potentially unlimited size integer value
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct Integer(
    #[cfg_attr(feature = "minicbor", n(0))]
    pub(crate) Radix,
    #[cfg_attr(feature = "minicbor", n(1))]
    pub(crate) Box<str>,
);

/// Potentially unlimited precision decimal value
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
#[cfg_attr(feature = "minicbor", cbor(transparent))]
pub struct Decimal(
    #[cfg_attr(feature = "minicbor", n(0))]
    pub(crate) Box<str>,
);

/// Possibly typed KDL scalar value
#[derive(Debug, Clone)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct Scalar {
    /// A type name if specified in parenthesis
    #[cfg_attr(feature = "minicbor", n(0))]
    pub type_name: Option<TypeName>,
    /// The actual value literal
    #[cfg_attr(feature = "minicbor", n(1))]
    pub literal: Literal,
}

/// Type identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName(TypeNameInner);

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeNameInner {
    Builtin(BuiltinType),
    Custom(Box<str>),
}

/// Known type identifier described by the KDL specification
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinType {
    /// `u8`: 8-bit unsigned integer type
    U8,
    /// `i8`: 8-bit signed integer type
    I8,
    /// `u16`: 16-bit unsigned integer type
    U16,
    /// `i16`: 16-bit signed integer type
    I16,
    /// `u32`: 32-bit unsigned integer type
    U32,
    /// `i32`: 32-bit signed integer type
    I32,
    /// `u64`: 64-bit unsigned integer type
    U64,
    /// `i64`: 64-bit signed integer type
    I64,
    /// `usize`: platform-dependent unsigned integer type
    Usize,
    /// `isize`: platform-dependent signed integer type
    Isize,
    /// `f32`: 32-bit floating point type
    F32,
    /// `f64`: 64-bit floating point type
    F64,
    /// `base64` denotes binary bytes type encoded using base64 encoding
    Base64,
}

/// Scalar KDL value
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub enum Literal {
    /// Null value
    #[cfg_attr(feature = "minicbor", n(0))]
    Null,
    /// Boolean value
    #[cfg_attr(feature = "minicbor", n(1))]
    Bool(
        #[cfg_attr(feature = "minicbor", n(0))]
        bool
    ),
    /// Integer value
    #[cfg_attr(feature = "minicbor", n(2))]
    Int(
        #[cfg_attr(feature = "minicbor", n(0))]
        Integer
    ),
    /// Decimal (or floating point) value
    #[cfg_attr(feature = "minicbor", n(3))]
    Decimal(
        #[cfg_attr(feature = "minicbor", n(0))]
        Decimal
    ),
    /// String value
    #[cfg_attr(feature = "minicbor", n(4))]
    String(
        #[cfg_attr(feature = "minicbor", n(0))]
        Box<str>
    ),
}

impl Node {
    /// Returns node children
    pub fn children(&self)
        -> impl Iterator<Item = &Node> +
                ExactSizeIterator
    {
        self.children.as_ref().map(|c| c.iter()).unwrap_or_else(|| [].iter())
    }
}

macro_rules! impl_pointer {
    ($ty:ty) => {
        impl Pointer for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ptr = self as *const Self;
                Pointer::fmt(&ptr, f)
            }
        }
    };
}

impl_pointer!(Node);
impl_pointer!(Scalar);
impl_pointer!(Literal);
impl_pointer!(TypeName);

impl BuiltinType {
    /// Returns string representation of the builtin type as defined by KDL
    /// specification
    pub const fn as_str(&self) -> &'static str {
        use BuiltinType::*;
        match self {
            U8 => "u8",
            I8 => "i8",
            U16 => "u16",
            I16 => "i16",
            U32 => "u32",
            I32 => "i32",
            U64 => "u64",
            I64 => "i64",
            Usize => "usize",
            Isize => "isize",
            F32 => "f32",
            F64 => "f64",
            Base64 => "base64",
        }
    }
    /// Returns `TypeName` structure for the builtin type
    pub const fn as_type(self) -> TypeName {
        TypeName(TypeNameInner::Builtin(self))
    }
}

impl TypeName {
    pub(crate) fn from_string(val: Box<str>) -> TypeName {
        use TypeNameInner::*;

        match BuiltinType::from_str(&val[..]) {
            Ok(b) => TypeName(Builtin(b)),
            _ => TypeName(Custom(val)),
        }
    }
    /// Returns string represenation of the type name
    pub fn as_str(&self) -> &str {
        match &self.0 {
            TypeNameInner::Builtin(t) => t.as_str(),
            TypeNameInner::Custom(t) => t.as_ref(),
        }
    }
    /// Returns `BuiltinType` enum for the type if typename matches builtin
    /// type
    ///
    /// Note: checking for `is_none()` is not forward compatible. In future we
    /// may add additional builtin type. Always use `as_str` for types that
    /// aren't yet builtin.
    pub const fn as_builtin(&self) -> Option<&BuiltinType> {
        match &self.0 {
            TypeNameInner::Builtin(t) => Some(t),
            TypeNameInner::Custom(_) => None,
        }
    }
}

impl FromStr for BuiltinType {
    type Err = ();
    fn from_str(s: &str) -> Result<BuiltinType, ()> {
        use BuiltinType::*;
        match s {
            "u8" => Ok(U8),
            "i8" => Ok(I8),
            "u16" => Ok(U16),
            "i16" => Ok(I16),
            "u32" => Ok(U32),
            "i32" => Ok(I32),
            "u64" => Ok(U64),
            "i64" => Ok(I64),
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "base64" => Ok(Base64),
            _ => Err(())
        }
    }
}

impl FromStr for TypeName {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<TypeName, Infallible> {
        use TypeNameInner::*;
        match BuiltinType::from_str(s) {
            Ok(b) => Ok(TypeName(Builtin(b))),
            Err(()) => Ok(TypeName(Custom(s.into()))),
        }
    }
}

impl Deref for TypeName {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.as_str(), f)
    }
}

impl Into<TypeName> for BuiltinType {
    fn into(self) -> TypeName {
        self.as_type()
    }
}

#[cfg(feature = "minicbor")]
mod cbor {
    use super::TypeName;
    use minicbor::{Decoder, Encoder};
    use minicbor::encode::Encode;
    use minicbor::decode::Decode;

    impl<'d, C> Decode<'d, C> for TypeName {
        fn decode(d: &mut Decoder<'d>, _: &mut C)
            -> Result<Self, minicbor::decode::Error>
        {
            d.str().and_then(|s| s.parse().map_err(|e| match e {}))
        }
    }
    impl<C> Encode<C> for TypeName {
        fn encode<W>(&self, e: &mut Encoder<W>, ctx: &mut C)
            -> Result<(), minicbor::encode::Error<W::Error>>
            where W: minicbor::encode::write::Write
        {
            self.as_str().encode(e, ctx)
        }
    }
}
