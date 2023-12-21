// #![deny(unsafe_op_in_unsafe_fn)]
//
// use std::cmp::Ordering;
// use std::iter::FromIterator;
// use core::fmt;
// use core::mem::{swap, ManuallyDrop};
// use core::ptr;
// use std::vec;
//
// pub(crate) struct BinaryHeap<T, C> where C: Fn(&T, &T) -> Ordering {
//     data: Vec<T>,
//     cmp: C,
// }
//
// impl<T: Clone, C: Clone> Clone for BinaryHeap<T, C> {
//     fn clone(&self) -> Self {
//         BinaryHeap {
//             data: self.data.clone(),
//             cmp: self.cmp.clone(),
//         }
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         self.data.clone_from(&source.data);
//     }
// }
//
// impl<T: fmt::Debug, C> fmt::Debug for BinaryHeap<T, C> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.iter()).finish()
//     }
// }
//
// impl<T, C> BinaryHeap<T, C> {
//     pub fn from_vec_cmp(vec: Vec<T>, cmp: C) -> Self {
//         unsafe { BinaryHeap::from_vec_cmp_raw(vec, cmp, true) }
//     }
//
//     unsafe fn from_vec_cmp_raw(vec: Vec<T>, cmp: C, rebuild: bool) -> Self {
//         let mut heap = BinaryHeap { data: vec, cmp };
//         if rebuild && !heap.data.is_empty() {
//             heap.rebuild();
//         }
//         heap
//     }
// }
//
// impl<T, F> BinaryHeap<T, Comparator<T, F>>
//     where
//       F: Fn(&T, &T) -> Ordering,
// {
//     pub fn new_by(f: F) -> Self {
//         BinaryHeap::from_vec_cmp(vec![], Comparator(f))
//     }
//
//     pub fn with_capacity_by(capacity: usize, f: F) -> Self {
//         BinaryHeap::from_vec_cmp(Vec::with_capacity(capacity), Comparator(f))
//     }
// }
//
// impl<T, C> BinaryHeap<T, C> {
//
//     /// Removes the greatest item from the binary heap and returns it, or `None` if it
//     /// is empty.
//     ///
//     /// # Time complexity
//     ///
//     /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
//     pub fn pop(&mut self) -> Option<T> {
//         self.data.pop().map(|mut item| {
//             if !self.is_empty() {
//                 swap(&mut item, &mut self.data[0]);
//                 // SAFETY: !self.is_empty() means that self.len() > 0
//                 unsafe { self.sift_down_to_bottom(0) };
//             }
//             item
//         })
//     }
//
//     /// Pushes an item onto the binary heap.
//     ///
//     /// # Time complexity
//     ///
//     /// The expected cost of `push`, averaged over every possible ordering of
//     /// the elements being pushed, and over a sufficiently large number of
//     /// pushes, is *O*(1). This is the most meaningful cost metric when pushing
//     /// elements that are *not* already in any sorted pattern.
//     ///
//     /// The time complexity degrades if elements are pushed in predominantly
//     /// ascending order. In the worst case, elements are pushed in ascending
//     /// sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
//     /// containing *n* elements.
//     ///
//     /// The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
//     /// occurs when capacity is exhausted and needs a resize. The resize cost
//     /// has been amortized in the previous figures.
//     // #[stable(feature = "rust1", since = "1.0.0")]
//     pub fn push(&mut self, item: T) {
//         let old_len = self.len();
//         self.data.push(item);
//         // SAFETY: Since we pushed a new item it means that
//         //  old_len = self.len() - 1 < self.len()
//         unsafe { self.sift_up(0, old_len) };
//     }
//
//     /// Consumes the `BinaryHeap` and returns a vector in sorted
//     /// (ascending) order.
//     ///
//     pub fn into_sorted_vec(mut self) -> Vec<T> {
//         let mut end = self.len();
//         while end > 1 {
//             end -= 1;
//             // SAFETY: `end` goes from `self.len() - 1` to 1 (both included),
//             //  so it's always a valid index to access.
//             //  It is safe to access index 0 (i.e. `ptr`), because
//             //  1 <= end < self.len(), which means self.len() >= 2.
//             unsafe {
//                 let ptr = self.data.as_mut_ptr();
//                 ptr::swap(ptr, ptr.add(end));
//             }
//             // SAFETY: `end` goes from `self.len() - 1` to 1 (both included) so:
//             //  0 < 1 <= end <= self.len() - 1 < self.len()
//             //  Which means 0 < end and end < self.len().
//             unsafe { self.sift_down_range(0, end) };
//         }
//         self.into_vec()
//     }
//
//     // The implementations of sift_up and sift_down use unsafe blocks in
//     // order to move an element out of the vector (leaving behind a
//     // hole), shift along the others and move the removed element back into the
//     // vector at the final location of the hole.
//     // The `Hole` type is used to represent this, and make sure
//     // the hole is filled back at the end of its scope, even on panic.
//     // Using a hole reduces the constant factor compared to using swaps,
//     // which involves twice as many moves.
//
//     /// # Safety
//     ///
//     /// The caller must guarantee that `pos < self.len()`.
//     unsafe fn sift_up(&mut self, start: usize, pos: usize) -> usize {
//         // Take out the value at `pos` and create a hole.
//         // SAFETY: The caller guarantees that pos < self.len()
//         let mut hole = unsafe { Hole::new(&mut self.data, pos) };
//
//         while hole.pos() > start {
//             let parent = (hole.pos() - 1) / 2;
//
//             // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
//             //  and so hole.pos() - 1 can't underflow.
//             //  This guarantees that parent < hole.pos() so
//             //  it's a valid index and also != hole.pos().
//             if self
//               .cmp
//               .compares_le(hole.element(), unsafe { hole.get(parent) })
//             {
//                 break;
//             }
//
//             // SAFETY: Same as above
//             unsafe { hole.move_to(parent) };
//         }
//
//         hole.pos()
//     }
//
//     /// Take an element at `pos` and move it down the heap,
//     /// while its children are larger.
//     ///
//     /// # Safety
//     ///
//     /// The caller must guarantee that `pos < end <= self.len()`.
//     unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
//         // SAFETY: The caller guarantees that pos < end <= self.len().
//         let mut hole = unsafe { Hole::new(&mut self.data, pos) };
//         let mut child = 2 * hole.pos() + 1;
//
//         // Loop invariant: child == 2 * hole.pos() + 1.
//         while child <= end.saturating_sub(2) {
//             // compare with the greater of the two children
//             // SAFETY: child < end - 1 < self.len() and
//             //  child + 1 < end <= self.len(), so they're valid indexes.
//             //  child == 2 * hole.pos() + 1 != hole.pos() and
//             //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
//             // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
//             //  if T is a ZST
//             child += unsafe { self.cmp.compares_le(hole.get(child), hole.get(child + 1)) } as usize;
//
//             // if we are already in order, stop.
//             // SAFETY: child is now either the old child or the old child+1
//             //  We already proven that both are < self.len() and != hole.pos()
//             if self
//               .cmp
//               .compares_ge(hole.element(), unsafe { hole.get(child) })
//             {
//                 return;
//             }
//
//             // SAFETY: same as above.
//             unsafe { hole.move_to(child) };
//             child = 2 * hole.pos() + 1;
//         }
//
//         // SAFETY: && short circuit, which means that in the
//         //  second condition it's already true that child == end - 1 < self.len().
//         if child == end - 1
//           && self
//           .cmp
//           .compares_lt(hole.element(), unsafe { hole.get(child) })
//         {
//             // SAFETY: child is already proven to be a valid index and
//             //  child == 2 * hole.pos() + 1 != hole.pos().
//             unsafe { hole.move_to(child) };
//         }
//     }
//
//     /// # Safety
//     ///
//     /// The caller must guarantee that `pos < self.len()`.
//     unsafe fn sift_down(&mut self, pos: usize) {
//         let len = self.len();
//         // SAFETY: pos < len is guaranteed by the caller and
//         //  obviously len = self.len() <= self.len().
//         unsafe { self.sift_down_range(pos, len) };
//     }
//
//     /// Take an element at `pos` and move it all the way down the heap,
//     /// then sift it up to its position.
//     ///
//     /// Note: This is faster when the element is known to be large / should
//     /// be closer to the bottom.
//     ///
//     /// # Safety
//     ///
//     /// The caller must guarantee that `pos < self.len()`.
//     unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
//         let end = self.len();
//         let start = pos;
//
//         // SAFETY: The caller guarantees that pos < self.len().
//         let mut hole = unsafe { Hole::new(&mut self.data, pos) };
//         let mut child = 2 * hole.pos() + 1;
//
//         // Loop invariant: child == 2 * hole.pos() + 1.
//         while child <= end.saturating_sub(2) {
//             // SAFETY: child < end - 1 < self.len() and
//             //  child + 1 < end <= self.len(), so they're valid indexes.
//             //  child == 2 * hole.pos() + 1 != hole.pos() and
//             //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
//             // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
//             //  if T is a ZST
//             child += unsafe { self.cmp.compares_le(hole.get(child), hole.get(child + 1)) } as usize;
//
//             // SAFETY: Same as above
//             unsafe { hole.move_to(child) };
//             child = 2 * hole.pos() + 1;
//         }
//
//         if child == end - 1 {
//             // SAFETY: child == end - 1 < self.len(), so it's a valid index
//             //  and child == 2 * hole.pos() + 1 != hole.pos().
//             unsafe { hole.move_to(child) };
//         }
//         pos = hole.pos();
//         drop(hole);
//
//         // SAFETY: pos is the position in the hole and was already proven
//         //  to be a valid index.
//         unsafe { self.sift_up(start, pos) };
//     }
//
//     /// Rebuild assuming data[0..start] is still a proper heap.
//     fn rebuild_tail(&mut self, start: usize) {
//         if start == self.len() {
//             return;
//         }
//
//         let tail_len = self.len() - start;
//
//         #[inline(always)]
//         fn log2_fast(x: usize) -> usize {
//             (usize::BITS - x.leading_zeros() - 1) as usize
//         }
//
//         // `rebuild` takes O(self.len()) operations
//         // and about 2 * self.len() comparisons in the worst case
//         // while repeating `sift_up` takes O(tail_len * log(start)) operations
//         // and about 1 * tail_len * log_2(start) comparisons in the worst case,
//         // assuming start >= tail_len. For larger heaps, the crossover point
//         // no longer follows this reasoning and was determined empirically.
//         let better_to_rebuild = if start < tail_len {
//             true
//         } else if self.len() <= 2048 {
//             2 * self.len() < tail_len * log2_fast(start)
//         } else {
//             2 * self.len() < tail_len * 11
//         };
//
//         if better_to_rebuild {
//             self.rebuild();
//         } else {
//             for i in start..self.len() {
//                 // SAFETY: The index `i` is always less than self.len().
//                 unsafe { self.sift_up(0, i) };
//             }
//         }
//     }
//
//     fn rebuild(&mut self) {
//         let mut n = self.len() / 2;
//         while n > 0 {
//             n -= 1;
//             // SAFETY: n starts from self.len() / 2 and goes down to 0.
//             //  The only case when !(n < self.len()) is if
//             //  self.len() == 0, but it's ruled out by the loop condition.
//             unsafe { self.sift_down(n) };
//         }
//     }
// }
//
// impl<T, C> BinaryHeap<T, C> {
//     /// Returns an iterator which retrieves elements in heap order.
//     /// This method consumes the original heap.
//     pub fn into_iter_sorted(self) -> IntoIterSorted<T, C> {
//         IntoIterSorted { inner: self }
//     }
//
//     /// Returns the greatest item in the binary heap, or `None` if it is empty.
//     ///
//     /// # Time complexity
//     ///
//     /// Cost is *O*(1) in the worst case.
//     pub fn peek(&self) -> Option<&T> {
//         self.data.get(0)
//     }
//
//     /// Returns the number of elements the binary heap can hold without reallocating.
//     pub fn capacity(&self) -> usize {
//         self.data.capacity()
//     }
//
//     /// Reserves capacity for at least `additional` more elements to be inserted in the
//     /// `BinaryHeap`. The collection may reserve more space to avoid frequent reallocations.
//     ///
//     /// # Panics
//     ///
//     /// Panics if the new capacity overflows `usize`.
//     pub fn reserve(&mut self, additional: usize) {
//         self.data.reserve(additional);
//     }
//
//     /// Returns the length of the binary heap.
//     pub fn len(&self) -> usize {
//         self.data.len()
//     }
//
//     /// Checks if the binary heap is empty.
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }
//
// /// Hole represents a hole in a slice i.e., an index without valid value
// /// (because it was moved from or duplicated).
// /// In drop, `Hole` will restore the slice by filling the hole
// /// position with the value that was originally removed.
// struct Hole<'a, T: 'a> {
//     data: &'a mut [T],
//     elt: ManuallyDrop<T>,
//     pos: usize,
// }
//
// impl<'a, T> Hole<'a, T> {
//     /// Create a new `Hole` at index `pos`.
//     ///
//     /// Unsafe because pos must be within the data slice.
//     #[inline]
//     unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
//         debug_assert!(pos < data.len());
//         // SAFE: pos should be inside the slice
//         let elt = unsafe { ptr::read(data.get_unchecked(pos)) };
//         Hole {
//             data,
//             elt: ManuallyDrop::new(elt),
//             pos,
//         }
//     }
//
//     #[inline]
//     fn pos(&self) -> usize {
//         self.pos
//     }
//
//     /// Returns a reference to the element removed.
//     #[inline]
//     fn element(&self) -> &T {
//         &self.elt
//     }
//
//     /// Returns a reference to the element at `index`.
//     ///
//     /// Unsafe because index must be within the data slice and not equal to pos.
//     #[inline]
//     unsafe fn get(&self, index: usize) -> &T {
//         debug_assert!(index != self.pos);
//         debug_assert!(index < self.data.len());
//         unsafe { self.data.get_unchecked(index) }
//     }
//
//     /// Move hole to new location
//     ///
//     /// Unsafe because index must be within the data slice and not equal to pos.
//     #[inline]
//     unsafe fn move_to(&mut self, index: usize) {
//         debug_assert!(index != self.pos);
//         debug_assert!(index < self.data.len());
//         unsafe {
//             let ptr = self.data.as_mut_ptr();
//             let index_ptr: *const _ = ptr.add(index);
//             let hole_ptr = ptr.add(self.pos);
//             ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
//         }
//         self.pos = index;
//     }
// }
//
// impl<T> Drop for Hole<'_, T> {
//     #[inline]
//     fn drop(&mut self) {
//         // fill the hole again
//         unsafe {
//             let pos = self.pos;
//             ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
//         }
//     }
// }
//
// #[derive(Clone, Debug)]
// pub struct IntoIterSorted<T, C> {
//     inner: BinaryHeap<T, C>,
// }
//
// impl<T, C> Iterator for IntoIterSorted<T, C> {
//     type Item = T;
//
//     #[inline]
//     fn next(&mut self) -> Option<T> {
//         self.inner.pop()
//     }
//
//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let exact = self.inner.len();
//         (exact, Some(exact))
//     }
// }
//
// impl<T: Ord, C> From<Vec<T>> for BinaryHeap<T, C> {
//     /// Converts a `Vec<T>` into a `BinaryHeap<T>`.
//     ///
//     /// This conversion happens in-place, and has *O*(*n*) time complexity.
//     fn from(vec: Vec<T>) -> Self {
//         BinaryHeap::from_vec(vec)
//     }
// }
//
// impl<T: Ord, C> FromIterator<T> for BinaryHeap<T, C> {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         BinaryHeap::from(iter.into_iter().collect::<Vec<_>>())
//     }
// }
//
// impl<T, C> Extend<T> for BinaryHeap<T, C> {
//     #[inline]
//     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         self.extend_desugared(iter);
//     }
// }
//
// impl<T, C> BinaryHeap<T, C> {
//     fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
//         let iterator = iter.into_iter();
//         let (lower, _) = iterator.size_hint();
//
//         self.reserve(lower);
//
//         iterator.for_each(move |elem| self.push(elem));
//     }
// }
