use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

use crate::extensions::*;
use crate::Iterable;

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
  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    self.iter().all(predicate)
  }

  #[inline]
  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool {
    self.iter().any(predicate)
  }

  #[inline]
  fn collect<B>(&self) -> B
  where
    Key: Clone,
    Value: Clone,
    B: FromIterator<(Key, Value)>,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn count_by(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> usize {
    self.iter().filter(|&x| predicate(x)).count()
  }

  #[inline]
  fn count_unique(&self) -> usize
  where
    Value: Eq + Hash,
  {
    count_unique(self.values())
  }

  #[inline]
  fn disjoint<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a,
  {
    disjoint(self.keys(), elements)
  }

  #[inline]
  fn filter_map_ref<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().filter_map(function).collect()
  }

  #[inline]
  fn filter_ref(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Self
  where
    Key: Clone,
    Value: Clone,
  {
    self.iter().filter(|&x| predicate(x)).map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn find(&self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)> {
    self.iter().find(|&x| predicate(x))
  }

  #[inline]
  fn find_map_ref<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  #[inline]
  fn flat_map_ref<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().flat_map(function).collect()
  }

  #[inline]
  fn fold_ref<B>(&self, initial_value: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B {
    self.iter().fold(initial_value, function)
  }

  #[inline]
  fn for_each(&self, function: impl FnMut((&Key, &Value))) {
    self.iter().for_each(function)
  }

  #[inline]
  fn map_ref<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.iter().map(function).collect()
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn max_by_key<K>(&self, mut to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>
  where
    K: Ord,
  {
    self.iter().max_by_key(|&x| to_key(x))
  }

  #[inline]
  fn min_by(&self, mut compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn min_by_key<K>(&self, mut to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>
  where
    K: Ord,
  {
    self.iter().min_by_key(|&x| to_key(x))
  }

  #[inline]
  fn minmax_by(
    &self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering,
  ) -> Option<((&Key, &Value), (&Key, &Value))> {
    minmax_by_pairs(self.iter(), compare)
  }

  #[inline]
  fn minmax_by_key<K>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<((&Key, &Value), (&Key, &Value))>
  where
    K: Ord,
  {
    minmax_by_key_pairs(self.iter(), to_key)
  }

  #[inline]
  fn partition_map_ref<L1, W1, L2, W2>(
    &self, function: impl FnMut((&Key, &Value)) -> Result<(L1, W1), (L2, W2)>,
  ) -> (Self::This<L1, W1>, Self::This<L2, W2>)
  where
    Self::This<L1, W1>: Default + Extend<(L1, W1)>,
    Self::This<L2, W2>: Default + Extend<(L2, W2)>,
  {
    partition_map_pairs(self.iter(), function)
  }

  #[inline]
  fn reduce_ref(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)> {
    reduce_pairs(self.iterator(), function)
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
  fn to_bmap(self) -> BTreeMap<Key, Value>
  where
    Key: Ord + Clone,
    Value: Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_bset(self) -> BTreeSet<(Key, Value)>
  where
    Key: Ord + Clone,
    Value: Ord + Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_deque(self) -> VecDeque<(Key, Value)>
  where
    Key: Clone,
    Value: Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_heap(self) -> BinaryHeap<(Key, Value)>
  where
    Key: Ord + Clone,
    Value: Ord + Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_keys(&self) -> Vec<Key>
  where
    Key: Clone,
  {
    self.keys().cloned().collect()
  }

  #[inline]
  fn to_list(self) -> LinkedList<(Key, Value)>
  where
    Key: Clone,
    Value: Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_map(self) -> HashMap<Key, Value>
  where
    Key: Eq + Hash + Clone,
    Value: Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_set(self) -> HashSet<(Key, Value)>
  where
    Key: Eq + Hash + Clone,
    Value: Eq + Hash + Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }

  #[inline]
  fn to_values(&self) -> Vec<Value>
  where
    Value: Clone,
  {
    self.values().cloned().collect()
  }

  #[inline]
  fn to_vec(self) -> Vec<(Key, Value)>
  where
    Key: Clone,
    Value: Clone,
  {
    self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
  }
}
