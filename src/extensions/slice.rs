pub trait Slice<Item> {
  fn init(&self) -> &Self;

  /// Creates an slice without initial elements based on a predicate.
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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = &[-1i32, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|x| x.is_negative()), &[0, 1]);
  /// ```
  ///
  /// Because the closure passed to `skip_while()` takes a reference, and some
  /// slices contain references, this leads to a possibly confusing
  /// situation, where the type of the closure argument is a double reference:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = &[&-1, &0, &1];
  ///
  /// assert_eq!(a.skip_while(|x| **x < 0), &[&0, &1]); // need two *s!
  /// ```
  fn skip_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;

  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  fn tail(&self) -> &Self;

  /// Creates an slice without trailing elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of the slice, and yield elements
  /// while it returns `true`.
  ///
  /// After `false` is returned, `take_while()`'s job is over, and the
  /// rest of the elements are ignored.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = &[-1i32, 0, 1];
  ///
  /// assert_eq!(a.take_while(|x| x.is_negative()), &[-1]);
  /// ```
  ///
  /// Because the closure passed to `take_while()` takes a reference, and some
  /// slices contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = &[&-1, &0, &1];
  ///
  /// assert_eq!(a.take_while(|x| **x < 0), &[&-1]); // need two *s!
  /// ```
  fn take_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;
}
