/// Ordered collection operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent an ordered collection
///
pub trait Ordered<Item> {
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

#[inline]
pub(crate) fn positions<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> Vec<usize>
where
  Item: 'a,
{
  iterator.enumerate().filter(|(_, item)| predicate(item)).map(|(index, _)| index).collect()
}
