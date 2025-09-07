use std::collections::HashMap;
use std::hash::Hash;

use crate::Iterable;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

#[allow(clippy::implicit_hasher)]
impl<Key: Eq + Hash, Value> Map<Key, Value> for HashMap<Key, Value> {
  type This<X, V> = HashMap<X, V>;

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
  fn delete(mut self, key: &Key) -> Self
  where
    Key: PartialEq,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let _unused = self.remove(key);
    self
  }

  #[inline]
  fn delete_multi<'a>(mut self, keys: &'a impl Iterable<Item<'a> = &'a Key>) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    for key in keys.iterator() {
      let _unused = self.remove(key);
    }
    self
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
