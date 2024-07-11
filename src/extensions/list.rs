/// List operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a list
/// - May consume the collection and its elements
/// - May create a new collection
///
pub trait List<Item> {
  /// Returns the first element of the slice, or `None` if it is empty.
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

  /// Returns the last element of the slice, or `None` if it is empty.
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
  #[inline]
  fn repeat(self, n: usize) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let values = self.into_iter().collect::<Vec<Item>>();
    let size = values.len() * n;
    values.into_iter().cycle().take(size).collect()
  }
}
