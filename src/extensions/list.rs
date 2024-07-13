use crate::extensions::util::unfold::unfold;

/// List operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a list
/// - Does not consume the list or its elements
/// - Does not create a new list
///
pub trait List<Item> {
  /// Returns the first element of this sequence, or `None` if it is empty.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.first(), Some(&1));
  /// ```
  fn first(&self) -> Option<&Item>;

  /// Returns the last element of this sequence, or `None` if it is empty.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.last(), Some(&3));
  /// ```
  fn last(&self) -> Option<&Item>;

  /// Creates a new collection by repeating this collection specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.repeat(3), LinkedList::from([1, 2, 3, 1, 2, 3, 1, 2, 3]));
  /// ```
  fn repeat(self, n: usize) -> Self
  where
    Item: Clone;
}

pub(crate) fn repeat<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>, n: usize,
) -> Collection {
  let collection = iterator.collect::<Vec<&Item>>();
  let mut values = collection.iter().cycle();
  let mut remaining = collection.len() * n;
  unfold(|| {
    if remaining == 0 {
      return None;
    }
    let result = values.next().map(|&x| x.clone());
    remaining -= 1;
    result
  })
  .collect()
}
