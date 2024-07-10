use std::collections::HashMap;
use std::hash::Hash;
use crate::Iterable;

/// Ordered collection operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent an ordered collection
///
pub trait Ordered<Item> {
  /// Tests if a collection contains all elements of another collection exactly
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

  /// Tests if a collection contains all elements of another collection
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

  /// Searches for an element in a collection, returning its index.
  ///
  /// `position()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and if one of them
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
  /// This function might panic if the collection has more than `usize::MAX`
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

  /// Searches for an element in a collection, returning all its indices.
  ///
  /// `positions()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, each time one of them
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
  /// This function might panic if the collection has more than `usize::MAX`
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

  /// Searches for an element in a collection, returning its index.
  ///
  /// `position_of()` compares each element of the collection with the specified value,
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
  /// This function might panic if the collection has more than `usize::MAX`
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

  /// Searches for an element in a collection, returning all its indices.
  ///
  /// `positions_of()` compares each element of the collection with the specified value,
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
  /// This function might panic if the collection has more than `usize::MAX`
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
  fn positions_of(&self, element: &Item) -> Vec<usize>
  where
    Item: PartialEq,
  {
    self.positions(|x| x == element)
  }
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

#[inline]
pub(crate) fn positions<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> Vec<usize>
where
  Item: 'a,
{
  iterator.enumerate().filter(|(_, item)| predicate(item)).map(|(index, _)| index).collect()
}
