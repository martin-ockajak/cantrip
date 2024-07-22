/// Vector transform operations.
///
/// Methods have the following properties:
///
/// - Creates a new collection
/// 
pub trait TransformVec<Item> {
  /// Creates a new vector from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_vec()`].
  ///
  /// [`to_vec()`]: TransformVec::to_vec
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.into_vec(), vec![1, 2, 3]);
  /// ```
  #[inline]
  fn into_vec(self) -> Vec<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new vector from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_vec()`].
  ///
  /// [`into_vec()`]: TransformVec::into_vec
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = LinkedList::from([1, 2, 3]);
  ///
  /// assert_eq!(a.to_vec(), vec![1, 2, 3]);
  /// ```
  fn to_vec(&self) -> Vec<Item>
  where
    Item: Clone;
}
