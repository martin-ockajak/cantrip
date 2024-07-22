/// Vector transform operations.
///
/// Methods have the following properties:
///
/// - Creates a new collection
/// 
pub trait TransformVecTo<Item> {
  /// Creates a new vector from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_vec()`].
  ///
  /// [`to_vec()`]: crate::TransformVec::to_vec
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
}

impl <Item, I> TransformVecTo<Item> for I where I: Iterator<Item = Item> {
  #[inline]
  fn into_vec(self) -> Vec<Item>
  where
    Self: IntoIterator<Item = Item> + Sized
  {
    self.collect()
  }
}
