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

  /// Searches for an element in a slice, returning its index.
  ///
  /// `position()` compares each element of the slice with the specified value,
  /// and if one of them matches, then `position()` returns [`Some(index)`].
  /// If none of the elements match, it returns [`None`].
  ///
  /// `position()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a matching element.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the slice has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.index_of(&2), Some(1));
  /// assert_eq!(a.index_of(&5), None);
  /// ```
  #[inline]
  fn index_of(&self, value: &Item) -> Option<usize>
  where
    Item: PartialEq,
  {
    self.position(|x| x == value)
  }

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

  /// Searches for an element in a slice, returning its index.
  ///
  /// `position()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the slice, and if one of them
  /// returns `true`, then `position()` returns [`Some(index)`]. If all of
  /// them return `false`, it returns [`None`].
  ///
  /// `position()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a `true`.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the slice has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = &[1, 2, 3];
  ///
  /// assert_eq!(a.position(|&x| x == 2), Some(1));
  /// assert_eq!(a.position(|&x| x == 5), None);
  /// ```
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

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
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
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
  /// use cantrip::*;
  ///
  /// let a = &[&-1, &0, &1];
  ///
  /// assert_eq!(a.skip_while(|x| **x < 0), &[&0, &1]); // need two *s!
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

  /// Creates a slice without trailing elements based on a predicate.
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
  /// use cantrip::*;
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
  /// use cantrip::*;
  ///
  /// let a = &[&-1, &0, &1];
  ///
  /// assert_eq!(a.take_while(|x| **x < 0), &[&-1]); // need two *s!
  /// ```
  fn take_while(&self, predicate: impl FnMut(&Item) -> bool) -> &Self;
}
