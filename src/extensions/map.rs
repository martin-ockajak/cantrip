#![allow(missing_docs)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use crate::extensions::iterable::Iterable;

/// Map operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a map
/// - May consume the collection and its elements
/// - May create a new collection
///
pub trait Map<Key, Value> {
  type This<K, V>;

  // FIXME - add documentation
  // FIXME - implement these methods
  // equivalent
  // includes

  /// Creates a map by adding a key/value pair to the original map.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let source = HashMap::from([
  /// #  (1, "a"),
  /// #  (2, "b"),
  /// #  (3, "c"),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.add(4, "d"), HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  ///   (4, "d"),
  /// ]));
  /// # let a = source.clone();
  /// assert_eq!(a.add(1, "d"), HashMap::from([
  ///   (1, "d"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]));
  /// ```
  #[inline]
  fn add(self, key: Key, value: Value) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  /// Creates a map by appending all key/value pairs from another collection to
  /// the original map.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let source = HashMap::from([
  /// #  (1, "a"),
  /// #  (2, "b"),
  /// #  (3, "c"),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.add_all(vec![(4, "d"), (5, "e")]), HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  ///   (4, "d"),
  ///   (5, "e"),
  /// ]));
  /// # let a = source.clone();
  /// assert_eq!(a.add_all(vec![(1, "d"), (5, "e")]), HashMap::from([
  ///   (1, "d"),
  ///   (2, "b"),
  ///   (3, "c"),
  ///   (5, "e"),
  /// ]));
  /// ```
  #[inline]
  fn add_all(self, iterable: impl IntoIterator<Item = (Key, Value)>) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iterable).collect()
  }

  /// Tests if every key/value pair in the map matches a predicate.
  ///
  /// `all()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each key/value pair in the map, and if they all return
  /// `true`, then so does `all()`. If any of them return `false`, it
  /// returns `false`.
  ///
  /// `all()` is short-circuiting; in other words, it will stop processing
  /// as soon as it finds a `false`, given that no matter what else happens,
  /// the result will also be `false`.
  ///
  /// An empty map returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.all(|(&k, &v)| k > 0 && v.len() > 0));
  /// assert!(e.all(|(&k, _)| k > 0));
  ///
  /// assert!(!a.all(|(&k, _)| k > 2));
  /// ```
  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  /// Tests if all values of the map are equal.
  ///
  /// `all_equal()` returns `true` if all values of the map are equal
  /// and `false` if a pair of unequal values exist.
  ///
  /// An empty map returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "a"),
  ///   (3, "a"),
  /// ]);
  /// let b = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.all_values_equal());
  /// assert!(e.all_values_equal());
  ///
  /// assert!(!b.all_values_equal());
  /// ```
  fn all_values_equal(&self) -> bool
  where
    Value: PartialEq;

  /// Tests if all values of the map are unique.
  ///
  /// `all_equal()` returns `true` if all values of the map are unique
  /// and `false` if a pair of equal values exist.
  ///
  /// An empty map returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let b = HashMap::from([
  ///   (1, "a"),
  ///   (2, "a"),
  ///   (3, "a"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.all_values_unique());
  /// assert!(e.all_values_unique());
  ///
  /// assert!(!b.all_values_unique());
  /// ```
  fn all_values_unique(&self) -> bool
  where
    Value: Eq + Hash;

  /// Tests if any key/value pair in the map matches a predicate.
  ///
  /// `any()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each key/value pair in the map, and if any of them return
  /// `true`, then so does `any()`. If they all return `false`, it
  /// returns `false`.
  ///
  /// `any()` is short-circuiting; in other words, it will stop processing
  /// as soon as it finds a `true`, given that no matter what else happens,
  /// the result will also be `true`.
  ///
  /// An empty map returns `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.any(|(&k, &v)| k > 0 && v.len() > 0));
  ///
  /// assert!(!a.any(|(&k, _)| k > 5));
  /// assert!(!e.any(|(&k, _)| k > 0));
  /// ```
  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  /// Counts key/value pairs of a map that satisfy a predicate.
  ///
  /// `count_by()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each key/value pair in the map, and counts those which
  /// return `true`, disregarding those which return `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.count_by(|(&k, &v)| k == 2 && v == "b"), 1);
  /// assert_eq!(a.count_by(|(&k, _)| k == 5), 0);
  /// ```
  fn count_by(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> usize;

  #[inline]
  fn delete(self, key: &Key) -> Self
  where
    Key: PartialEq,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter_map(|(k, v)| if &k != key { Some((k, v)) } else { None }).collect()
  }

  #[inline]
  fn delete_all<'a>(self, iterable: &'a impl Iterable<Item<'a> = &'a Key>) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let removed: HashSet<&Key> = HashSet::from_iter(iterable.iterator());
    self.into_iter().filter(|(k, _)| !removed.contains(k)).collect()
  }

  #[inline]
  fn fill(key: Key, value: Value, size: usize) -> Self
  where
    Key: Clone,
    Value: Clone,
    Self: FromIterator<(Key, Value)>,
  {
    iter::repeat((key, value)).take(size).collect()
  }

  #[inline]
  fn fill_with(mut value: impl FnMut() -> (Key, Value), size: usize) -> Self
  where
    Key: Clone,
    Value: Clone,
    Self: FromIterator<(Key, Value)>,
  {
    iter::repeat(value()).take(size).collect()
  }

  #[inline]
  fn filter(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(k, v)| predicate((k, v))).collect()
  }

  #[inline]
  fn filter_keys(self, mut predicate: impl FnMut(&Key) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(k, _)| predicate(k)).collect()
  }

  #[inline]
  fn filter_values(self, mut predicate: impl FnMut(&Value) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  fn filter_map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  #[inline]
  fn filter_map_to<L, W>(self, function: impl FnMut((Key, Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().filter_map(function).collect()
  }

  fn find(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)>;

  fn find_map<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  #[inline]
  fn find_map_to<B>(self, function: impl FnMut((Key, Value)) -> Option<B>) -> Option<B>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().find_map(function)
  }

  fn flat_map<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>;

  #[inline]
  fn flat_map_to<L, W, R>(self, function: impl FnMut((Key, Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().flat_map(function).collect()
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B;

  #[inline]
  fn intersect<'a>(self, iterable: &'a impl Iterable<Item<'a> = &'a Key>) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let retained: HashSet<&Key> = HashSet::from_iter(iterable.iterator());
    self.into_iter().filter(|(k, _)| retained.contains(k)).collect()
  }

  fn join_items(&self, separator: &str) -> String
  where
    Key: Display,
    Value: Display;

  fn map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  #[inline]
  fn map_to<L, W>(self, function: impl FnMut((Key, Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().map(function).collect()
  }

  #[inline]
  fn map_keys<L: Eq + Hash>(self, mut function: impl FnMut(&Key) -> L) -> Self::This<L, Value>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, Value>: FromIterator<(L, Value)>,
  {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  #[inline]
  fn map_values<W: Eq + Hash>(self, mut function: impl FnMut(&Value) -> W) -> Self::This<Key, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<Key, W>: FromIterator<(Key, W)>,
  {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }

  fn max_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  fn max_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>;

  #[inline]
  fn max_item(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.max_by(|x, y| x.cmp(&y))
  }

  fn min_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  fn min_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>;

  #[inline]
  fn min_item(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.min_by(|(k1, v1), (k2, v2)| (k1, v1).cmp(&(k2, v2)))
  }

  fn minmax_by(
    &self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering,
  ) -> Option<((&Key, &Value), (&Key, &Value))>;

  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<((&Key, &Value), (&Key, &Value))>;

  #[inline]
  fn minmax_item(&self) -> Option<((&Key, &Value), (&Key, &Value))>
  where
    Key: Ord,
    Value: Ord,
  {
    self.minmax_by(|(x1, x2), (y1, y2)| (x1, x2).cmp(&(y1, y2)))
  }

  #[inline]
  fn partition(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> (Self, Self)
  where
    Self: IntoIterator<Item = (Key, Value)> + Default + Extend<(Key, Value)>,
  {
    self.into_iter().partition(|(k, v)| predicate((k, v)))
  }

  fn partition_map<L1, W1, L2, W2>(
    &self, function: impl FnMut((&Key, &Value)) -> Result<(L1, W1), (L2, W2)>,
  ) -> (Self::This<L1, W1>, Self::This<L2, W2>)
  where
    Self::This<L1, W1>: Default + Extend<(L1, W1)>,
    Self::This<L2, W2>: Default + Extend<(L2, W2)>;

  #[inline]
  fn product_keys<S>(self) -> Key
  where
    Key: Product,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).product()
  }

  #[inline]
  fn product_values<S>(self) -> Value
  where
    Value: Product,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).product()
  }

  fn reduce(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)>;

  /// Creates a map from the original map by replacing the specified key
  /// and its value with a different key/value pair.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.replace(&3, 4, "d"), HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (4, "d"),
  /// ]));
  /// ```
  #[inline]
  fn replace(self, value: &Key, replacement_key: Key, replacement_value: Value) -> Self
  where
    Key: PartialEq,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let mut replaced = Some((replacement_key, replacement_value));
    self
      .into_iter()
      .map(|(key, val)| if &key == value { replaced.take().unwrap_or((key, val)) } else { (key, val) })
      .collect()
  }

  /// Creates a map from the original map by replacing the given occurrences of elements
  /// found in another collection with elements of a replacement collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.replace_all(&vec![2, 3], vec![(4, "d"), (5, "e")]),HashMap::from([
  ///   (1, "a"),
  ///   (4, "d"),
  ///   (5, "e"),
  /// ]));
  /// ```
  #[inline]
  fn replace_all<'a>(
    self, elements: &'a impl Iterable<Item<'a> = &'a Key>, replacement: impl IntoIterator<Item = (Key, Value)>,
  ) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let iterator = elements.iterator();
    let removed: HashSet<&Key> = HashSet::from_iter(iterator);
    self.into_iter().filter(|x| !removed.contains(&x.0)).chain(replacement).collect()
  }

  fn scan<S, L, W>(
    self, initial_state: S, function: impl FnMut(&mut S, (&Key, &Value)) -> Option<(L, W)>,
  ) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  #[inline]
  fn scan_to<S, L, W>(
    self, initial_state: S, function: impl FnMut(&mut S, (Key, Value)) -> Option<(L, W)>,
  ) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().scan(initial_state, function).collect()
  }

  /// Tests if all the key of a map can be found in another collection.
  ///
  /// Returns `true` if this map is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.subset(&vec![2, 1, 3, 4]));
  /// assert!(e.subset(&vec![2, 1, 3, 4]));
  ///
  /// assert!(!a.subset(&vec![2, 1]));
  /// assert!(!a.subset(&vec![]));
  /// ```
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

  /// Tests if all the elements of another collection can be found among the keys of this map.
  ///
  /// Returns `true` if the other collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.superset(&vec![2, 1]));
  /// assert!(a.superset(&vec![]));
  ///
  /// assert!(!a.superset(&vec![2, 1, 3, 4]));
  /// assert!(!e.superset(&vec![2, 1]));
  /// ```
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

  #[inline]
  fn sum_keys(self) -> Key
  where
    Key: Sum,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).sum()
  }

  #[inline]
  fn sum_values(self) -> Value
  where
    Value: Sum,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).sum()
  }

  #[inline]
  fn unit(key: Key, value: Value) -> Self
  where
    Self: FromIterator<(Key, Value)>,
  {
    iter::once((key, value)).collect()
  }
}

#[inline]
pub(crate) fn all_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, predicate: impl FnMut((&K, &V)) -> bool,
) -> bool {
  iterator.all(predicate)
}

#[inline]
pub(crate) fn any_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, predicate: impl FnMut((&K, &V)) -> bool,
) -> bool {
  iterator.any(predicate)
}

#[inline]
pub(crate) fn count_by_pairs<'a, K: 'a, V: 'a>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut predicate: impl FnMut((&K, &V)) -> bool,
) -> usize {
  iterator.filter(|&x| predicate(x)).count()
}

#[inline]
pub(crate) fn filter_map_pairs<'a, K: 'a, V: 'a, L, W, Result: FromIterator<(L, W)>>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut(&(&K, &V)) -> Option<(L, W)>,
) -> Result {
  iterator.filter_map(|x| function(&x)).collect()
}

#[inline]
pub(crate) fn find_map_pairs<'a, K: 'a, V: 'a, B>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut(&(&K, &V)) -> Option<B>,
) -> Option<B> {
  iterator.find_map(|x| function(&x))
}

#[inline]
pub(crate) fn flat_map_pairs<'a, K: 'a, V: 'a, L, W, R: IntoIterator<Item = (L, W)>, Result: FromIterator<(L, W)>>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut(&(&K, &V)) -> R,
) -> Result {
  iterator.flat_map(|x| function(&x)).collect()
}

pub(crate) fn join_items_pairs<'a, K: Display + 'a, V: Display + 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, separator: &str,
) -> String {
  match iterator.next() {
    Some((key, value)) => {
      let mut result = String::with_capacity(separator.len() * iterator.size_hint().0);
      write!(&mut result, "{},{}", key, value).unwrap();
      for (key, value) in iterator {
        result.push_str(separator);
        write!(&mut result, "{},{}", key, value).unwrap();
      }
      result.shrink_to_fit();
      result
    }
    None => String::new(),
  }
}

#[inline]
pub(crate) fn map_pairs<'a, K: 'a, V: 'a, L, W, Result: FromIterator<(L, W)>>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut(&(&K, &V)) -> (L, W),
) -> Result {
  iterator.map(|x| function(&x)).collect()
}

pub(crate) fn minmax_by_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut compare: impl FnMut((&K, &V), (&K, &V)) -> Ordering,
) -> Option<((&'a K, &'a V), (&'a K, &'a V))> {
  match iterator.next() {
    Some(item) => {
      let mut min = item;
      let mut max = min;
      for item in iterator {
        if compare(item, min) == Ordering::Less {
          min = item;
        }
        if compare(item, max) == Ordering::Greater {
          max = item;
        }
      }
      Some((min, max))
    }
    None => None,
  }
}

#[inline]
pub(crate) fn minmax_by_key_pairs<'a, K: 'a, V: 'a, E: Ord>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut to_key: impl FnMut((&K, &V)) -> E,
) -> Option<((&'a K, &'a V), (&'a K, &'a V))> {
  minmax_by_pairs(iterator, |x, y| to_key(x).cmp(&to_key(y)))
}

pub(crate) fn partition_map_pairs<'a, K, V, L1, W1, L2, W2, Left, Right>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V)) -> Result<(L1, W1), (L2, W2)>,
) -> (Left, Right)
where
  K: 'a,
  V: 'a,
  Left: Default + Extend<(L1, W1)>,
  Right: Default + Extend<(L2, W2)>,
{
  let mut result_left = Left::default();
  let mut result_right = Right::default();
  for item in iterator {
    match function(item) {
      Ok(value) => result_left.extend(iter::once(value)),
      Err(value) => result_right.extend(iter::once(value)),
    }
  }
  (result_left, result_right)
}

#[inline]
pub(crate) fn reduce_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V),
) -> Option<(K, V)> {
  iterator.next().and_then(|value1| {
    iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
  })
}
