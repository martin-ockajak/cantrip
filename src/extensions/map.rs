use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

pub trait Map<K, V> {
  type Root<X, Y>;

  fn add(self, key: K, value: V) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  fn all(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool;

  fn count_by(&self, predicate: impl FnMut((&K, &V)) -> bool) -> usize;

  fn concat(self, iterable: impl IntoIterator<Item = (K, V)>) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn delete(self, key: &K) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    self.into_iter().filter_map(|(k, v)| if &k != key { Some((k, v)) } else { None }).collect()
  }

  fn diff(self, iterable: impl IntoIterator<Item = K>) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    let mut removed: HashSet<K> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|(k, _)| !removed.contains(k)).collect()
  }

  fn filter(self, mut predicate: impl FnMut((&K, &V)) -> bool) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    self.into_iter().filter(|(k, v)| predicate((k, v))).collect()
  }

  fn filter_keys(self, mut predicate: impl FnMut(&K) -> bool) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    self.into_iter().filter(|(k, _)| predicate(k)).collect()
  }

  fn filter_map<L, W>(&self, function: impl FnMut((&K, &V)) -> Option<(L, W)>) -> Self::Root<L, W>
  where
    K: Eq + Hash,
    L: Eq + Hash;

  fn filter_values(self, mut predicate: impl FnMut(&V) -> bool) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  fn find(&self, predicate: impl FnMut((&K, &V)) -> bool) -> Option<(&K, &V)>;

  fn find_map<B>(&self, function: impl FnMut((&K, &V)) -> Option<B>) -> Option<B>
  where
    K: Eq + Hash,
    B: Eq + Hash;

  fn flat_map<L, W, R>(&self, function: impl FnMut((&K, &V)) -> R) -> Self::Root<L, W>
  where
    L: Eq + Hash,
    R: IntoIterator<Item = (L, W)>;

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&K, &V)) -> B) -> B;

  fn intersect(self, iterable: impl IntoIterator<Item = K>) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = (K, V)> + Sized + FromIterator<(K, V)>,
  {
    let mut retained: HashSet<K> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|(k, _)| retained.contains(k)).collect()
  }

  fn map<L, W>(&self, function: impl FnMut((&K, &V)) -> (L, W)) -> Self::Root<L, W>
  where
    L: Eq + Hash;

  fn map_keys<L>(self, function: impl FnMut(&K) -> L) -> Self::Root<L, V>
  where
    K: Eq + Hash,
    L: Eq + Hash;

  fn map_values<W>(self, function: impl FnMut(&V) -> W) -> Self::Root<K, W>
  where
    K: Eq + Hash,
    W: Eq + Hash;

  fn max_by(&self, compare: impl FnMut((&K, &V), (&K, &V)) -> Ordering) -> Option<(&K, &V)>;

  fn min_by(&self, compare: impl FnMut((&K, &V), (&K, &V)) -> Ordering) -> Option<(&K, &V)>;

  fn product_keys<S>(self) -> K
  where
    K: Product,
    Self: IntoIterator<Item = (K, V)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).product()
  }

  fn product_values<S>(self) -> V
  where
    V: Product,
    Self: IntoIterator<Item = (K, V)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).product()
  }

  fn reduce(&self, function: impl FnMut((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)>;

  fn sum_keys(self) -> K
  where
    K: Sum,
    Self: IntoIterator<Item = (K, V)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).sum()
  }

  fn sum_values(self) -> V
  where
    V: Sum,
    Self: IntoIterator<Item = (K, V)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).sum()
  }

  fn unit(key: K, value: V) -> Self
  where
    K: Eq + Hash,
    Self: FromIterator<(K, V)>,
  {
    iter::once((key, value)).collect()
  }
}

pub(crate) fn all_pair<A>(mut iterator: impl Iterator<Item = A>, mut predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.all(|x| predicate(&x))
}

pub(crate) fn any_pair<A>(mut iterator: impl Iterator<Item = A>, mut predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.any(|x| predicate(&x))
}

pub(crate) fn count_by_pair<A>(iterator: impl Iterator<Item = A>, predicate: impl FnMut(&A) -> bool) -> usize {
  iterator.filter(predicate).count()
}

pub(crate) fn fold_pair<A, B>(iterator: impl Iterator<Item = A>, init: B, mut function: impl FnMut(B, &A) -> B) -> B {
  iterator.fold(init, |r, x| function(r, &x))
}

pub(crate) fn reduce_pair<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V),
) -> Option<(K, V)> {
  iterator.next().and_then(|value1| {
    iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
  })
}
