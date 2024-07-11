use crate::Iterable;

/// Reversible collection operations.
///
/// Methods have the following properties:
///
/// - Requires an efficient way to traverse the collection in reverse direction
/// - Does not consume the collection or its elements
/// - Does not create a new collection
///
pub trait Reversible<Item> {
  /// Computes the length of the longest common suffix shared by this collection and another collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.common_suffix_length(&vec![0, 1, 2, 3]), 3);
  /// assert_eq!(a.common_suffix_length(&vec![2, 3]), 2);
  ///
  /// assert_eq!(a.common_suffix_length(&vec![]), 0);
  /// ```
  fn common_suffix_length<'a, I>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item, Iterator<'a> = I>) -> usize
  where
    I: DoubleEndedIterator<Item = &'a Item>,
    Item: PartialEq + 'a;

  /// Searches for an element of this collection that satisfies a predicate, starting from the back.
  ///
  /// `rfind()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this collection, starting at the end, and if any
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
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rfind(|&x| x == 2), Some(&2));
  ///
  /// assert_eq!(a.rfind(|&x| x == 5), None);
  /// ```
  fn rfind(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  /// A collection method that reduces this collection's elements to a single,
  /// final value, starting from the back.
  ///
  /// This is the reverse version of [`Iterator::fold()`]: it takes elements
  /// starting from the back of this collection.
  ///
  /// `rfold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of this collection, `rfold()`
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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // the sum of all the elements of a
  /// let sum = a.rfold(0, |acc, x| acc + x);
  ///
  /// assert_eq!(sum, 6);
  /// ```
  ///
  /// This example demonstrates the right-associative nature of `rfold()`:
  /// it builds a string, starting with an initial value
  /// and continuing with each element from the back until the front:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let numbers = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// let result = numbers.rfold(zero, |acc, x| {
  ///   format!("({x} + {acc})")
  /// });
  ///
  /// assert_eq!(result, "(1 + (2 + (3 + (4 + (5 + 0)))))");
  /// ```
  #[inline]
  fn rfold<B, I>(self, initial_value: B, function: impl FnMut(B, Item) -> B) -> B
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + Sized,
  {
    let iterator = self.into_iter();
    iterator.rfold(initial_value, function)
  }

  /// Searches for an element in this collection from the right, returning its index.
  ///
  /// `rposition()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this collection, starting from the end,
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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rposition(|&x| x == 3), Some(2));
  /// assert_eq!(a.rposition(|&x| x == 5), None);
  /// ```
  ///
  /// Stopping at the first `true`:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 2, 3, 4];
  ///
  /// assert_eq!(a.rposition(|&x| x >= 2), Some(3));
  ///
  /// assert_eq!(a.rposition(|&x| x == 5), None);
  /// ```
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;
}

pub(crate) fn common_suffix_length<'a, Item, I>(
  reversed_iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item, Iterator<'a> = I>,
) -> usize
where
  I: DoubleEndedIterator<Item = &'a Item>,
  Item: PartialEq + 'a,
{
  let mut result = 0_usize;
  for (item, element) in reversed_iterator.zip(elements.iterator().rev()) {
    if item != element {
      return result;
    }
    result += 1;
  }
  result
}
