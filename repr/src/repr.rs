use alloc::{
    boxed::Box,
    vec::Vec
};
use core::{
    cmp::{max, min},
    fmt::{Debug, Display},
    marker::Destruct,
    slice::Iter
};

// TODO(rnarkk) Seq (class) as `or` for char, &str as `and` for char?
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Repr<I: ~const Integral> {
    Zero,  // TODO(rnarkk) let it hold word boundary?
    One(I),  // TODO(rnarkk)  Seq(I, I)
    Seq(Seq<I>),  // TODO(rnarkk)
    Not(Box<Repr<I>>),
    Or(Box<Repr<I>>, Box<Repr<I>>),
    And(Box<Repr<I>>, Box<Repr<I>>),
    Xor(Box<Repr<I>>, Box<Repr<I>>),
    Add(Box<Repr<I>>, Box<Repr<I>>),  // RegexSet
    Sub(Box<Repr<I>>, Seq<I>),
    Mul(Box<Repr<I>>, Range),
    // Map(Box<Repr<I>>, Fn(Box<Repr<I>>), Fn(Box<Repr<I>>))
}

impl<I: ~const Integral> Repr<I> {
    pub const fn empty() -> Self {
        Self::Zero
    }
    
    pub const fn not(self) -> Self {
        Self::Not(box self)
    }
    
    pub const fn and(self, other: Self) -> Self {
        Self::And(box self, box other)
    }
    
    pub const fn or(self, other: Self) -> Self {
        Self::Or(box self, box other)
    }
    
    pub const fn xor(self, other: Self) -> Self {
        Self::Xor(box self, box other)
    }
    
    pub const fn add(self, other: Self) -> Self {
        Self::Add(box self, box other)
    }
    
    pub const fn sub(self, seq: Seq<I>) -> Self {
        Self::Sub(box self, seq)
    }
    
    pub const fn mul(self, range: Range) -> Self {
        Self::Mul(box self, range)
    }
}

impl const Repr<char> {
    /// `.` expression that matches any character except for `\n`. To build an
    /// expression that matches any character, including `\n`, use the `any`
    /// method.
    pub const fn dot() -> Self {
        Self::Or(Self::Seq(Seq('\0', '\x09')), Self::Seq(Seq('\x0B', '\u{10FFFF}')))
    }

    /// `(?s).` expression that matches any character, including `\n`. To build an
    /// expression that matches any character except for `\n`, then use the
    /// `dot` method.
    pub const fn any() -> Self {
        Self::Seq(Seq('\0', '\u{10FFFF}'))
    }
}

impl const Repr<u8> {
    pub const fn dot() -> Self {
        Self::Or(Self::Seq(Seq(b'\0', b'\x09')), Self::Seq(Seq(b'\x0B', b'\xFF')))
    }

    pub const fn any() -> Self {
        Self::Seq(Seq(b'\0', b'\xFF'))
    }
}

impl<const N: usize, I: ~const Integral> const Into<[I; N]> for Repr<I> {
    fn into(self) -> [I; N] {
        match self {
            Repr::Zero => [],
            Repr::Not(repr) => {
                
            }
            Repr::Xor(lhs, rhs) => lhs.clone().or(rhs).sub(lhs.and(rhs)),
            _ => unimplemented!()
        }
    }
}

impl<I: ~const Integral> const IntoIterator for Repr<I> {
    type Item = I;
    type IntoIter: Iter<'a, I>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = Vec::new();
        match self {
            _ => unimplemented!()
        }
    }
}

#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, Default, PartialEq, PartialOrd, Ord)]
pub struct Seq<I: ~const Integral>(pub I, pub I);

impl<I: ~const Integral> const Seq<I> {
    pub const fn new(from: I, to: I) -> Self {
        if from <= to {
            Seq(from, to)
        } else {
            Seq(to, from)
        }
    }
    
    /// Intersect this Seq with the given Seq and return the result.
    ///
    /// If the intersection is empty, then this returns `None`.
    pub const fn and(&self, other: &Self) -> Option<Self> {
        let from = max(self.0, other.0);
        let to = min(self.1, other.1);
        if from <= to {
            Some(Self::new(from, to))
        } else {
            None
        }
    }
    
    /// Union the given overlapping Seq into this Seq.
    ///
    /// If the two ranges aren't contiguous, then this returns `None`.
    pub const fn or(&self, other: &Self) -> Option<Self> {
        if !self.is_contiguous(other) {
            return None;
        }
        let from = max(self.0, other.0);
        let to = min(self.1, other.1);
        Some(Self::new(from, to))
    }
    
    /// Compute the symmetric difference the given Seq from this Seq. This
    /// returns the union of the two ranges minus its intersection.
    pub const fn xor(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        let or = match self.or(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(or) => or,
        };
        let and = match self.and(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(and) => and,
        };
        or.sub(&and)
    }
    
    /// Subtract the given Seq from this Seq and return the resulting
    /// Seqs.
    ///
    /// If subtraction would result in an empty Seq, then no Seqs are
    /// returned.
    fn sub(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        if self.le(other) {
            return (None, None);
        }
        if self.is_intersection_empty(other) {
            return (Some(self.clone()), None);
        }
        let add_lower = other.0 > self.0;
        let add_upper = other.1 < self.1;
        // We know this because !self.le(other) and the ranges have
        // a non-empty intersection.
        assert!(add_lower || add_upper);
        let mut ret = (None, None);
        if add_lower {
            let upper = other.0.decrement();
            ret.0 = Some(Self::create(self.0, upper));
        }
        if add_upper {
            let lower = other.1.increment();
            let range = Self::create(lower, self.1);
            if ret.0.is_none() {
                ret.0 = Some(range);
            } else {
                ret.1 = Some(range);
            }
        }
        ret
    }
    
    /// If Seqs are either overlapping or adjacent.
    pub const fn is_contiguous(&self, other: &Self) -> bool {
        let lower1 = self.0.as_u32();
        let upper1 = self.1.as_u32();
        let lower2 = other.0.as_u32();
        let upper2 = other.1.as_u32();
        max(lower1, lower2) <= min(upper1, upper2).saturating_add(1)
    }

    /// If the intersection of this range and the
    /// other range is empty.
    pub const fn is_intersection_empty(&self, other: &Self) -> bool {
        let (lower1, upper1) = (self.0, self.1);
        let (lower2, upper2) = (other.0, other.1);
        max(lower1, lower2) > min(upper1, upper2)
    }

    /// Returns true if and only if this range is a subset of the other range.
    pub const fn le(&self, other: &Self) -> bool {
        let (lower1, upper1) = (self.0, self.1);
        let (lower2, upper2) = (other.0, other.1);
        (lower2 <= lower1 && lower1 <= upper2)
        && (lower2 <= upper1 && upper1 <= upper2)
    }
    
//     /// Apply Unicode simple case folding to this character class, in place.
//     /// The character class will be expanded to include all simple case folded
//     /// character variants.
//     ///
//     /// If this is a byte oriented character class, then this will be limited
//     /// to the ASCII ranges `A-Z` and `a-z`.
//     pub fn case_fold_simple(&mut self);
}

impl const Seq<char> {
    /// Expand this character class such that it contains all case folded
    /// characters, according to Unicode's "simple" mapping. For example, if
    /// this class consists of the range `a-z`, then applying case folding will
    /// result in the class containing both the ranges `a-z` and `A-Z`.
    ///
    /// # Panics
    ///
    /// This routine panics when the case mapping data necessary for this
    /// routine to complete is unavailable. This occurs when the `unicode-case`
    /// feature is not enabled.
    ///
    /// Callers should prefer using `try_case_fold_simple` instead, which will
    /// return an error instead of panicking.
    /// =======================================================================
    /// Apply simple case folding to this Unicode scalar value range.
    ///
    /// Additional ranges are appended to the given vector. Canonical ordering
    /// is *not* maintained in the given vector.
    fn case_fold_simple(
        &self,
        ranges: &mut Vec<ClassUnicodeRange>,
    ) -> Result<(), unicode::CaseFoldError> {
        if !unicode::contains_simple_case_mapping(self.start, self.end)? {
            return Ok(());
        }
        let start = self.start as u32;
        let end = (self.end as u32).saturating_add(1);
        let mut next_simple_cp = None;
        for cp in (start..end).filter_map(char::from_u32) {
            if next_simple_cp.map_or(false, |next| cp < next) {
                continue;
            }
            let it = match unicode::simple_fold(cp)? {
                Ok(it) => it,
                Err(next) => {
                    next_simple_cp = next;
                    continue;
                }
            };
            for cp_folded in it {
                ranges.push(ClassUnicodeRange::new(cp_folded, cp_folded));
            }
        }
        Ok(())
    }

    /// Expand this character class such that it contains all case folded
    /// characters, according to Unicode's "simple" mapping. For example, if
    /// this class consists of the range `a-z`, then applying case folding will
    /// result in the class containing both the ranges `a-z` and `A-Z`.
    ///
    /// # Error
    ///
    /// This routine returns an error when the case mapping data necessary
    /// for this routine to complete is unavailable. This occurs when the
    /// `unicode-case` feature is not enabled.
    pub fn try_case_fold_simple(
        &mut self,
    ) -> result::Result<(), CaseFoldError> {
        self.0.case_fold_simple()
    }
    
    /// Returns true if and only if this character class will only ever match
    /// valid UTF-8.
    ///
    /// A character class can match invalid UTF-8 only when the following
    /// conditions are met:
    ///
    /// 1. The translator was configured to permit generating an expression
    ///    that can match invalid UTF-8. (By default, this is disabled.)
    /// 2. Unicode mode (via the `u` flag) was disabled either in the concrete
    ///    syntax or in the parser builder. By default, Unicode mode is
    ///    enabled.
    pub const fn is_always_utf8(&self) -> bool {
        true
    }

    /// Returns true if and only if this character class will either match
    /// nothing or only ASCII bytes. Stated differently, this returns false
    /// if and only if this class contains a non-ASCII codepoint.
    pub fn is_all_ascii(&self) -> bool {
        self.0.intervals().last().map_or(true, |r| r.end <= '\x7F')
    }
}

impl const Debug for Seq<char> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let start = if !self.0.is_whitespace() && !self.0.is_control()
        {
            self.0.to_string()
        } else {
            format!("0x{:X}", self.0 as u32)
        };
        let end = if !self.1.is_whitespace() && !self.1.is_control() {
            self.1.to_string()
        } else {
            format!("0x{:X}", self.1 as u32)
        };
        f.debug_struct("ClassUnicodeRange")
            .field("start", &start)
            .field("end", &end)
            .finish()
    }
}

impl const Seq<u8> {
    /// Expand this character class such that it contains all case folded
    /// characters. For example, if this class consists of the range `a-z`,
    /// then applying case folding will result in the class containing both the
    /// ranges `a-z` and `A-Z`.
    ///
    /// Note that this only applies ASCII case folding, which is limited to the
    /// characters `a-z` and `A-Z`.
    /// =======================================================================
    /// Apply simple case folding to this byte range. Only ASCII case mappings
    /// (for a-z) are applied.
    ///
    /// Additional ranges are appended to the given vector. Canonical ordering
    /// is *not* maintained in the given vector.
    fn case_fold_simple(
        &self,
        ranges: &mut Vec<ClassBytesRange>,
    ) -> Result<(), unicode::CaseFoldError> {
        if !ClassBytesRange::new(b'a', b'z').is_intersection_empty(self) {
            let lower = max(self.start, b'a');
            let upper = min(self.end, b'z');
            ranges.push(ClassBytesRange::new(lower - 32, upper - 32));
        }
        if !ClassBytesRange::new(b'A', b'Z').is_intersection_empty(self) {
            let lower = max(self.start, b'A');
            let upper = min(self.end, b'Z');
            ranges.push(ClassBytesRange::new(lower + 32, upper + 32));
        }
        Ok(())
    }
    
    pub const fn is_always_utf8(&self) -> bool {
        self.is_all_ascii()
    }

    /// If this character class will either match
    /// nothing or only ASCII bytes. Or this class contains a non-ASCII byte.
    pub const fn is_all_ascii(&self) -> bool {
        self.1 <= 0x7F
    }
}

impl Debug for Seq<u8> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = f.debug_struct("ClassBytesRange");
        if self.0 <= 0x7F {
            debug.field("start", &(self.0 as char));
        } else {
            debug.field("start", &self.0);
        }
        if self.1 <= 0x7F {
            debug.field("end", &(self.1 as char));
        } else {
            debug.field("end", &self.1);
        }
        debug.finish()
    }
}

/// A single character, where a character is either
/// defined by a Unicode scalar value or an arbitrary byte. Unicode characters
/// are preferred whenever possible. In particular, a `Byte` variant is only
/// ever produced when it could match invalid UTF-8.
/// ==========================================================================
/// Type of characters. A character is either
/// defined by a Unicode scalar value or a byte. Unicode characters are used
/// by default, while bytes are used when Unicode mode (via the `u` flag) is
/// disabled.
///
/// A character class, regardless of its character type, is represented by a
/// sequence of non-overlapping non-adjacent ranges of characters.
///
/// Note that unlike [`Literal`](enum.Literal.html), a `Bytes` variant may
/// be produced even when it exclusively matches valid UTF-8. This is because
/// a `Bytes` variant represents an intention by the author of the regular
/// expression to disable Unicode mode, which in turn impacts the semantics of
/// case insensitive matching. For example, `(?i)k` and `(?i-u)k` will not
/// match the same set of strings.
#[const_trait]
pub trait Integral:
    Copy + ~const Clone + Debug + Eq + ~const PartialEq + ~const  PartialOrd
    + ~const Ord
    + ~const Destruct
{
    const MIN: Self;
    const MAX: Self;
    fn as_u32(self) -> u32;
    fn succ(self) -> Self;
    fn pred(self) -> Self;
}

/// Unicode scalar values
impl const Integral for char {
    const MIN: Self = '\x00';
    const MAX: Self = '\u{10FFFF}';
    fn as_u32(self) -> u32 {
        self as u32
    }

    fn succ(self) -> Self {
        match self {
            '\u{D7FF}' => '\u{E000}',
            c => char::from_u32((c as u32).checked_add(1).unwrap()).unwrap(),
        }
    }

    fn pred(self) -> Self {
        match self {
            '\u{E000}' => '\u{D7FF}',
            c => char::from_u32((c as u32).checked_sub(1).unwrap()).unwrap(),
        }
    }
}

/// Arbitrary bytes
impl const Integral for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
    fn as_u32(self) -> u32 {
        self as u32
    }
    fn succ(self) -> Self {
        self.checked_add(1).unwrap()
    }
    fn pred(self) -> Self {
        self.checked_sub(1).unwrap()
    }
}

// 24bit
#[derive_const(Clone, PartialEq, PartialOrd, Ord)]
#[derive(Copy, Debug, Eq)]
pub enum Range {
    Empty,
    From(usize),
    To(usize),
    Full(usize, usize),
}

impl Range {
    /// Returns true if and only if this repetition operator makes it possible
    /// to match the empty string.
    ///
    /// Note that this is not defined inductively. For example, while `a*`
    /// will report `true`, `()+` will not, even though `()` matches the empty
    /// string and one or more occurrences of something that matches the empty
    /// string will always match the empty string. In order to get the
    /// inductive definition, see the corresponding method on
    /// [`Hir`](struct.Hir.html).
    pub const fn is_match_empty(&self) -> bool {
        match self {
            Range::Empty => true,
            Range::To(_) => true,
            Range::From(n) => n == 0,
            Range::Full(n, _) => n == 0,
        }
    }
}
