use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::iter;

// pub trait MultiMapFromIterator<K, V>: Sized {
//   fn from_iter<I: IntoIterator<Item = (K, V)>>(iterator: I) -> Self;
// }

pub trait MultiMap<K, C> {
  fn add<V>(&mut self, k: K, v: V)
  where
    C: Extend<V> + Default;
}

impl<K: Eq + Hash, C> MultiMap<K, C> for HashMap<K, C> {
  fn add<V>(&mut self, k: K, v: V)
  where
    C: Extend<V> + Default,
  {
    self.entry(k).and_modify(|values| values.extend(iter::once(v))).or_insert(C::default());
  }
}

impl<K: Ord, C> MultiMap<K, C> for BTreeMap<K, C> {
  fn add<V>(&mut self, k: K, v: V)
  where
    C: Extend<V> + Default,
  {
    self.entry(k).and_modify(|values| values.extend(iter::once(v))).or_insert(C::default());
  }
}
