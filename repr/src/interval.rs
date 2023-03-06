//! TODO(rnarkk) Refactor and relocate them in crate::repr
//! <https://en.wikipedia.org/wiki/Interval_arithmetic>
//! <https://en.wikipedia.org/wiki/Boundary_(topology)>
//! <https://en.wikipedia.org/wiki/Partition_of_a_set>
//! <https://en.wikipedia.org/wiki/Sequence>

// This module contains an *internal* implementation of interval sets.
//
// The primary invariant that interval sets guards is canonical ordering. That
// is, every interval set contains an ordered sequence of intervals where
// no two intervals are overlapping or adjacent. While this invariant is
// occasionally broken within the implementation, it should be impossible for
// callers to observe it.
//
// Since case folding (as implemented below) breaks that invariant, we roll
// that into this API even though it is a little out of place in an otherwise
// generic interval set. (Hence the reason why the `unicode` module is imported
// here.)
//
// Some of the implementation complexity here is a result of me wanting to
// preserve the sequential representation without using additional memory.
// In many cases, we do use linear extra memory, but it is at most 2x and it
// is amortized. If we relaxed the memory requirements, this implementation
// could become much simpler. The extra memory is honestly probably OK, but
// character classes (especially of the Unicode variety) can become quite
// large, and it would be nice to keep regex compilation snappy even in debug
// builds. (In the past, I have been careless with this area of code and it has
// caused slow regex compilations in debug mode, so this isn't entirely
// unwarranted.)
//
// Tests on this are relegated to the public API of HIR in src/hir.rs.
// Tests for interval sets are written in src/hir.rs against the public API.

// impl<I: Interval> Repr<I> {
//     /// Create a new set from a sequence of intervals.
//     ///
//     /// The given ranges do not need to be in any specific order, and ranges
//     /// may overlap.
//     pub const fn new<const N: usize>(ranges: [I; N]) -> Self {
//         ranges.into_iter().reduce(|acc, e| Self::Or(acc, e))
//         // set.canonicalize();
//         // set
//     }

//     /// Expand this interval set such that it contains all case folded
//     /// characters. For example, if this class consists of the range `a-z`,
//     /// then applying case folding will result in the class containing both the
//     /// ranges `a-z` and `A-Z`.
//     ///
//     /// This returns an error if the necessary case mapping data is not
//     /// available.
//     pub fn case_fold_simple(&mut self) -> Result<(), ()> {
//         let len = self.0.len();
//         for i in 0..len {
//             let range = self.0[i];
//             if let Err(err) = range.case_fold_simple(&mut self.0) {
//                 self.canonicalize();
//                 return Err(err);
//             }
//         }
//         self.canonicalize();
//         Ok(())
//     }

//     /// Intersect this set with the given set, in place.
//     pub fn and(self, other: &Self) -> Self {
//         if self.0.is_empty() {
//             return;
//         }
//         if other.0.is_empty() {
//             self.0.clear();
//             return;
//         }

//         // There should be a way to do this in-place with constant memory,
//         // but I couldn't figure out a simple way to do it. So just append
//         // the intersection to the end of this range, and then drain it before
//         // we're done.
//         let drain_end = self.0.len();

//         let mut ita = 0..drain_end;
//         let mut itb = 0..other.0.len();
//         let mut a = ita.next().unwrap();
//         let mut b = itb.next().unwrap();
//         loop {
//             if let Some(ab) = self.0[a].intersect(&other.0[b]) {
//                 self.0.push(ab);
//             }
//             let (it, aorb) =
//                 if self.0[a].upper() < other.0[b].upper() {
//                     (&mut ita, &mut a)
//                 } else {
//                     (&mut itb, &mut b)
//                 };
//             match it.next() {
//                 Some(v) => *aorb = v,
//                 None => break,
//             }
//         }
//         self.0.drain(..drain_end);
//     }

//     /// Subtract the given set from this set, in place.
//     pub fn sub(self, other: &Self) -> Self {
//         if self.0.is_empty() || other.0.is_empty() {
//             return;
//         }

//         // This algorithm is (to me) surprisingly complex. A search of the
//         // interwebs indicate that this is a potentially interesting problem.
//         // Folks seem to suggest interval or segment trees, but I'd like to
//         // avoid the overhead (both runtime and conceptual) of that.
//         //
//         // The following is basically my Shitty First Draft. Therefore, in
//         // order to grok it, you probably need to read each line carefully.
//         // Simplifications are most welcome!
//         //
//         // Remember, we can assume the canonical format invariant here, which
//         // says that all ranges are sorted, not overlapping and not adjacent in
//         // each class.
//         let drain_end = self.0.len();
//         let (mut a, mut b) = (0, 0);
//         'LOOP: while a < drain_end && b < other.0.len() {
//             // Basically, the easy cases are when neither range overlaps with
//             // each other. If the `b` range is less than our current `a`
//             // range, then we can skip it and move on.
//             if other.0[b].upper() < self.0[a].lower() {
//                 b += 1;
//                 continue;
//             }
//             // ... similarly for the `a` range. If it's less than the smallest
//             // `b` range, then we can add it as-is.
//             if self.0[a].upper() < other.0[b].lower() {
//                 let range = self.0[a];
//                 self.0.push(range);
//                 a += 1;
//                 continue;
//             }
//             // Otherwise, we have overlapping ranges.
//             assert!(!self.0[a].is_intersection_empty(&other.0[b]));

//             // This part is tricky and was non-obvious to me without looking
//             // at explicit examples (see the tests). The trickiness stems from
//             // two things: 1) subtracting a range from another range could
//             // yield two ranges and 2) after subtracting a range, it's possible
//             // that future ranges can have an impact. The loop below advances
//             // the `b` ranges until they can't possible impact the current
//             // range.
//             //
//             // For example, if our `a` range is `a-t` and our next three `b`
//             // ranges are `a-c`, `g-i`, `r-t` and `x-z`, then we need to apply
//             // subtraction three times before moving on to the next `a` range.
//             let mut range = self.0[a];
//             while b < other.0.len()
//                 && !range.is_intersection_empty(&other.0[b])
//             {
//                 let old_range = range;
//                 range = match range.difference(&other.0[b]) {
//                     (None, None) => {
//                         // We lost the entire range, so move on to the next
//                         // without adding this one.
//                         a += 1;
//                         continue 'LOOP;
//                     }
//                     (Some(range1), None) | (None, Some(range1)) => range1,
//                     (Some(range1), Some(range2)) => {
//                         self.0.push(range1);
//                         range2
//                     }
//                 };
//                 // It's possible that the `b` range has more to contribute
//                 // here. In particular, if it is greater than the original
//                 // range, then it might impact the next `a` range *and* it
//                 // has impacted the current `a` range as much as possible,
//                 // so we can quit. We don't bump `b` so that the next `a`
//                 // range can apply it.
//                 if other.0[b].upper() > old_range.upper() {
//                     break;
//                 }
//                 // Otherwise, the next `b` range might apply to the current
//                 // `a` range.
//                 b += 1;
//             }
//             self.0.push(range);
//             a += 1;
//         }
//         while a < drain_end {
//             let range = self.0[a];
//             self.0.push(range);
//             a += 1;
//         }
//         self.0.drain(..drain_end);
//     }


//     /// Negate this interval set.
//     ///
//     /// For all `x` where `x` is any element, if `x` was in this set, then it
//     /// will not be in this set after negation.
//     pub fn not(self) -> Self {
//         if self.0.is_empty() {
//             let (min, max) = (I::MIN, I::MAX);
//             order::add(self.0, I::new(min, max))
//         }

//         // There should be a way to do this in-place with constant memory,
//         // but I couldn't figure out a simple way to do it. So just append
//         // the negation to the end of this range, and then drain it before
//         // we're done.
//         let drain_end = self.0.len();

//         // We do checked arithmetic below because of the canonical ordering
//         // invariant.
//         if self.0[0].lower() > I::MIN {
//             let upper = self.0[0].lower().decrement();
//             self.0.push(I::new(I::MIN, upper));
//         }
//         for i in 1..drain_end {
//             let lower = self.0[i - 1].upper().increment();
//             let upper = self.0[i].lower().decrement();
//             order::add(self.0, I::new(lower, upper))
//         }
//         if self.0[drain_end - 1].upper() < I::MAX {
//             let lower = self.0[drain_end - 1].upper().increment();
//             order::add(self.0, I::new(lower, I::MAX))
//         }
//         self.0.drain(..drain_end);
//     }

//     /// Converts this set into a canonical ordering.
//     fn canonicalize(&mut self) {
//         if self.is_canonical() {
//             return;
//         }
//         self.0.sort();
//         assert!(!self.0.is_empty());

//         // Is there a way to do this in-place with constant memory? I couldn't
//         // figure out a way to do it. So just append the canonicalization to
//         // the end of this range, and then drain it before we're done.
//         let drain_end = self.0.len();
//         for oldi in 0..drain_end {
//             // If we've added at least one new range, then check if we can
//             // merge this range in the previously added range.
//             if self.0.len() > drain_end {
//                 let (last, rest) = self.0.split_last_mut().unwrap();
//                 if let Some(union) = last.union(&rest[oldi]) {
//                     *last = union;
//                     continue;
//                 }
//             }
//             let range = self.0[oldi];
//             self.0.push(range);
//         }
//         self.0.drain(..drain_end);
//     }

//     /// Returns true if and only if this class is in a canonical ordering.
//     fn is_canonical(&self) -> bool {
//         for pair in self.0.windows(2) {
//             if pair[0] >= pair[1] {
//                 return false;
//             }
//             if pair[0].is_contiguous(&pair[1]) {
//                 return false;
//             }
//         }
//         true
//     }
// }
