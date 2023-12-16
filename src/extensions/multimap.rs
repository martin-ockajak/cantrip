use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::iter;

pub trait MultiMap<K, C> {
  fn from_pairs<V>(iterator: impl IntoIterator<Item = (K, V)>) -> Self
  where
    C: Extend<V> + Default;
}

impl<K: Eq + Hash, C> MultiMap<K, C> for HashMap<K, C> {
  fn from_pairs<V>(iterator: impl IntoIterator<Item = (K, V)>) -> Self
  where
    C: Extend<V> + Default,
  {
    let mut result = Self::default();
    for (key, value) in iterator {
      result.entry(key).and_modify(|values| values.extend(iter::once(value))).or_insert(C::default());
    }
    result
  }
}

impl<K: Ord, C> MultiMap<K, C> for BTreeMap<K, C> {
  fn from_pairs<V>(iterator: impl IntoIterator<Item = (K, V)>) -> Self
  where
    C: Extend<V> + Default,
  {
    let mut result = Self::default();
    for (key, value) in iterator {
      result.entry(key).and_modify(|values| values.extend(iter::once(value))).or_insert(C::default());
    }
    result
  }
}
