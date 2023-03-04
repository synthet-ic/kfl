**TODO**

- Can `ignore` be represented by Map?
- Can flags, infos and assertions be represented by Map?
- Derivatives and lookaheads

**`Map`**

- Capturing/grouping

**`Zero`**

- `Zero` acts as both an additive and multiplicative unit?

**`Div`** (quotients)

- Case insensitivity

**Comparisons**

| name | regex | op | kfl (exp.) |
| - | - | - | - |
| concat | `ab` | `a & b` | `and { a b }` |
| alternation | `a\|b` | `a \| b` | `or { a b }` |
| kleen star | `a*` | `[a]` | `star { a }`
| optional | `a?` | `a?` | `opt { a }` |
| repetition | `a{n,m}` | `a * (n..m)` | `mul n m { a }` |
| class | `[a-z]` | `'a'..'z'` | `seq a z` |
| negation | `[^a-z]` | !`'a'..'z'` | `not { seq { a z } }` |

**Targets**

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssertionKind {
    /// `^`,  `(?m:^)`
    StartLine,
    /// `$`, `(?m:$)`
    EndLine,
    /// `\A`
    StartText,
    /// `\z`
    EndText,
    /// `\b`, `(?-u:\b)`
    WordBoundary,
    /// `\B`, `(?-u:\B)`
    NotWordBoundary,
}
```

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Flag {
    /// `i`
    CaseInsensitive,
    /// `m`
    MultiLine,
    /// `s`
    DotMatchesNewLine,
    /// `U`
    SwapGreed,
    /// `u`
    Unicode,
    /// `x`
    IgnoreWhitespace,
}
```

```rust
/// An anchor assertion. An anchor assertion match always has zero length.
/// The high-level intermediate representation for an anchor assertion.
///
/// A matching anchor assertion is always zero-length.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Anchor {
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following a `\n` character.
    StartLine,
    /// Match the end of a line or the end of text. Specifically,
    /// this matches at the end position of the input, or at the position
    /// immediately preceding a `\n` character.
    EndLine,
    /// Match the beginning of text. Specifically, this matches at the starting
    /// position of the input.
    StartText,
    /// Match the end of text. Specifically, this matches at the ending
    /// position of the input.
    EndText,
}

/// A word boundary assertion, which may or may not be Unicode aware. A
/// word boundary assertion match always has zero length.
/// The high-level intermediate representation for a word-boundary assertion.
///
/// A matching word boundary assertion is always zero-length.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WordBoundary {
    /// Match a Unicode-aware word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    Unicode,
    /// Match a Unicode-aware negation of a word boundary.
    UnicodeNegate,
    /// Match an ASCII-only word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    Ascii,
    /// Match an ASCII-only negation of a word boundary.
    AsciiNegate,
}
```

- Recouse non-greedy pattern to _
