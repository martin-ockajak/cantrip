use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;

use crate::Iterable;

/// Ordered collection operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent an ordered collection
///
pub trait Ordered<Item> {
  /// Computes the length of the longest common prefix shared by this sequence and another collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.common_prefix_length(&vec![1, 2, 3, 4]), 3);
  /// assert_eq!(a.common_prefix_length(&vec![1, 2]), 2);
  ///
  /// assert_eq!(a.common_prefix_length(&vec![]), 0);
  /// ```
  fn common_prefix_length<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> usize
  where
    Item: PartialEq + 'a;

  /// Computes the length of the longest common suffix shared by this sequence and another collection.
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

  /// Counts number of unique elements in this sequence.
  ///
  /// Returns `0` for an empty sequence.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let b = vec![1, 1];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.count_unique(), 3);
  /// assert_eq!(b.count_unique(), 1);
  ///
  /// assert_eq!(e.count_unique(), 0);
  /// ```
  fn count_unique(&self) -> usize
  where
    Item: Eq + Hash;

  /// Tests if this sequence contains all elements of another collection exactly
  /// as many times as their appear in the other collection and vice versa.
  ///
  /// Returns `true` if the other collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 3];
  ///
  /// assert!(a.equivalent(&vec![3, 2, 1, 3]));
  ///
  /// assert!(!a.equivalent(&vec![1, 2, 2]));
  /// assert!(!a.equivalent(&vec![1, 1, 2, 3, 3]));
  /// ```
  fn equivalent<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a;

  /// Tests if this sequence contains all elements of another collection
  /// at least as many times as their appear in the other collection.
  ///
  /// Returns `true` if the other collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert!(a.includes(&vec![1, 2]));
  /// assert!(a.includes(&vec![1, 3, 3]));
  /// assert!(a.includes(&vec![]));
  ///
  /// assert!(!a.includes(&vec![1, 1, 2]));
  /// assert!(!a.includes(&vec![3, 4]));
  /// assert!(!e.includes(&vec![1]));
  /// ```
  fn includes<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a;

  /// Combine all collection elements into one `String`, separated by `sep`.
  ///
  /// Use the `Display` implementation of each element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = [1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.join_items(", "), "1, 2, 3");
  /// assert_eq!(e.join_items(", "), "");
  /// ```
  fn join_items(&self, separator: &str) -> String
  where
    Item: Display;

  /// Searches for an element in this sequence, returning its index.
  ///
  /// `position()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this sequence, and if one of them
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
  /// This function might panic if this sequence has more than `usize::MAX`
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
  /// assert_eq!(a.position(|&x| x == 2), Some(1));
  ///
  /// assert_eq!(a.position(|&x| x == 5), None);
  /// ```
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  /// Searches for an element in this sequence, returning all its indices.
  ///
  /// `positions()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this sequence, each time one of them
  /// returns `true`, then `positions()` adds the element index to its result.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if this sequence has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.positions(|&x| x % 2 == 0), vec![1]);
  /// assert_eq!(a.positions(|&x| x % 2 == 1), vec![0, 2]);
  ///
  /// assert_eq!(a.positions(|&x| x > 3), vec![]);
  /// ```
  fn positions(&self, predicate: impl FnMut(&Item) -> bool) -> Vec<usize>;

  /// Searches for an element in this sequence, returning its index.
  ///
  /// `position_of()` compares each element of this sequence with the specified value,
  /// and if one of them matches, then `position_of()` returns [`Some(index)`].
  /// If none of the elements match, it returns [`None`].
  ///
  /// `position_of()` is short-circuiting; in other words, it will stop
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
  /// This function might panic if this sequence has more than `usize::MAX`
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
  /// assert_eq!(a.position_of(&2), Some(1));
  ///
  /// assert_eq!(a.position_of(&5), None);
  /// ```
  #[inline]
  fn position_of(&self, value: &Item) -> Option<usize>
  where
    Item: PartialEq,
  {
    self.position(|x| x == value)
  }

  /// Searches for an element in this sequence, returning all its indices.
  ///
  /// `positions_of()` compares each element of this sequence with the specified value,
  /// and each time one of them matches, then `indices_f()` adds the element index
  /// to its result.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if this sequence has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 1];
  ///
  /// assert_eq!(a.positions_of(&1), vec![0, 2]);
  ///
  /// assert_eq!(a.positions_of(&5), vec![]);
  /// ```
  #[inline]
  fn positions_of(&self, value: &Item) -> Vec<usize>
  where
    Item: PartialEq,
  {
    self.positions(|x| x == value)
  }

  /// Searches for a sub-sequence in this sequence, returning its index.
  ///
  /// After finding a starting element of specified sequence in this sequence,
  /// `position_seq()` compares each element of this sequence with the specified value,
  /// and if all of them matche, then `position_seq()` returns [`Some(start_index)`].
  /// If any of the elements do not match, it returns [`None`].
  ///
  /// `position_seq()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a matching sequence.
  ///
  /// Returns `Some(0)` if specified sequence is empty.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if this sequence has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(start_index)`]: Some
  /// [`Some(0)`]: Some
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(a.position_sequence(&vec![2, 3, 4]), Some(1));
  /// assert_eq!(a.position_sequence(&vec![]), Some(0));
  ///
  /// assert_eq!(a.position_sequence(&vec![1, 3]), None);
  /// ```
  fn position_sequence<'a>(&'a self, sequence: &'a impl Iterable<Item<'a> = &'a Item>) -> Option<usize>
  where
    Item: PartialEq + 'a;

  /// Searches for an element of this sequence that satisfies a predicate, starting from the back.
  ///
  /// `rfind()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this sequence, starting at the end, and if any
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

  /// A collection method that reduces this sequence's elements to a single,
  /// final value, starting from the back.
  ///
  /// This is the reverse version of [`Iterator::fold()`]: it takes elements
  /// starting from the back of this sequence.
  ///
  /// `rfold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of this sequence, `rfold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// This is a non-consuming variant of [`rfold_to`].
  ///
  /// Note: `rfold()` combines elements in a *right-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *left-associative* version of `rfold()`, see [`Iterator::fold()`].
  ///
  /// [`rfold_to`]: crate::Sequence::rfold_to
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
  fn rfold<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B;

  /// Searches for an element in this sequence from the right, returning its index.
  ///
  /// `rposition()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this sequence, starting from the end,
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

pub(crate) fn common_prefix_length<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> usize
where
  Item: PartialEq + 'a,
{
  let mut result = 0_usize;
  for (item, element) in iterator.zip(elements.iterator()) {
    if item != element {
      return result;
    }
    result += 1;
  }
  result
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

#[inline]
pub(crate) fn count_unique<'a, Item: Eq + Hash + 'a>(iterator: impl Iterator<Item = &'a Item>) -> usize {
  let items: HashSet<&Item> = HashSet::from_iter(iterator);
  items.len()
}

pub(crate) fn equivalent<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool
where
  Item: Eq + Hash + 'a,
{
  let elements_iterator = elements.iterator();
  let mut excluded: HashMap<&Item, usize> = HashMap::with_capacity(iterator.size_hint().0);
  let mut remaining = 0_usize;
  for item in elements_iterator {
    *excluded.entry(item).or_default() += 1;
    remaining += 1;
  }
  for item in iterator {
    match excluded.get_mut(item) {
      Some(count) => {
        if *count > 0 {
          *count -= 1;
          remaining = remaining.saturating_sub(1);
        } else {
          return false
        }
      },
      None => return false
    }
  };
  remaining == 0
}

pub(crate) fn includes<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool
where
  Item: Eq + Hash + 'a,
{
  let elements_iterator = elements.iterator();
  let mut excluded: HashMap<&Item, usize> = HashMap::with_capacity(iterator.size_hint().0);
  let mut remaining = 0_usize;
  for item in elements_iterator {
    *excluded.entry(item).or_default() += 1;
    remaining += 1;
  }
  for item in iterator {
    if let Some(count) = excluded.get_mut(item) {
      if *count > 0 {
        *count -= 1;
        remaining = remaining.saturating_sub(1);
      }
    }
  };
  remaining == 0
}

pub(crate) fn join_items<'a, Item: Display + 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, separator: &str,
) -> String {
  match iterator.next() {
    Some(item) => {
      let mut result = String::with_capacity(separator.len() * iterator.size_hint().0);
      write!(&mut result, "{}", item).unwrap();
      for item in iterator {
        result.push_str(separator);
        write!(&mut result, "{}", item).unwrap();
      }
      result.shrink_to_fit();
      result
    }
    None => String::new(),
  }
}

#[inline]
pub(crate) fn positions<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> Vec<usize>
where
  Item: 'a,
{
  iterator.enumerate().filter(|(_, item)| predicate(item)).map(|(index, _)| index).collect()
}

pub(crate) fn position_sequence<'a, Item>(
  mut iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> Option<usize>
where
  Item: PartialEq + 'a,
{
  let mut elements_iterator = elements.iterator();
  if let Some(first_element) = elements_iterator.next() {
    if let Some(start_index) = iterator.position(|item| item == first_element) {
      for (item, element) in iterator.zip(elements_iterator) {
        if item != element {
          return None;
        }
      }
      Some(start_index)
    } else {
      None
    }
  } else {
    Some(0)
  }
}
