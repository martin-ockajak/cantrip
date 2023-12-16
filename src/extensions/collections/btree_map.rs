use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::extensions::*;

impl<Key, Value> Map<Key, Value> for BTreeMap<Key, Value> {
  type This<X, V> = BTreeMap<X, V>;

  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn count_by(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> usize {
    self.iter().filter(|&(k, v)| predicate((k, v))).count()
  }

  fn find(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn max_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn reduce(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)> {
    reduce_pair(self.iter(), function)
  }
}

impl<Key, Value> OrdMap<Key, Value> for BTreeMap<Key, Value> {
  type This<X, V> = BTreeMap<X, V>;

  fn filter_map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Key: Ord,
    L: Ord,
  {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>
  where
    Key: Ord,
    B: Ord,
  {
    self.iter().find_map(function)
  }

  fn flat_map<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    L: Ord,
    R: IntoIterator<Item = (L, W)>,
  {
    self.iter().flat_map(function).collect()
  }

  fn map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    L: Ord,
  {
    self.iter().map(function).collect()
  }

  fn map_keys<L>(self, mut function: impl FnMut(&Key) -> L) -> Self::This<L, Value>
  where
    Key: Ord,
    L: Ord,
  {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  fn map_values<W>(self, mut function: impl FnMut(&Value) -> W) -> Self::This<Key, W>
  where
    Key: Ord,
    W: Ord,
  {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::BTreeMap;

  use crate::extensions::*;

  #[quickcheck]
  fn map(data: BTreeMap<i32, i32>) -> bool {
    let function = |(k, v): (&i32, &i32)| (*k, *v as i64);
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<BTreeMap<i32, i64>>();
    result == expected
  }
}
