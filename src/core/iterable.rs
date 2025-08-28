use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// Conversion into an [`Iterator`] over references.
///
/// By implementing `Iterable` for a type, you define how it will be converted to
/// an iterator over references. This is common for types which describe a
/// collection of some kind.
///
/// This is a reference-based equivalent of standard [`IntoIterator`] providing
/// the `iterator()` method which is a generic equivalent of various `iter()`
/// methods for various standard collection types.
///
/// See also: [`FromIterator`].
///
/// [`IntoIterator`]: IntoIterator
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use crate::cantrip::*;
///
/// let a = vec![1, 2, 3];
/// let mut iter = a.iterator();
///
/// assert_eq!(Some(&1), iter.next());
/// assert_eq!(Some(&2), iter.next());
/// assert_eq!(Some(&3), iter.next());
/// assert_eq!(None, iter.next());
/// ```
/// Implementing `IntoIterator` for your type:
///
/// It is common to use `Iterable` as a trait bound. This allows the input collection type
/// to change, so long as it is still an iterator. Additional bounds can be specified by
/// restricting on `Item`:
///
/// ```
/// use crate::cantrip::*;
///
/// fn collect_as_strings<'a, T>(collection: &'a impl Iterable<Item<'a> = &'a T>) -> Vec<String>
/// where T: IntoIterator + 'a + std::fmt::Debug {
///   collection.iterator().map(|item| format!("{item:?}")).collect()
/// }
/// ```
pub trait Iterable {
  /// The type of the elements being iterated over.
  type Item<'collection>
  where Self: 'collection;

  /// Which kind of iterator are we turning this into?
  type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
  where Self: 'collection;

  /// Creates an iterator from a value.
  ///
  /// See the [`Iterable`] documentation for more.
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let mut iter = a.iterator();
  ///
  /// assert_eq!(Some(&1), iter.next());
  /// assert_eq!(Some(&2), iter.next());
  /// assert_eq!(Some(&3), iter.next());
  /// assert_eq!(None, iter.next());
  /// ```
  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c>;
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct OptionIterator<'c, T> {
  pub(crate) iterator: core::option::Iter<'c, T>,
}

impl<'c, T> Iterator for OptionIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for Option<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = OptionIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    OptionIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ResultIterator<'c, T> {
  pub(crate) iterator: core::result::Iter<'c, T>,
}

impl<'c, T> Iterator for ResultIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item, E> Iterable for Result<Item, E> {
  type Item<'c>
    = &'c Item
  where
    E: 'c,
    Item: 'c;
  type Iterator<'c>
    = ResultIterator<'c, Item>
  where
    E: 'c,
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    ResultIterator { iterator: self.iter() }
  }
}

impl<Item> Iterable for [Item] {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = SliceIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    SliceIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct SliceIterator<'c, T> {
  pub(crate) iterator: core::slice::Iter<'c, T>,
}

impl<'c, T> Iterator for SliceIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

#[allow(single_use_lifetimes)]
impl<'c, T> DoubleEndedIterator for SliceIterator<'c, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.iterator.next_back()
  }
}

impl<Item> Iterable for Vec<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = SliceIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    SliceIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct LinkedListIterator<'c, T> {
  pub(crate) iterator: std::collections::linked_list::Iter<'c, T>,
}

impl<'c, T> Iterator for LinkedListIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

#[allow(single_use_lifetimes)]
impl<'c, T> DoubleEndedIterator for LinkedListIterator<'c, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.iterator.next_back()
  }
}

impl<Item> Iterable for LinkedList<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = LinkedListIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    LinkedListIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct VecDequeIterator<'c, T> {
  pub(crate) iterator: std::collections::vec_deque::Iter<'c, T>,
}

impl<'c, T> Iterator for VecDequeIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

#[allow(single_use_lifetimes)]
impl<'c, T> DoubleEndedIterator for VecDequeIterator<'c, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.iterator.next_back()
  }
}

impl<Item> Iterable for VecDeque<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = VecDequeIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    VecDequeIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct HashSetIterator<'c, T> {
  pub(crate) iterator: std::collections::hash_set::Iter<'c, T>,
}

impl<'c, T> Iterator for HashSetIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for HashSet<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = HashSetIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashSetIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct BTreeSetIterator<'c, T> {
  pub(crate) iterator: std::collections::btree_set::Iter<'c, T>,
}

impl<'c, T> Iterator for BTreeSetIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for BTreeSet<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = BTreeSetIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeSetIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct BinaryHeapIterator<'c, T> {
  pub(crate) iterator: std::collections::binary_heap::Iter<'c, T>,
}

impl<'c, T> Iterator for BinaryHeapIterator<'c, T> {
  type Item = &'c T;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Item> Iterable for BinaryHeap<Item> {
  type Item<'c>
    = &'c Item
  where Item: 'c;
  type Iterator<'c>
    = BinaryHeapIterator<'c, Item>
  where Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BinaryHeapIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct HashMapIterator<'c, Key, Value> {
  pub(crate) iterator: std::collections::hash_map::Iter<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for HashMapIterator<'c, Key, Value> {
  type Item = (&'c Key, &'c Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Key, Value> Iterable for HashMap<Key, Value> {
  type Item<'c>
    = (&'c Key, &'c Value)
  where
    Key: 'c,
    Value: 'c;
  type Iterator<'c>
    = HashMapIterator<'c, Key, Value>
  where
    Key: 'c,
    Value: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashMapIterator { iterator: self.iter() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct BTreeMapIterator<'c, Key, Value> {
  pub(crate) iterator: std::collections::btree_map::Iter<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for BTreeMapIterator<'c, Key, Value> {
  type Item = (&'c Key, &'c Value);

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

impl<Key, Value> Iterable for BTreeMap<Key, Value> {
  type Item<'c>
    = (&'c Key, &'c Value)
  where
    Key: 'c,
    Value: 'c;
  type Iterator<'c>
    = BTreeMapIterator<'c, Key, Value>
  where
    Key: 'c,
    Value: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeMapIterator { iterator: self.iter() }
  }
}
