//! Error types for the kfl library
//!
//! You only need [`Error`](enum@Error) exposed as `kfl::Error` unless you
//! do manual implementations of any of the `Decode*` traits.
extern crate std;

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::BTreeSet,
    format,
    string::String,
    vec,
    vec::Vec
};
use core::fmt::{self, Display, Write};

use thiserror::Error;
use miette::{Diagnostic, NamedSource};

use crate::span::Span;

/// Main error that is returned from KDL parsers
///
/// Implements [`miette::Diagnostic`] so can be used to print nice error
/// output with code snippets.
///
/// See [crate documentation](crate#Errors) and [miette] documentation to
/// find out how to deal with them.
#[derive(Debug, Diagnostic, Error)]
#[error("error parsing KDL")]
pub struct Error {
    #[source_code]
    pub(crate) source_code: NamedSource,
    #[related]
    pub(crate) errors: Vec<miette::Error>,
}

/// An error type that is returned by decoder traits and emitted to the context
///
/// These are elements of the
#[derive(Debug, Diagnostic, Error)]
#[non_exhaustive]
pub enum DecodeError {
    /// Unexpected type name encountered
    ///
    /// Type names are identifiers and strings in parenthesis before node names
    /// or values.
    #[error("{} for {}, found {}", expected, rust_type,
            found.as_ref().map(|x| x.as_ref()).unwrap_or("no type name"))]
    #[diagnostic()]
    TypeName {
        /// Position of the type name
        #[label = "unexpected type name"]
        span: Span,
        /// Type name contained in the source code
        found: Option<Box<str>>,
        /// Expected type name or type names
        expected: ExpectedType,
        /// Rust type that is being decoded when error is encountered
        rust_type: &'static str,
    },
    /// Different scalar kind was encountered than expected
    ///
    /// This is emitted when integer is used instead of string, and similar. It
    /// may also be encountered when `null` is used for non-optional field.
    #[diagnostic()]
    #[error("expected {} scalar, found {}", expected, found)]
    ScalarKind {
        /// Position of the unexpected scalar
        #[label("unexpected {}", found)]
        span: Span,
        /// Scalar kind (or multiple) expected at this position
        expected: &'static str,
        /// Kind of scalar that is found
        found: Box<str>,
    },
    /// Some required element is missing
    ///
    /// This is emitted on missing required attributes, properties, or children.
    /// (missing type names are emitted using [`DecodeError::TypeName`])
    #[diagnostic()]
    #[error("{}", message)]
    Missing {
        /// Position of the node name of which has missing element
        #[label("node starts here")]
        span: Span,
        /// Description of what's missing
        message: String,
    },
    /// Missing named node at top level
    ///
    /// This is similar to `Missing` but is only emitted for nodes on the
    /// document level. This is separate error because there is no way to show
    /// span where missing node is expected (end of input is not very helpful).
    #[diagnostic()]
    #[error("{}", message)]
    MissingNode {
        /// Descriptino of what's missing
        message: String,
    },
    /// Unexpected entity encountered
    ///
    /// This is emitted for entities (arguments, properties, children) that have
    /// to matching structure field to put into, and also for nodes that aren't
    /// expected to be encountered twice.
    #[diagnostic()]
    #[error("{}", message)]
    Unexpected {
        /// Position of the unexpected element
        #[label("unexpected {}", kind)]
        span: Span,
        /// Kind of element that was found
        kind: &'static str,
        /// Description of the error
        message: String,
    },
    /// Bad scalar conversion
    ///
    /// This error is emitted when some scalar value of right kind cannot be
    /// converted to the Rust value. Including, but not limited to:
    /// 1. Integer value out of range
    /// 2. `FromStr` returned error for the value parse by
    ///    `#[kfl(.., str)]`
    #[error("{}", source)]
    #[diagnostic()]
    Conversion {
        /// Position of the scalar that could not be converted
        #[label("invalid value")]
        span: Span,
        /// Original error
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    /// Unsupported value
    ///
    /// This is currently used to error out on `(base64)` values when `base64`
    /// feature is not enabled.
    #[error("{}", message)]
    #[diagnostic()]
    Unsupported {
        /// Position of the value that is unsupported
        #[label = "unsupported value"]
        span: Span,
        /// Description of why the value is not supported
        message: Cow<'static, str>,
    },
    /// Custom error that can be emitted during decoding
    ///
    /// This is not used by the kfl itself. Note most of the time it's
    /// better to use [`DecodeError::Conversion`] as that will associate
    /// source code span to the error.
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync + 'static>),
}

///
#[allow(dead_code)]
#[derive(Debug, Diagnostic, Error)]
#[non_exhaustive]
pub enum EncodeError {
    /// TODO(rnarkk)
    #[diagnostic()]
    #[error("{} is a skipped variant", found)]
    ExtraVariant {
        ///
        found: String,
    },
    ///
    #[diagnostic()]
    #[error("{}", message)]
    Unexpected {
        // /// Position of the unexpected element
        // #[label("unexpected {}", kind)]
        // span: Span,
        /// Kind of element that was found
        kind: &'static str,
        /// Description of the error
        message: String,
    },
    ///
    #[diagnostic()]
    #[error(transparent)]
    Custom(Box<dyn std::error::Error + Send + Sync + 'static>)
}

///
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum TokenFormat {
    Char(char),
    Token(&'static str),
    Kind(&'static str),
    OpenRaw(usize),
    CloseRaw(usize),
    Eoi,
}

struct FormatUnexpected<'a>(&'a TokenFormat, &'a BTreeSet<TokenFormat>);

#[derive(Debug, Diagnostic, Error)]
pub(crate) enum ParseError {
    #[error("{}", FormatUnexpected(found, expected))]
    #[diagnostic()]
    Unexpected {
        label: Option<&'static str>,
        #[label("{}", label.unwrap_or("unexpected token"))]
        span: Span,
        found: TokenFormat,
        expected: BTreeSet<TokenFormat>,
    },
    #[error("unclosed {} {}", label, opened)]
    #[diagnostic()]
    Unclosed {
        label: &'static str,
        #[label = "opened here"]
        opened_at: Span,
        opened: TokenFormat,
        #[label("expected {}", expected)]
        expected_at: Span,
        expected: TokenFormat,
        found: TokenFormat,
    },
    #[error("{}", message)]
    #[diagnostic()]
    Message {
        label: Option<&'static str>,
        #[label("{}", label.unwrap_or("unexpected token"))]
        span: Span,
        message: String,
    },
}

///
#[allow(dead_code)]
#[derive(Debug, Diagnostic, Error)]
pub(crate) enum PrintError {
    ///
    #[diagnostic()]
    #[error("{}", message)]
    Unexpected {
        /// Position of the unexpected element
        #[label("unexpected {}", kind)]
        span: Span,
        /// Kind of element that was found
        kind: &'static str,
        /// Description of the error
        message: String,
    },
}

impl From<Option<char>> for TokenFormat {
    fn from(chr: Option<char>) -> TokenFormat {
        if let Some(chr) = chr {
            TokenFormat::Char(chr)
        } else {
            TokenFormat::Eoi
        }
    }
}

impl From<char> for TokenFormat {
    fn from(chr: char) -> TokenFormat {
        TokenFormat::Char(chr)
    }
}

impl From<&'static str> for TokenFormat {
    fn from(s: &'static str) -> TokenFormat {
        TokenFormat::Token(s)
    }
}

impl Display for TokenFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenFormat::*;
        match self {
            // do not escape quotes as we use backticks
            Char('"') => write!(f, "`\"`"),
            Char('\'') => write!(f, "`\'`"),
            // also single backslash should not confuse anybody in this context
            Char('\\') => write!(f, r"`\`"),

            Char(c) => write!(f, "`{}`", c.escape_default()),
            Token(s) => write!(f, "`{}`", s.escape_default()),
            Kind(s) => write!(f, "{}", s),
            Eoi => write!(f, "end of input"),
            OpenRaw(0) => {
                f.write_str("`r\"`")
            }
            OpenRaw(n) => {
                f.write_str("`r")?;
                for _ in 0..*n {
                    f.write_char('#')?;
                }
                f.write_str("\"`")
            }
            CloseRaw(0) => {
                f.write_str("`\"`")
            }
            CloseRaw(n) => {
                f.write_str("`\"")?;
                for _ in 0..*n {
                    f.write_char('#')?;
                }
                f.write_char('`')
            }
        }
    }
}

impl Display for FormatUnexpected<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "found {}", self.0)?;
            let mut iter = self.1.iter();
        if let Some(item) = iter.next() {
            write!(f, ", expected {}", item)?;
            let back = iter.next_back();
            for item in iter {
                write!(f, ", {}", item)?;
            }
            if let Some(item) = back {
                write!(f, " or {}", item)?;
            }
        }
        Ok(())
    }
}

impl ParseError {
    pub(crate) fn with_expected_kind(mut self, token: &'static str) -> Self {
        match &mut self {
            ParseError::Unexpected { ref mut expected, .. } => {
                *expected = [TokenFormat::Kind(token)].into_iter().collect();
            }
            _ => {},
        }
        self
    }
    pub(crate) fn with_no_expected(mut self) -> Self {
        match &mut self {
            ParseError::Unexpected { ref mut expected, .. } => {
                *expected = BTreeSet::new();
            }
            _ => {},
        }
        self
    }
}

use chumsky::input::Input;

impl<'a> chumsky::error::Error<'a, &'a str> for ParseError {
    fn expected_found<E>(expected: E, found: Option<char>, span: <&'a str as Input<'a>>::Span)
        -> Self
        where E: IntoIterator<Item = Option<char>>
    {
        ParseError::Unexpected {
            label: None,
            span: span.into(),
            found: found.into(),
            expected: expected.into_iter().map(Into::into).collect(),
        }
    }
    fn merge(mut self, other: Self) -> Self {
        use ParseError::*;
        match (&mut self, other) {
            (Unclosed { .. }, _) => self,
            (_, other@Unclosed { .. }) => other,
            (Unexpected { expected: ref mut dest, .. },
             Unexpected { expected, .. }) => {
                dest.extend(expected.into_iter());
                self
            }
            (_, other) => todo!("{} -> {}", self, other),
        }
    }
    // fn unclosed_delimiter(
    //     unclosed_span: Self::Span,
    //     unclosed: char,
    //     span: Self::Span,
    //     expected: char,
    //     found: Option<char>
    // ) -> Self {
    //     ParseError::Unclosed {
    //         label: "delimited",
    //         opened_at: unclosed_span,
    //         opened: unclosed.into(),
    //         expected_at: span,
    //         expected: expected.into(),
    //         found: found.into(),
    //     }
    // }
}

impl DecodeError {
    /// Construct [`DecodeError::Conversion`] error
    pub fn conversion<E>(span: Span, err: E) -> Self
        where E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        DecodeError::Conversion {
            span,
            source: err.into(),
        }
    }
    /// Construct [`DecodeError::ScalarKind`] error
    pub fn scalar_kind(span: Span, expected: &'static str, found: Box<str>) -> Self {
        DecodeError::ScalarKind {
            span,
            expected,
            found,
        }
    }
    /// Construct [`DecodeError::Missing`] error
    pub fn missing(span: Span, message: impl Into<String>) -> Self {
        DecodeError::Missing {
            span,
            message: message.into(),
        }
    }
    /// Construct [`DecodeError::Unexpected`] error
    pub fn unexpected(span: Span, kind: &'static str,
                      message: impl Into<String>)
        -> Self
    {
        DecodeError::Unexpected {
            span,
            kind,
            message: message.into(),
        }
    }
    /// Construct [`DecodeError::Unsupported`] error
    pub fn unsupported<T, M>(span: Span, message: M)-> Self
        where M: Into<Cow<'static, str>>,
    {
        DecodeError::Unsupported {
            span,
            message: message.into(),
        }
    }
}

/// Wrapper around expected type that is used in [`DecodeError::TypeName`].
#[derive(Debug)]
pub struct ExpectedType {
    types: Vec<Box<str>>,
    no_type: bool,
}

impl ExpectedType {
    /// Declare that decoder expects no type (no parens at all) for the value
    pub fn no_type() -> Self {
        ExpectedType {
            types: [].into(),
            no_type: true,
        }
    }
    /// Declare the type that has to be attached to the value
    pub fn required(ty: impl Into<Box<str>>) -> Self {
        ExpectedType {
            types: vec![ty.into()],
            no_type: false,
        }
    }
    /// Declare the type that can be attached to the value
    ///
    /// But no type is also okay in this case (although, "no type" and specified
    /// type can potentially have different meaning).
    pub fn optional(ty: impl Into<Box<str>>) -> Self {
        ExpectedType {
            types: vec![ty.into()],
            no_type: true,
        }
    }
}

impl Display for ExpectedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.types.is_empty() {
            write!(f, "no type")
        } else {
            let mut iter = self.types.iter();
            if let Some(first) = iter.next() {
                write!(f, "{}", first)?;
            }
            let last = if self.no_type {
                None
            } else {
                iter.next_back()
            };
            for item in iter {
                write!(f, ", {}", item)?;
            }
            if self.no_type {
                write!(f, " or no type")?;
            } else if let Some(last) = last {
                write!(f, " or {}", last)?;
            }
            Ok(())
        }
    }
}

impl EncodeError {
    /// TODO(rnarkk)
    pub fn extra_variant(found: impl Into<String>) -> Self {
        EncodeError::ExtraVariant {
            found: found.into(),
        }
    }
}
