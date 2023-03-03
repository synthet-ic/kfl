/// The kind of a single literal expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiteralKind {
    /// The literal is written verbatim, e.g., `a` or `☃`.
    Verbatim,
    /// The literal is written as an escape because it is punctuation, e.g.,
    /// `\*` or `\[`.
    Punctuation,
    /// The literal is written as an octal escape, e.g., `\141`.
    Octal,
    /// The literal is written as a hex code with a fixed number of digits
    /// depending on the type of the escape, e.g., `\x61` or or `\u0061` or
    /// `\U00000061`.
    HexFixed(HexLiteralKind),
    /// The literal is written as a hex code with a bracketed number of
    /// digits. The only restriction is that the bracketed hex code must refer
    /// to a valid Unicode scalar value.
    HexBrace(HexLiteralKind),
    /// The literal is written as a specially recognized escape, e.g., `\f`
    /// or `\n`.
    Special(SpecialLiteralKind),
}

/// The type of a special literal.
///
/// A special literal is a special escape sequence recognized by the regex
/// parser, e.g., `\f` or `\n`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpecialLiteralKind {
    /// Bell, spelled `\a` (`\x07`).
    Bell,
    /// Form feed, spelled `\f` (`\x0C`).
    FormFeed,
    /// Tab, spelled `\t` (`\x09`).
    Tab,
    /// Line feed, spelled `\n` (`\x0A`).
    LineFeed,
    /// Carriage return, spelled `\r` (`\x0D`).
    CarriageReturn,
    /// Vertical tab, spelled `\v` (`\x0B`).
    VerticalTab,
    /// Space, spelled `\ ` (`\x20`). Note that this can only appear when
    /// parsing in verbose mode.
    Space,
}

/// The type of a Unicode hex literal.
///
/// Note that all variants behave the same when used with brackets. They only
/// differ when used without brackets in the number of hex digits that must
/// follow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HexLiteralKind {
    /// A `\x` prefix. When used without brackets, this form is limited to
    /// two digits.
    X,
    /// A `\u` prefix. When used without brackets, this form is limited to
    /// four digits.
    UnicodeShort,
    /// A `\U` prefix. When used without brackets, this form is limited to
    /// eight digits.
    UnicodeLong,
}

impl HexLiteralKind {
    /// The number of digits that must be used with this literal form when
    /// used without brackets. When used with brackets, there is no
    /// restriction on the number of digits.
    pub fn digits(&self) -> u32 {
        match *self {
            HexLiteralKind::X => 2,
            HexLiteralKind::UnicodeShort => 4,
            HexLiteralKind::UnicodeLong => 8,
        }
    }
}

impl LiteralKind {
    /// If this literal was written as a `\x` hex escape, then this returns
    /// the corresponding byte value. Otherwise, this returns `None`.
    pub fn byte(&self, c: char) -> Option<u8> {
        let short_hex = LiteralKind::HexFixed(HexLiteralKind::X);
        if c as u32 <= 255 && self == short_hex {
            Some(c as u8)
        } else {
            None
        }
    }
}

fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result {
    use crate::ast::LiteralKind::*;

    match ast.kind {
        Verbatim => self.wtr.write_char(ast.c),
        Punctuation => write!(self.wtr, r"\{}", ast.c),
        Octal => write!(self.wtr, r"\{:o}", ast.c as u32),
        HexFixed(ast::HexLiteralKind::X) => {
            write!(self.wtr, r"\x{:02X}", ast.c as u32)
        }
        HexFixed(ast::HexLiteralKind::UnicodeShort) => {
            write!(self.wtr, r"\u{:04X}", ast.c as u32)
        }
        HexFixed(ast::HexLiteralKind::UnicodeLong) => {
            write!(self.wtr, r"\U{:08X}", ast.c as u32)
        }
        HexBrace(ast::HexLiteralKind::X) => {
            write!(self.wtr, r"\x{{{:X}}}", ast.c as u32)
        }
        HexBrace(ast::HexLiteralKind::UnicodeShort) => {
            write!(self.wtr, r"\u{{{:X}}}", ast.c as u32)
        }
        HexBrace(ast::HexLiteralKind::UnicodeLong) => {
            write!(self.wtr, r"\U{{{:X}}}", ast.c as u32)
        }
        Special(ast::SpecialLiteralKind::Bell) => {
            self.wtr.write_str(r"\a")
        }
        Special(ast::SpecialLiteralKind::FormFeed) => {
            self.wtr.write_str(r"\f")
        }
        Special(ast::SpecialLiteralKind::Tab) => self.wtr.write_str(r"\t"),
        Special(ast::SpecialLiteralKind::LineFeed) => {
            self.wtr.write_str(r"\n")
        }
        Special(ast::SpecialLiteralKind::CarriageReturn) => {
            self.wtr.write_str(r"\r")
        }
        Special(ast::SpecialLiteralKind::VerticalTab) => {
            self.wtr.write_str(r"\v")
        }
        Special(ast::SpecialLiteralKind::Space) => {
            self.wtr.write_str(r"\ ")
        }
    }
}
