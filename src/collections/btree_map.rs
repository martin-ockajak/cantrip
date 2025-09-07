use std::collections::BTreeMap;
use std::hash::Hash;

use crate::Iterable;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Key: Ord, Value> Map<Key, Value> for BTreeMap<Key, Value> {
  type This<X, V> = BTreeMap<X, V>;

  #[inline]
  fn add(mut self, key: Key, value: Value) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let _unused = self.insert(key, value);
    self
  }

  #[inline]
  fn add_multi(mut self, entries: impl IntoIterator<Item = (Key, Value)>) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    for (k, v) in entries {
      let _unused = self.insert(k, v);
    }
    self
  }

  #[inline]
  fn count_unique(&self) -> usize
  where
    Value: Eq + Hash,
  {
    count_unique(self.values())
  }

  #[inline]
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a,
  {
    subset(self.keys(), elements)
  }

  #[inline]
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a,
  {
    superset(self.keys(), elements)
  }

  #[inline]
  fn to_keys(&self) -> Vec<Key>
  where
    Key: Clone,
  {
    self.keys().cloned().collect()
  }

  #[inline]
  fn to_values(&self) -> Vec<Value>
  where
    Value: Clone,
  {
    self.values().cloned().collect()
  }
}
