use std::hash::Hash;
use crate::Iterable;

/// Slice operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a slice
/// - Does not consume the collection or its elements
/// - Does not create a new collection
///
pub trait Slice<Item> {
  // FIXME - implement these methods
  // index_of_sequence
  // subsequence
  // subset
  // superset

  /// Tests if all elements of the slice are equal.
  ///
  /// `all_equal()` returns `true` if all elements of the slice are equal
  /// and `false` if a pair of unequal elements exist.
  ///
  /// An empty slice returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[1, 1, 1];
  /// let b = &[1, 2, 3];
  ///
  /// assert!(a.all_equal());
  ///
  /// assert!(!b.all_equal());
  /// ```
  fn all_equal(&self) -> bool
  where
    Item: PartialEq;

  /// Tests if all elements of the slice are unique.
  ///
  /// `all_equal()` returns `true` if all elements of the slice are unique
  /// and `false` if a pair of equal elements exist.
  ///
  /// An empty slice returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[1, 2, 3];
  /// let b = &[1, 1, 1];
  ///
  /// assert!(a.all_unique());
  ///
  /// assert!(!b.all_unique());
  /// ```
  fn all_unique(&self) -> bool
  where
    Item: Eq + Hash;

  /// Computes the length of the longest common prefix shared by a slice and another collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[1, 2, 3];
  ///
  /// assert_eq!(a.common_prefix_length(&vec![1, 2, 3, 4]), 3);
  /// assert_eq!(a.common_prefix_length(&vec![1, 2]), 2);
  ///
  /// assert_eq!(a.common_prefix_length(&vec![]), 0);
  /// ```
  fn common_prefix_length<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> usize
  where
    Item: PartialEq + 'a;

  /// Creates a slice from the original slice without the last element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = &[1, 2, 3];
  ///
  /// assert_eq!(a.init(), &[1, 2]);
  /// ```
  fn init(&self) -> &Self;

  /// Creates a slice that skips the first `n` elements from the original slice.
  ///
  /// `skip(n)` skips elements until `n` elements are skipped or the end of the
  /// slice is reached (whichever happens first). After that, all the remaining
  /// elements are yielded. In particular, if the original slice is too short,
  /// then the returned slice is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = &[1, 2, 3];
  ///
  /// assert_eq!(a.skip(2), &[3]);
  /// ```
  fn skip(&self, n: usize) -> &Self;

  /// Creates a slice without initial elements based on a predicate.
  ///
  /// [`skip`]: Slice::skip
  ///
  /// `skip_while()` takes a closure as an argument. It will call this
  /// closure on each element of the slice, and ignore elements
  /// until it returns `false`.
  ///
  /// After `false` is returned, `skip_while()`'s job is over, and the
  /// rest of the elements are yielded.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[-1, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|&x| x < 0), &[0, 1]);
  /// ```
  fn skip_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;

  /// Creates a slice from the original slice without the first element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = &[1, 2, 3];
  ///
  /// assert_eq!(a.tail(), &[2, 3]);
  /// ```
  fn tail(&self) -> &Self;

  /// Creates a slice that yields the first `n` elements, or fewer
  /// if the original slice has fewer than `n` elements.
  ///
  /// `take(n)` yields elements until `n` elements are yielded or the end of
  /// the slice is reached (whichever happens first).
  /// The returned slice is a prefix of length `n` if the original slice
  /// contains at least `n` elements, otherwise it contains all the
  /// (fewer than `n`) elements of the original slice.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[1, 2, 3];
  ///
  /// assert_eq!(a.take(2), &[1, 2]);
  /// ```
  ///
  /// If less than `n` elements are available,
  /// `take` will limit itself to the size of the original slice:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[1, 2];
  ///
  /// assert_eq!(a.take(5), &[1, 2]);
  /// ```
  fn take(&self, n: usize) -> &Self;

  /// Creates a slice without trailing elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of the slice, and yield elements
  /// while it returns `true`.
  ///
  /// After `false` is returned, `take_while()`'s job is over, and the
  /// rest of the elements are ignored.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[-1, 0, 1];
  ///
  /// assert_eq!(a.take_while(|&x| x < 0), &[-1]);
  /// ```
  fn take_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;
}
