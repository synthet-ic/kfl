#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassUnicodeKind {
    /// A one letter abbreviated class, e.g., `\pN`.
    OneLetter(char),
    /// A binary property, general category or script. The string may be
    /// empty.
    Named(String),
    /// A property name and an associated value.
    NamedValue {
        /// The type of Unicode op used to associate `name` with `value`.
        op: ClassUnicodeOpKind,
        /// The property name (which may be empty).
        name: String,
        /// The property value (which may be empty).
        value: String,
    },
}

/// The type of op used in a Unicode character class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassUnicodeOpKind {
    /// A property set to a specific value, e.g., `\p{scx=Katakana}`.
    Equal,
    /// A property set to a specific value using a colon, e.g.,
    /// `\p{scx:Katakana}`.
    Colon,
    /// A property that isn't a particular value, e.g., `\p{scx!=Katakana}`.
    NotEqual,
}
