use std::cmp::Ordering;
use std::iter;
use std::iter::{Product, Sum};

pub trait Map<Key, Value> {
  type This<K, V>;

  #[inline]
  fn add(self, key: Key, value: Value) -> Self
    where
      Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  fn count_by(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> usize;

  fn find(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)>;

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B;

  fn max_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  #[inline]
  fn max_entry(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.max_by(|(k1, v1), (k2, v2)| (k1, v1).cmp(&(k2, v2)))
  }

  fn min_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  #[inline]
  fn min_entry(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.min_by(|(k1, v1), (k2, v2)| (k1, v1).cmp(&(k2, v2)))
  }

  #[inline]
  fn merge(self, iterable: impl IntoIterator<Item = (Key, Value)>) -> Self
    where
      Self: IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iterable).collect()
  }

  #[inline]
  fn partition(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> (Self, Self)
  where
    Self:
      Sized + Default + Extend<(Key, Value)> + IntoIterator<Item = (Key, Value)> + Sized + FromIterator<(Key, Value)>,
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
pub(crate) fn all_pairs<A>(mut iterator: impl Iterator<Item = A>, mut predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.all(|x| predicate(&x))
}

#[inline]
pub(crate) fn any_pairs<A>(mut iterator: impl Iterator<Item = A>, mut predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.any(|x| predicate(&x))
}

#[inline]
pub(crate) fn count_by_pairs<A>(iterator: impl Iterator<Item = A>, predicate: impl FnMut(&A) -> bool) -> usize {
  iterator.filter(predicate).count()
}

#[inline]
pub(crate) fn fold_pairs<'a, K: 'a, V: 'a, B>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, init: B, function: impl FnMut(B, (&K, &V)) -> B,
) -> B {
  iterator.fold(init, function)
}

#[inline]
pub(crate) fn reduce_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V),
) -> Option<(K, V)> {
  iterator.next().and_then(|value1| {
    iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
  })
}
