//! kfl supports to kinds of the span for parsing
//!
//! 1. [`Span`] which only tracks byte offset from the start of the source code
//! 2. [`LineSpan`] which also track line numbers
//!
//! This distinction is important during parsing stage as [`Span`] is normally
//! faster. And [`LineSpan`] is still faster than find out line/column number
//! for each span separately, and is also more convenient if you need this
//! information.
//!
//! On the other hand, on the decode stage you can convert your span types into
//! more elaborate thing that includes file name or can refer to the defaults
//! as a separate kind of span. See [`traits::DecodeSpan`].

use std::{
    fmt::Display,
    ops::Range
};

// use crate::traits;

/// Reexport of [miette::SourceSpan] trait that we use for parsing
pub use miette::SourceSpan;

/// Normal byte offset span
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct Span(
    #[cfg_attr(feature = "minicbor", n(0))]
    pub usize,
    #[cfg_attr(feature = "minicbor", n(1))]
    pub usize,
);

/// Line and column position of the datum in the source code
// TODO(tailhook) optimize Eq to check only offset
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct LinePos {
    /// Zero-based byte offset
    #[cfg_attr(feature = "minicbor", n(0))]
    pub offset: usize,
    /// Zero-based line number
    #[cfg_attr(feature = "minicbor", n(1))]
    pub line: usize,
    /// Zero-based column number
    #[cfg_attr(feature = "minicbor", n(2))]
    pub column: usize,
}

/// Span with line and column number
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "minicbor", derive(minicbor::Encode, minicbor::Decode))]
pub struct LineSpan(
    #[cfg_attr(feature = "minicbor", n(0))]
    pub LinePos,
    #[cfg_attr(feature = "minicbor", n(1))]
    pub LinePos,
);

impl Span {
    /// Length of the span in bytes
    pub fn len(&self) -> usize {
        self.1.saturating_sub(self.0)
    }
    ///
    pub fn at_start(&self, chars: usize) -> Self {
        Span(self.0, self.0 + chars)
    }
    ///
    pub fn at_end(&self) -> Self {
        Span(self.1, self.1)
    }
    ///
    pub fn before_start(&self, chars: usize) -> Self {
        Span(self.0.saturating_sub(chars), self.0)
    }
}

impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        (self.0, self.1.saturating_sub(self.0)).into()
    }
}

impl From<chumsky::zero_copy::span::SimpleSpan<usize>> for Span {
    fn from(value: chumsky::zero_copy::span::SimpleSpan<usize>) -> Self {
        Self(value.start, value.end)
    }
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Span {
        Span(r.start, r.end)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)?;
        "..".fmt(f)?;
        self.1.fmt(f)?;
        Ok(())
    }
}

impl Into<SourceSpan> for LineSpan {
    fn into(self) -> SourceSpan {
        (self.0.offset, self.1.offset.saturating_sub(self.0.offset)).into()
    }
}

impl chumsky::zero_copy::span::Span for Span {
    type Context = ();
    type Offset = usize;
}

// #[allow(missing_debug_implementations)]
// mod sealed {
//     pub struct OffsetTracker {
//         pub(crate) offset: usize,
//     }

//     #[cfg(feature = "line-numbers")]
//     pub struct LineTracker {
//         pub(crate) offset: usize,
//         pub(crate) caret_return: bool,
//         pub(crate) line: usize,
//         pub(crate) column: usize,
//     }

// }

// impl traits::sealed::SpanTracker for sealed::OffsetTracker {
//     type Span = Span;
//     fn next_span(&mut self, c: char) -> Span {
//         let start = self.offset;
//         self.offset += c.len_utf8();
//         Span(start, self.offset)
//     }
// }

// impl traits::sealed::Sealed for Span {
//     type Tracker = sealed::OffsetTracker;
//     fn at_start(&self, chars: usize) -> Self {
//         Span(self.0, self.0 + chars)
//     }

//     fn at_end(&self) -> Self {
//         Span(self.1, self.1)
//     }

//     fn before_start(&self, chars: usize) -> Self {
//         Span(self.0.saturating_sub(chars), self.0)
//     }

//     fn len(&self) -> usize {
//         self.1.saturating_sub(self.0)
//     }

//     // fn stream(text: &str) -> traits::sealed::Stream<'_, Self, Self::Tracker>
//     //     where Self: chumsky::Span
//     // {
//     //     chumsky::Stream::from_iter(
//     //         Span(text.len(), text.len()),
//     //         traits::sealed::Map(text.chars(),
//     //                             sealed::OffsetTracker { offset: 0 }),
//     //     )
//     // }
// }

impl chumsky::zero_copy::span::Span for LineSpan {
    type Context = ();
    type Offset = LinePos;
    // fn new(_context: (), range: std::ops::Range<LinePos>) -> Self {
    //     LineSpan(range.start, range.end)
    // }
}

// #[cfg(feature = "line-numbers")]
// impl traits::sealed::SpanTracker for sealed::LineTracker {
//     type Span = LineSpan;
//     fn next_span(&mut self, c: char) -> LineSpan {
//         let offset = self.offset;
//         let line = self.line;
//         let column = self.column;
//         self.offset += c.len_utf8();
//         match c {
//             '\n' if self.caret_return => {}
//             '\r'|'\n'|'\x0C'|'\u{0085}'|'\u{2028}'|'\u{2029}' => {
//                 self.line += 1;
//                 self.column = 0;
//             }
//             '\t' => self.column += 8,
//             c => {
//                 self.column += unicode_width::UnicodeWidthChar::width(c)
//                     .unwrap_or(0);  // treat control chars as zero-length
//             }
//         }
//         self.caret_return = c == '\r';
//         LineSpan(
//             LinePos {
//                 line,
//                 column,
//                 offset,
//             },
//             LinePos {
//                 line: self.line,
//                 column: self.column,
//                 offset: self.offset,
//             },
//         )
//     }
// }

// #[cfg(feature = "line-numbers")]
// impl traits::sealed::Sealed for LineSpan {
//     type Tracker = sealed::LineTracker;
//     /// Note assuming ascii, single-width, non-newline chars here
//     fn at_start(&self, chars: usize) -> Self {
//         LineSpan(self.0, LinePos {
//             offset: self.0.offset + chars,
//             column: self.0.column + chars,
//             .. self.0
//         })
//     }

//     fn at_end(&self) -> Self {
//         LineSpan(self.1, self.1)
//     }

//     /// Note assuming ascii, single-width, non-newline chars here
//     fn before_start(&self, chars: usize) -> Self {
//         LineSpan(LinePos {
//             offset: self.0.offset.saturating_sub(chars),
//             column: self.0.column.saturating_sub(chars),
//             .. self.0
//         }, self.0)
//     }

//     fn len(&self) -> usize {
//         self.1.offset.saturating_sub(self.0.offset)
//     }

//     // fn stream(text: &str) -> traits::sealed::Stream<'_, Self, Self::Tracker>
//     //     where Self: chumsky::Span
//     // {
//     //     let mut caret_return = false;
//     //     let mut line = 0;
//     //     let mut last_line = text;
//     //     let mut iter = text.chars();
//     //     while let Some(c) = iter.next() {
//     //         match c {
//     //             '\n' if caret_return => {}
//     //             '\r'|'\n'|'\x0C'|'\u{0085}'|'\u{2028}'|'\u{2029}' => {
//     //                 line += 1;
//     //                 last_line = iter.as_str();
//     //             }
//     //             _ => {}
//     //         }
//     //         caret_return = c == '\r';
//     //     }
//     //     let column = unicode_width::UnicodeWidthStr::width(last_line);
//     //     let eoi = LinePos {
//     //         line,
//     //         column,
//     //         offset: text.len(),
//     //     };
//     //     chumsky::Stream::from_iter(
//     //         LineSpan(eoi, eoi),
//     //         traits::sealed::Map(
//     //             text.chars(),
//     //             sealed::LineTracker {
//     //                 caret_return: false,
//     //                 offset: 0,
//     //                 line: 0,
//     //                 column: 0,
//     //             },
//     //         ),
//     //     )
//     // }
// }
