use core::{
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
