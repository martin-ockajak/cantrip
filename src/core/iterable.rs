use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// Conversion into an [`Iterator`] over references.
///
/// By implementing `Iterable` for a type, you define how it will be converted to
/// an iterator over references. This is common for types which describe a
/// collection of some kind.
///
/// This is a reference-based equivalent of standard [`IntoIterator`] providing
/// the `iterator()` method, which is a generic equivalent of various `iter()`
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
/// Implementing `Iterable` for your type:
///
/// It is common to use `Iterable` as a trait bound. This allows the input collection type
/// to change, so long as it is still an iterator. Additional bounds can be specified by
/// restricting on `Item`:
///
/// ```
/// use crate::cantrip::*;
///
/// fn collect_as_strings<'a, T>(collection: &'a impl Iterable<Item<'a> = &'a T>) -> Vec<String>
/// where T: 'a + std::fmt::Debug {
///   collection.iterator().map(|item| format!("{item:?}")).collect()
/// }
/// ```
#[allow(clippy::elidable_lifetime_names)]
pub trait Iterable {
  /// The type of the elements being iterated over.
  type Item<'collection>
  where
    Self: 'collection;

  /// The resulting iterator type.
  type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
  where
    Self: 'collection;

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

#[allow(clippy::elidable_lifetime_names)]
impl<Item> Iterable for Option<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = core::option::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Item, E> Iterable for Result<Item, E> {
  type Item<'c>
    = &'c Item
  where
    E: 'c,
    Item: 'c;
  type Iterator<'c>
    = core::result::Iter<'c, Item>
  where
    E: 'c,
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Item> Iterable for [Item] {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = core::slice::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

impl<Item> Iterable for Vec<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = core::slice::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  #[allow(clippy::elidable_lifetime_names)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

impl<Item> Iterable for LinkedList<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = std::collections::linked_list::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  #[allow(clippy::elidable_lifetime_names)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

impl<Item> Iterable for VecDeque<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = std::collections::vec_deque::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  #[allow(clippy::elidable_lifetime_names)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::implicit_hasher)]
#[allow(clippy::elidable_lifetime_names)]
impl<Item> Iterable for HashSet<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = std::collections::hash_set::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Item> Iterable for BTreeSet<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = std::collections::btree_set::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Item> Iterable for BinaryHeap<Item> {
  type Item<'c>
    = &'c Item
  where
    Item: 'c;
  type Iterator<'c>
    = std::collections::binary_heap::Iter<'c, Item>
  where
    Item: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::implicit_hasher)]
#[allow(clippy::elidable_lifetime_names)]
impl<Key, Value> Iterable for HashMap<Key, Value> {
  type Item<'c>
    = (&'c Key, &'c Value)
  where
    Key: 'c,
    Value: 'c;
  type Iterator<'c>
    = std::collections::hash_map::Iter<'c, Key, Value>
  where
    Key: 'c,
    Value: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    self.iter()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Key, Value> Iterable for std::collections::hash_map::Keys<'_, Key, Value> {
  type Item<'c>
    = &'c Key
  where
    Self: 'c;
  type Iterator<'c>
    = HashMapKeysIterator<'c, Key, Value>
  where
    Self: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashMapKeysIterator { iterator: self.clone() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct HashMapKeysIterator<'c, Key, Value> {
  pub(crate) iterator: std::collections::hash_map::Keys<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for HashMapKeysIterator<'c, Key, Value> {
  type Item = &'c Key;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Key, Value> Iterable for std::collections::hash_map::Values<'_, Key, Value> {
  type Item<'c>
    = &'c Value
  where
    Self: 'c;
  type Iterator<'c>
    = HashMapValuesIterator<'c, Key, Value>
  where
    Self: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    HashMapValuesIterator { iterator: self.clone() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct HashMapValuesIterator<'c, Key, Value> {
  pub(crate) iterator: std::collections::hash_map::Values<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for HashMapValuesIterator<'c, Key, Value> {
  type Item = &'c Value;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

#[allow(clippy::elidable_lifetime_names)]
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

#[allow(clippy::elidable_lifetime_names)]
impl<Key, Value> Iterable for std::collections::btree_map::Keys<'_, Key, Value> {
  type Item<'c>
    = &'c Key
  where
    Self: 'c;
  type Iterator<'c>
    = BTreeMapKeysIterator<'c, Key, Value>
  where
    Self: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeMapKeysIterator { iterator: self.clone() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct BTreeMapKeysIterator<'c, Key, Value> {
  pub(crate) iterator: std::collections::btree_map::Keys<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for BTreeMapKeysIterator<'c, Key, Value> {
  type Item = &'c Key;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}

#[allow(clippy::elidable_lifetime_names)]
impl<Key, Value> Iterable for std::collections::btree_map::Values<'_, Key, Value> {
  type Item<'c>
    = &'c Value
  where
    Self: 'c;
  type Iterator<'c>
    = BTreeMapValuesIterator<'c, Key, Value>
  where
    Self: 'c;

  #[allow(clippy::needless_lifetimes)]
  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    BTreeMapValuesIterator { iterator: self.clone() }
  }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct BTreeMapValuesIterator<'c, Key, Value> {
  pub(crate) iterator: std::collections::btree_map::Values<'c, Key, Value>,
}

impl<'c, Key, Value> Iterator for BTreeMapValuesIterator<'c, Key, Value> {
  type Item = &'c Value;

  fn next(&mut self) -> Option<Self::Item> {
    self.iterator.next()
  }
}
