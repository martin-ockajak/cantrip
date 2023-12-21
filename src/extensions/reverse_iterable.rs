pub trait ReverseIterable<Item> {
  /// Searches for an element of an collection from the back that satisfies a predicate.
  ///
  /// `rfind()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, starting at the end, and if any
  /// of them return `true`, then `rfind()` returns [`Some(element)`]. If they all return
  /// `false`, it returns [`None`].
  ///
  /// `rfind()` is short-circuiting; in other words, it will stop processing
  /// as soon as the closure returns `true`.
  ///
  /// Because `rfind()` takes a reference, and many collections contain
  /// references, this leads to a possibly confusing situation where the
  /// argument is a double reference. You can see this effect in the
  /// examples below, with `&&x`.
  ///
  /// [`Some(element)`]: Some
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rfind(|&x| x == 2), Some(&2));
  ///
  /// assert_eq!(a.rfind(|&x| x == 5), None);
  /// ```
  ///
  /// Stopping at the first `true`:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rfind(|&x| x == 2), Some(&2));
  /// ```
  fn rfind(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  /// An collection method that reduces the collection's elements to a single,
  /// final value, starting from the back.
  ///
  /// This is the reverse version of [`Iterator::fold()`]: it takes elements
  /// starting from the back of the collection.
  ///
  /// `rfold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of the collection, `rfold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// Note: `rfold()` combines elements in a *right-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *left-associative* version of `rfold()`, see [`Iterator::fold()`].
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // the sum of all of the elements of a
  /// let sum = a.rfold(0, |acc, &x| acc + x);
  ///
  /// assert_eq!(sum, 6);
  /// ```
  ///
  /// This example demonstrates the right-associative nature of `rfold()`:
  /// it builds a string, starting with an initial value
  /// and continuing with each element from the back until the front:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let numbers = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// let result = numbers.rfold(zero, |acc, &x| {
  ///     format!("({x} + {acc})")
  /// });
  ///
  /// assert_eq!(result, "(1 + (2 + (3 + (4 + (5 + 0)))))");
  /// ```
  fn rfold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B;


  /// Searches for an element in an collection from the right, returning its
  /// index.
  ///
  /// `rposition()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, starting from the end,
  /// and if one of them returns `true`, then `rposition()` returns
  /// [`Some(index)`]. If all of them return `false`, it returns [`None`].
  ///
  /// `rposition()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a `true`.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rposition(|&x| x == 3), Some(2));
  ///
  /// assert_eq!(a.rposition(|&x| x == 5), None);
  /// ```
  ///
  /// Stopping at the first `true`:
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = vec![-1, 2, 3, 4];
  ///
  /// assert_eq!(a.rposition(|&x| x >= 2), Some(3));
  /// ```
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;
}
