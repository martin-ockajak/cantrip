use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

/// Non-consuming transform operations.
///
/// Methods have the following properties:
///
/// - Requires collection elements to implement [`Clone`]
/// - Does not consume the collection or its elements
/// - Creates a new collection
///
pub trait TransformTo<Item> {
  /// Creates a new ordered map from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_bmap()`].
  ///
  /// [`to_bmap()`]: crate::Transform::to_bmap
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::BTreeMap;
  ///
  /// let a = vec![(1, 1), (2, 2), (3, 3)];
  ///
  /// assert_eq!(a.into_bmap(), BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  #[inline]
  fn into_bmap<K, V>(self) -> BTreeMap<K, V>
  where
    K: Ord,
    Self: IntoIterator<Item = (K, V)> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new ordered set from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_bset()`].
  ///
  /// [`to_bset()`]: crate::Transform::to_bset
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::BTreeSet;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.into_bset(), BTreeSet::from([1, 2, 3]));
  /// ```
  #[inline]
  fn into_bset(self) -> BTreeSet<Item>
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new double-ended queue from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_deque()`].
  ///
  /// [`to_deque()`]: crate::Transform::to_deque
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::VecDeque;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.into_deque(), VecDeque::from([1, 2, 3]));
  /// ```
  #[inline]
  fn into_deque(self) -> VecDeque<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new priority queue from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_heap()`].
  ///
  /// [`to_heap()`]: crate::Transform::to_heap
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BinaryHeap, HashSet};
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.into_heap().into_iter().collect::<HashSet<_>>(),
  ///   BinaryHeap::from([1, 2, 3]).into_iter().collect()
  /// );
  /// ```
  #[inline]
  fn into_heap(self) -> BinaryHeap<Item>
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new doubly-linked list from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_list()`].
  ///
  /// [`to_list()`]: crate::Transform::to_list
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.into_list(), LinkedList::from([1, 2, 3]));
  /// ```
  #[inline]
  fn into_list(self) -> LinkedList<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new hash map from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_map()`].
  ///
  /// [`to_map()`]: crate::Transform::to_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![(1, 1), (2, 2), (3, 3)];
  ///
  /// assert_eq!(a.into_map(), HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  #[inline]
  fn into_map<K, V>(self) -> HashMap<K, V>
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new hash set from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_set()`].
  ///
  /// [`to_set()`]: crate::Transform::to_set
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashSet;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.into_set(), HashSet::from([1, 2, 3]));
  /// ```
  #[inline]
  fn into_set(self) -> HashSet<Item>
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }
}

impl<Item, I> TransformTo<Item> for I where I: IntoIterator<Item = Item> {}
