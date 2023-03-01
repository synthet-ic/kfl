use core::{
    ops::{
        BitOr, BitAnd, Range, RangeFrom, RangeTo, Mul, RangeBounds, Bound, Try,
        ControlFlow, FromResidual, RangeFull
    },
    str::pattern::{Pattern, Searcher, SearchStep}
};
use regex::{Regex, Matches};
use regex_syntax::{
    Parser,
    hir::{
        Hir, HirKind, Repetition, RepetitionKind, RepetitionRange, Class,
        ClassUnicode, ClassUnicodeRange,
        print::Printer
    }
};

#[derive(Clone, Debug, PartialEq)]
pub struct Pat(Hir);

impl Pat {
    pub fn new(re: &str) -> Self {
        let hir = Parser::new().parse(re).unwrap();
        Self(hir)
    }

    pub fn empty() -> Self {
        Self(Hir::empty())
    }

    pub fn or<P: Into<Self>>(self, pat: P) -> Self {
        let hir = Hir::alternation(vec![self.0, pat.into().0]);
        Self(hir)
    }

    pub fn and<P: Into<Self>>(self, pat: P) -> Self {
        let hir = Hir::concat(vec![self.0, pat.into().0]);
        Self(hir)
    }

    pub fn range(range: Range<char>) -> Self {
        let range = ClassUnicodeRange::new(range.start, range.end);
        let class = ClassUnicode::new([range]);
        Self(Hir::class(Class::Unicode(class)))
    }

    pub fn mul<R: RangeBounds<u32>>(self, range: R) -> Self {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Unbounded, Bound::Unbounded) => {
                self * ..
            },
            (Bound::Unbounded, Bound::Excluded(end)) => {
                self * (..*end)
            },
            (Bound::Included(start), Bound::Unbounded) => {
                self * (*start..)
            },
            (Bound::Included(start), Bound::Excluded(end)) => {
                self * (*start..*end)
            },
            _ => {
                panic!("In regex-ext, m..n is interpreted as m..=n.");
            }
        }
    }
}

impl From<char> for Pat {
    fn from(value: char) -> Self {
        Self::new(&value.to_string())
    }
}   

impl From<&str> for Pat {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&Hir> for Pat {
    fn from(value: &Hir) -> Self {
        Self(value.clone())
    }
}

impl From<Range<char>> for Pat {
    fn from(value: Range<char>) -> Self {
        Self::range(value)
    }
}

impl<T: Into<Pat>> From<[T; 1]> for Pat {
    fn from(value: [T; 1]) -> Self {
        value.into_iter().nth(0).unwrap().into() * ..
    }
}

impl BitAnd<char> for Pat {
    type Output = Pat;

    fn bitand(self, rhs: char) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAnd<&str> for Pat {
    type Output = Pat;

    fn bitand(self, rhs: &str) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAnd<Pat> for &str {
    type Output = Pat;

    fn bitand(self, rhs: Pat) -> Self::Output {
        rhs.clone().and(self)
    }
}

impl BitAnd<Pat> for Pat {
    type Output = Pat;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

// impl BitAnd<Range<u8>> for Pat {
//     type Output = Self;

//     fn bitand(self, rhs: Range<u8>) -> Self::Output {
//         self.and(rhs)
//     }
// }

impl BitAnd<Range<char>> for Pat {
    type Output = Pat;

    fn bitand(self, rhs: Range<char>) -> Self::Output {
        self.and(rhs)
    }
}

impl<T: Into<Pat>> BitAnd<[T; 1]> for Pat {
    type Output = Pat;

    fn bitand(self, rhs: [T; 1]) -> Self::Output {
        self.and(Pat::from(rhs) * ..)
    }
}

impl BitOr<char> for Pat {
    type Output = Pat;
    
    fn bitor(self, rhs: char) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOr<&str> for Pat {
    type Output = Pat;

    fn bitor(self, rhs: &str) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOr<Pat> for &str {
    type Output = Pat;

    fn bitor(self, rhs: Pat) -> Self::Output {
        rhs.or(self)
    }
}

impl BitOr<Pat> for Pat {
    type Output = Pat;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOr<Range<char>> for Pat {
    type Output = Pat;

    fn bitor(self, rhs: Range<char>) -> Self::Output {
        self.or(rhs)
    }
}

impl<T: Into<Pat>> BitOr<[T; 1]> for Pat {
    type Output = Pat;

    fn bitor(self, rhs: [T; 1]) -> Self::Output {
        self.or(Pat::from(rhs) * ..)
    }
}

impl Mul<u32> for Pat {
    type Output = Pat;

    fn mul(self, rhs: u32) -> Self::Output {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Exactly(rhs)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<RangeFull> for Pat {
    type Output = Pat;

    fn mul(self, _: RangeFull) -> Self::Output {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::AtLeast(0)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<Range<u32>> for Pat {
    type Output = Pat;

    fn mul(self, rhs: Range<u32>) -> Self::Output {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(rhs.start, rhs.end)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<RangeFrom<u32>> for Pat {
    type Output = Pat;

    fn mul(self, rhs: RangeFrom<u32>) -> Self::Output {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::AtLeast(rhs.start)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Mul<RangeTo<u32>> for Pat {
    type Output = Pat;

    fn mul(self, rhs: RangeTo<u32>) -> Self::Output {
        let rep = Repetition {
            kind: RepetitionKind::Range(RepetitionRange::Bounded(0, rhs.end)),
            greedy: true,
            hir: box self.0
        };
        Self(Hir::repetition(rep))
    }
}

impl Try for Pat {
    type Output = Pat;
    type Residual = Pat;

    fn from_output(output: Self::Output) -> Self {
        output * (0..1)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0.kind() {
            HirKind::Repetition(rep) => {
                let rep = Repetition {
                    kind: rep.kind.clone(),
                    greedy: false,
                    hir: rep.hir.clone()
                };
                ControlFlow::Continue(Self(Hir::repetition(rep)))
            },
            _ => ControlFlow::Continue(self * (0..1))
        }
    }
}

impl FromResidual for Pat {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

// impl Deref for Pat {
//     type Target = *mut Self;

//     fn deref(&self) -> &Self::Target {
//         &(*self * ..)
//     }
// }

impl Into<Regex> for Pat {
    fn into(self) -> Regex {
        let mut pat = String::new();
        Printer::new().print(&self.0, &mut pat).unwrap();
        Regex::new(&pat).unwrap()
    }
}

// #[derive(Debug)]
// pub struct RegexSearcher<'r, 't> {
//     haystack: &'t str,
//     it: Matches<'r, 't>,
//     last_step_end: usize,
//     next_match: Option<(usize, usize)>,
// }

// impl<'r, 't> Pattern<'t> for &'r Pat {
//     type Searcher = RegexSearcher<'r, 't>;

//     fn into_searcher(self, haystack: &'t str) -> RegexSearcher<'r, 't> {
//         RegexSearcher {
//             haystack,
//             it: self.find_iter(haystack),
//             last_step_end: 0,
//             next_match: None,
//         }
//     }
// }

// unsafe impl<'r, 't> Searcher<'t> for RegexSearcher<'r, 't> {
//     #[inline]
//     fn haystack(&self) -> &'t str {
//         self.haystack
//     }

//     #[inline]
//     fn next(&mut self) -> SearchStep {
//         if let Some((s, e)) = self.next_match {
//             self.next_match = None;
//             self.last_step_end = e;
//             return SearchStep::Match(s, e);
//         }
//         match self.it.next() {
//             None => {
//                 if self.last_step_end < self.haystack().len() {
//                     let last = self.last_step_end;
//                     self.last_step_end = self.haystack().len();
//                     SearchStep::Reject(last, self.haystack().len())
//                 } else {
//                     SearchStep::Done
//                 }
//             }
//             Some(m) => {
//                 let (s, e) = (m.start(), m.end());
//                 if s == self.last_step_end {
//                     self.last_step_end = e;
//                     SearchStep::Match(s, e)
//                 } else {
//                     self.next_match = Some((s, e));
//                     let last = self.last_step_end;
//                     self.last_step_end = s;
//                     SearchStep::Reject(last, s)
//                 }
//             }
//         }
//     }
// }
