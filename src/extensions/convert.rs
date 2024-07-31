use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

/// Conversion operations.
///
/// Methods have the following properties:
///
/// - Consumes the collection or its elements
/// - Creates a new collection
///
pub trait Convert<Item> {
  /// Creates a new ordered map from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::BTreeMap;
  ///
  /// let a = vec![(1, 1), (2, 2), (3, 3)];
  ///
  /// assert_eq!(a.to_bmap(), BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  #[inline]
  fn to_bmap<K, V>(self) -> BTreeMap<K, V>
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
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::BTreeSet;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.to_bset(), BTreeSet::from([1, 2, 3]));
  /// ```
  #[inline]
  fn to_bset(self) -> BTreeSet<Item>
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
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::VecDeque;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.to_deque(), VecDeque::from([1, 2, 3]));
  /// ```
  #[inline]
  fn to_deque(self) -> VecDeque<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new priority queue from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// [`to_heap()`]: crate::Convert::to_heap
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
  ///   a.to_heap().into_iter().collect::<HashSet<_>>(),
  ///   BinaryHeap::from([1, 2, 3]).into_iter().collect()
  /// );
  /// ```
  #[inline]
  fn to_heap(self) -> BinaryHeap<Item>
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
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.to_list(), LinkedList::from([1, 2, 3]));
  /// ```
  #[inline]
  fn to_list(self) -> LinkedList<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new hash map from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![(1, 1), (2, 2), (3, 3)];
  ///
  /// assert_eq!(a.to_map(), HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  #[inline]
  fn to_map<K, V>(self) -> HashMap<K, V>
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
  /// [`to_set()`]: crate::Convert::to_set
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashSet;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.to_set(), HashSet::from([1, 2, 3]));
  /// ```
  #[inline]
  fn to_set(self) -> HashSet<Item>
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new vector from the elements of this collection.
  ///
  /// This is an equivalent of [`Iterator::collect`].
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
  #[inline]
  fn to_vec(self) -> Vec<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }
}

impl<Item, I> Convert<Item> for I where I: IntoIterator<Item = Item> {}
