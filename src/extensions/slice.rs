/// Slice operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a slice
/// - Does not consume the collection or its elements
/// - Does not create a new collection
///
pub trait Slice<Item> {
  /// Creates a new slice from this slice without the last element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = [1, 2, 3];
  ///
  /// assert_eq!(a.init_ref(), [1, 2]);
  /// ```
  fn init_ref(&self) -> &Self;

  /// Creates a new slice that skips the first `n` elements from the original slice.
  ///
  /// `skip(n)` skips elements until `n` elements are skipped or the end of the
  /// slice is reached (whichever happens first). After that, all the remaining
  /// elements are yielded. In particular, if the original slice is too short,
  /// then the returned slice is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.skip_ref(2), [3]);
  /// ```
  fn skip_ref(&self, n: usize) -> &Self;

  /// Creates a new slice without initial elements based on a predicate.
  ///
  /// [`skip`]: Slice::skip_ref
  ///
  /// `skip_while()` takes a closure as an argument. It will call this
  /// closure on each element of this slice, and ignore elements
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
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.skip_while_ref(|&x| x < 3), [3]);
  /// ```
  fn skip_while_ref(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;

  /// Creates a new slice from the original slice without the first element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = [1, 2, 3];
  ///
  /// assert_eq!(a.tail_ref(), [2, 3]);
  /// ```
  fn tail_ref(&self) -> &Self;

  /// Creates a new slice that yields the first `n` elements, or fewer
  /// if the original slice has fewer than `n` elements.
  ///
  /// `take(n)` yields elements until `n` elements are yielded or the end of
  /// this slice is reached (whichever happens first).
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
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.take_ref(2), [1, 2]);
  /// ```
  ///
  /// If less than `n` elements are available,
  /// `take` will limit itself to the size of the original slice:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.take_ref(2), [1, 2]);
  /// assert_eq!(a.take_ref(5), [1, 2, 3]);
  /// ```
  fn take_ref(&self, n: usize) -> &Self;

  /// Creates a new slice without trailing elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of this slice, and yield elements
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
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.take_while_ref(|&x| x < 3), [1, 2]);
  /// ```
  fn take_while_ref(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;
}
