use std::cmp::Ordering;
use std::collections::HashSet;
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

  #[inline]
  fn add(self, key: Key, value: Value) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  #[inline]
  fn add_all(self, iterable: impl IntoIterator<Item = (Key, Value)>) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn all(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> bool;

  fn all_equal(&self) -> bool
  where
    Key: PartialEq,
    Value: PartialEq;

  fn any(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> bool;

  fn count_by(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> usize;

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

  fn filter_map<L, W>(&self, function: impl FnMut(&(&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
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

  fn find(&self, predicate: impl FnMut(&(&Key, &Value)) -> bool) -> Option<(&Key, &Value)>;

  fn find_map<B>(&self, function: impl FnMut(&(&Key, &Value)) -> Option<B>) -> Option<B>;

  #[inline]
  fn find_map_to<B>(self, function: impl FnMut((Key, Value)) -> Option<B>) -> Option<B>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().find_map(function)
  }

  fn flat_map<L, W, R>(&self, function: impl FnMut(&(&Key, &Value)) -> R) -> Self::This<L, W>
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

  fn map<L, W>(&self, function: impl FnMut(&(&Key, &Value)) -> (L, W)) -> Self::This<L, W>
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

  fn max_by(&self, compare: impl FnMut(&(&Key, &Value), &(&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  fn max_by_key<K: Ord>(&self, to_key: impl FnMut(&(&Key, &Value)) -> K) -> Option<(&Key, &Value)>;

  #[inline]
  fn max_item(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.max_by(|(k1, v1), (k2, v2)| (k1, v1).cmp(&(k2, v2)))
  }

  fn min_by(&self, compare: impl FnMut(&(&Key, &Value), &(&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  fn min_by_key<K: Ord>(&self, to_key: impl FnMut(&(&Key, &Value)) -> K) -> Option<(&Key, &Value)>;

  #[inline]
  fn min_item(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.min_by(|(k1, v1), (k2, v2)| (k1, v1).cmp(&(k2, v2)))
  }

  #[inline]
  fn partition(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> (Self, Self)
  where
    Self: Default + Extend<(Key, Value)> + IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().partition(|(k, v)| predicate((&k, &v)))
  }

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
pub(crate) fn all_pairs<Item>(
  mut iterator: impl Iterator<Item = Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> bool {
  iterator.all(|x| predicate(&x))
}

#[inline]
pub(crate) fn all_equal_pairs<Item: PartialEq>(mut iterator: impl Iterator<Item = Item>) -> bool {
  match iterator.next() {
    Some(head) => iterator.all(|x| x == head),
    None => false,
  }
}

#[inline]
pub(crate) fn any_pairs<Item>(
  mut iterator: impl Iterator<Item = Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> bool {
  iterator.any(|x| predicate(&x))
}

#[inline]
pub(crate) fn count_by_pairs<Item>(
  iterator: impl Iterator<Item = Item>, predicate: impl FnMut(&Item) -> bool,
) -> usize {
  iterator.filter(predicate).count()
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

#[inline]
pub(crate) fn map_pairs<'a, K: 'a, V: 'a, L, W, Result: FromIterator<(L, W)>>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut(&(&K, &V)) -> (L, W),
) -> Result {
  iterator.map(|x| function(&x)).collect()
}

#[inline]
pub(crate) fn reduce_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V),
) -> Option<(K, V)> {
  iterator.next().and_then(|value1| {
    iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
  })
}
