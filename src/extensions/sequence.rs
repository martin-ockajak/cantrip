use crate::Iterable;
use crate::extensions::frequencies;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};
use std::hash::Hash;

/// Ordered collection operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent an ordered collection
pub trait Sequence<Item>
where
  for<'i> &'i Self: IntoIterator<Item = &'i Item>,
{
  /// Computes the length of the longest common prefix shared by this sequence and another collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  //
  /// assert_eq!(a.common_prefix_length(&vec![1, 2, 3, 4]), 3);
  /// assert_eq!(a.common_prefix_length(&vec![1, 2]), 2);
  ///
  /// assert_eq!(a.common_prefix_length(&vec![]), 0);
  /// ```
  fn common_prefix_length<RefIterable>(&self, elements: &RefIterable) -> usize
  where
    for<'a> &'a RefIterable: IntoIterator<Item = &'a Item>,
    Item: PartialEq,
  {
    let iterator = self.into_iter();
    let mut result = 0_usize;
    for (item, element) in iterator.zip(elements.into_iter()) {
      if item != element {
        return result;
      }
      result += 1;
    }
    result
  }

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

  /// Counts the number of unique elements in this sequence.
  ///
  /// Returns `0` for an empty sequence.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.count_unique(), 3);
  ///
  /// assert_eq!(e.count_unique(), 0);
  /// ```
  fn count_unique(&self) -> usize
  where
    Item: Eq + Hash,
  {
    count_unique(self.into_iter())
  }

  /// Tests if this sequence contains all elements of another collection exactly
  /// as many times as they appear in the other collection and vice versa.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert!(a.equivalent(&vec![3, 2, 1, 2]));
  ///
  /// assert!(!a.equivalent(&vec![1, 3, 3]));
  /// assert!(!a.equivalent(&vec![1, 1, 2, 2, 3]));
  /// assert!(!a.equivalent(&vec![]));
  /// ```
  fn equivalent<RefIterable>(&self, elements: &RefIterable) -> bool
  where
    for<'a> &'a RefIterable: IntoIterator<Item = &'a Item>,
    Item: Eq + Hash,
  {
    let iterator = self.into_iter();
    let elements_iterator = elements.into_iter();
    let mut excluded = HashMap::<&Item, usize>::with_capacity(iterator.size_hint().0);
    let mut remaining = 0_usize;
    for item in elements_iterator {
      *excluded.entry(item).or_default() += 1;
      remaining += 1;
    }
    for item in iterator {
      if let Some(count) = excluded.get_mut(item)
        && *count > 0
      {
        *count -= 1;
        remaining = remaining.saturating_sub(1);
        continue;
      }
      return false;
    }
    remaining == 0
  }

  /// Find the position and value of the first element in this sequence satisfying a predicate.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.find_position(|&x| x == 2), Some((1, &2)));
  ///
  /// assert_eq!(a.find_position(|&x| x == 5), None);
  /// ```
  fn find_position(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<(usize, &Item)> {
    self.into_iter().enumerate().find(|(_, x)| predicate(x))
  }

  /// Compute the number of occurrences for each element in this sequence.
  ///
  /// # Example
  ///
  /// ```
  /// use std::collections::HashMap;
  ///
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.frequencies(), HashMap::from([(&1, 1), (&2, 2), (&3, 1),]));
  /// ```
  fn frequencies<'a>(&'a self) -> HashMap<&'a Item, usize>
  where
    Item: Eq + Hash + 'a,
  {
    frequencies(self.into_iter())
  }

  /// Compute the number of occurrences for each group of elements in this sequence according to
  /// the specified discriminator function.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  ///
  /// # Example
  ///
  /// ```
  /// use std::collections::HashMap;
  ///
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.frequencies_by(|x| x % 2), HashMap::from([(0, 2), (1, 2),]));
  /// ```
  fn frequencies_by<K: Eq + Hash>(&self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, usize> {
    let iterator = self.into_iter();
    let mut result = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      *result.entry(to_key(item)).or_default() += 1;
    }
    result
  }

  /// Combine all elements of this sequence into one `String`, separated by `sep`.
  ///
  /// Use the `Display` implementation of each element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.joined(", "), "1, 2, 3");
  /// assert_eq!(e.joined(", "), "");
  /// ```
  fn joined(&self, separator: &str) -> String
  where
    Item: Display,
  {
    let mut iterator = self.into_iter();
    if let Some(item) = iterator.next() {
      let mut result = String::with_capacity((separator.len() + 1) * iterator.size_hint().0);
      let _unused = write!(&mut result, "{item}");
      for item in iterator {
        result.push_str(separator);
        let _unused = write!(&mut result, "{item}");
      }
      result
    } else {
      String::new()
    }
  }

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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.position(|&x| x == 2), Some(1));
  ///
  /// assert_eq!(a.position(|&x| x == 5), None);
  /// ```
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.into_iter().position(predicate)
  }

  /// Searches for an element in this sequence, returning all its indices.
  ///
  /// `position_multi()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of this sequence. Each time one of them
  /// returns `true`, then `position_multi()` adds the element index to its result.
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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.position_multi(|&x| x % 2 == 0), vec![1, 2]);
  ///
  /// assert_eq!(a.position_multi(|&x| x > 3), vec![]);
  /// ```
  fn position_multi(&self, mut predicate: impl FnMut(&Item) -> bool) -> Vec<usize> {
    self.into_iter().enumerate().filter(|(_, item)| predicate(item)).map(|(index, _)| index).collect()
  }

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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.position_of(&2), Some(1));
  ///
  /// assert_eq!(a.position_of(&5), None);
  /// ```
  #[inline]
  fn position_of(&self, element: &Item) -> Option<usize>
  where
    Item: PartialEq,
  {
    self.position(|x| x == element)
  }

  /// Searches for an element in this sequence, returning all its indices.
  ///
  /// `position_of_multi()` compares each element of this sequence with the specified value,
  /// and each time one of them matches, then `position_of_multi()` adds the element index
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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.position_of_multi(&2), vec![1, 2]);
  ///
  /// assert_eq!(a.position_of_multi(&5), vec![]);
  /// ```
  #[inline]
  fn position_of_multi(&self, element: &Item) -> Vec<usize>
  where
    Item: PartialEq,
  {
    self.position_multi(|x| x == element)
  }

  /// Searches for a subsequence in this sequence, returning its index.
  ///
  /// After finding a starting element of the specified sequence in this sequence,
  /// `position_sequence()` compares each element of this sequence with the specified value,
  /// and if all of them match, then `position_sequence()` returns [`Some(start_index)`].
  /// If any of the elements do not match, it returns [`None`].
  ///
  /// `position_sequence()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a matching sequence.
  ///
  /// Returns `Some(0)` if the specified sequence is empty.
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
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.position_sequence(&vec![2, 2]), Some(1));
  /// assert_eq!(a.position_sequence(&vec![]), Some(0));
  ///
  /// assert_eq!(a.position_sequence(&vec![1, 3]), None);
  /// ```
  fn position_sequence<RefIterable>(&self, elements: &RefIterable) -> Option<usize>
  where
    for<'a> &'a RefIterable: IntoIterator<Item = &'a Item>,
    Item: PartialEq,
  {
    let mut iterator = self.into_iter();
    let mut elements_iterator = elements.into_iter();
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
  /// assert_eq!(a.rfind(|&x| x % 2 == 1), Some(&3));
  ///
  /// assert_eq!(a.rfind(|&x| x == 5), None);
  /// ```
  fn rfind(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  /// Reduces this sequence's elements to a single, final value, starting from the back.
  ///
  /// This is the reverse version of [`Iterator::fold()`]: it takes elements
  /// starting from the back of this sequence.
  ///
  /// `rfold_ref()` takes two arguments: an initial value, and a closure with two
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
  /// Folding is useful whenever you have a collection of something and want
  /// to produce a single value from it.
  ///
  /// This is a non-consuming variant of [`rfold()`].
  ///
  /// Note: `rfold_ref()` combines elements in a *right-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *left-associative* version of `rfold_ref()`, see [`fold_ref()`].
  ///
  /// [`rfold()`]: crate::SequenceTo::rfold
  /// [`fold_ref()`]: crate::Collection::fold_ref
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
  /// // the sum of all the elements in a
  /// assert_eq!(a.rfold_ref(0, |acc, x| acc + x), 6);
  /// ```
  ///
  /// This example demonstrates the right-associative nature of `rfold()`:
  /// it builds a string, starting with an initial value
  /// and continuing with each element from the back until the front:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// assert_eq!(
  ///   a.rfold_ref(zero, |acc, x| { format!("({x} + {acc})") }),
  ///   "(1 + (2 + (3 + (4 + (5 + 0)))))"
  /// );
  /// ```
  fn rfold_ref<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B;

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
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rposition(|&x| x % 2 == 1), Some(2));
  ///
  /// assert_eq!(a.rposition(|&x| x == 5), None);
  /// ```
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;
}

pub(crate) fn common_suffix_length<'a, Item: PartialEq + 'a, I: DoubleEndedIterator<Item = &'a Item>>(
  reversed_iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item, Iterator<'a> = I>,
) -> usize {
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
  let items = iterator.collect::<HashSet<_>>();
  items.len()
}
