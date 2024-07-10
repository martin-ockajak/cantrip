use crate::extensions::util::unfold::unfold;
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
  /// Computes the length of the longest common suffix shared by a collection and another collection.
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

  /// Searches for an element of a collection that satisfies a predicate, starting from the back.
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
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rfind(|&x| x == 2), Some(&2));
  /// assert_eq!(a.rfind(|&x| x == 5), None);
  /// ```
  fn rfind(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  /// A collection method that reduces the collection's elements to a single,
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

  /// Creates a collection by padding the original collection to a minimum length of
  /// `size` and filling missing elements with specified value, starting from the back.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // FIXME - check semantics
  /// let padded = a.pad(5, 4);
  /// assert_eq!(padded, vec![1, 2, 3, 4, 4]);
  /// ```
  #[inline]
  fn rpad<I>(self, size: usize, element: Item) -> Self
  where
    Item: Clone,
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.rpad_with(size, |_| element.clone())
  }

  /// Creates a collection by padding the original collection to a minimum length of
  /// `size` and filling missing elements using a closure `to_element`, starting from the back.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // FIXME - check semantics
  /// let padded = a.rpad_with(5, |i| 2 * i);
  /// assert_eq!(padded, vec![3, 2, 1, 6, 8]);
  /// ```
  #[inline]
  fn rpad_with<I>(self, size: usize, mut to_element: impl FnMut(usize) -> Item) -> Self
  where
    Item: Clone,
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter().rev();
    unfold(0_usize, |position| {
      let result = iterator.next().or_else(|| if *position < size { Some(to_element(*position)) } else { None });
      *position += 1;
      result
    })
    .collect()
  }

  /// Searches for an element in a collection from the right, returning its index.
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
  /// ```
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  /// Creates a collection that skips the first `n` elements from the original collection,
  /// starting from the back.
  ///
  /// `rskip(n)` skips elements until `n` elements are skipped or the beginning of the
  /// collection is reached (whichever happens first). After that, all the remaining
  /// elements are yielded. In particular, if the original collection is too short,
  /// then the returned collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rskip(2), vec![1]);
  /// ```
  #[inline]
  fn rskip<I>(self, n: usize) -> Self
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.into_iter().rev().skip(n).collect()
  }

  /// Creates a collection without trailing elements based on a predicate.
  ///
  /// [`rskip`]: Reversible::rskip
  ///
  /// `rskip_while()` takes a closure as an argument. It will call this
  /// closure on each element of the collection, starting at the end,
  /// and ignore elements until it returns `false`.
  ///
  /// After `false` is returned, `rskip_while()`'s job is over, and the
  /// rest of the elements are yielded.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1i32, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|x| x.is_negative()), &[0, 1]);
  /// ```
  ///
  /// Because the closure passed to `skip_while()` takes a reference, and some
  /// collections contain references, this leads to a possibly confusing
  /// situation, where the type of the closure argument is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|x| *x < 0), vec![0, 1]);
  /// ```
  #[inline]
  fn rskip_while<I>(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.into_iter().rev().skip_while(predicate).collect()
  }

  /// Creates a collection that yields the last `n` elements, or fewer
  /// if the original collection has fewer than `n` elements.
  ///
  /// `take(n)` yields elements until `n` elements are yielded or the start of
  /// the collection is reached (whichever happens first).
  /// The returned collection is a suffix of length `n` if the original collection
  /// contains at least `n` elements, otherwise it contains all the
  /// (fewer than `n`) elements of the original collection.
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
  /// assert_eq!(a.rtake(2), vec![3, 2]);
  /// ```
  ///
  /// If less than `n` elements are available,
  /// `take` will limit itself to the size of the original collection:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2];
  ///
  /// assert_eq!(a.rtake(5), vec![2, 1]);
  /// ```
  #[inline]
  fn rtake<I>(self, n: usize) -> Self
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.into_iter().rev().take(n).collect()
  }

  /// Creates a collection without initial elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of the collection, starting at the end,
  /// and yield elements while it returns `true`.
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
  /// let a = vec![-1i32, 0, 1];
  ///
  /// assert_eq!(a.rtake_while(|x| x.is_positive()), vec![1]);
  /// ```
  ///
  /// Because the closure passed to `take_while()` takes a reference, and some
  /// collections contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 0, 1];
  ///
  /// assert_eq!(a.take_while(|x| *x < 0), vec![-1]);
  /// ```
  #[inline]
  fn rtake_while<I>(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.into_iter().rev().take_while(predicate).collect()
  }
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
