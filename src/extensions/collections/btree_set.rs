use crate::extensions::*;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, LinkedList};
use std::hash::Hash;

impl<Item> Traversable<Item> for BTreeSet<Item> {
  #[inline]
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  #[inline]
  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    any(self.iter(), predicate)
  }

  #[inline]
  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize {
    count_by(self.iter(), predicate)
  }

  #[inline]
  fn disjoint<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    disjoint(self.iter(), elements)
  }

  #[inline]
  fn find(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().find(|&x| predicate(x))
  }

  #[inline]
  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  #[inline]
  fn fold<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().fold(initial_value, function)
  }

  #[inline]
  fn for_each(&self, function: impl FnMut(&Item)) {
    self.iter().for_each(function)
  }

  #[inline]
  fn group_fold<K, B>(
    &self, to_key: impl FnMut(&Item) -> K, initial_value: B, function: impl FnMut(B, &Item) -> B,
  ) -> HashMap<K, B>
  where
    K: Eq + Hash,
    B: Clone,
  {
    group_fold(self.iter(), to_key, initial_value, function)
  }

  #[inline]
  fn group_reduce<K>(
    &self, to_key: impl FnMut(&Item) -> K, function: impl FnMut(&Item, &Item) -> Item,
  ) -> HashMap<K, Item>
  where
    K: Eq + Hash,
    Item: Clone,
  {
    group_reduce(self.iter(), to_key, function)
  }

  #[inline]
  fn max_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn max_by_key<K>(&self, mut to_key: impl FnMut(&Item) -> K) -> Option<&Item>
  where
    K: Ord,
  {
    self.iter().max_by_key(|&x| to_key(x))
  }

  #[inline]
  fn min_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  #[inline]
  fn min_by_key<K>(&self, mut to_key: impl FnMut(&Item) -> K) -> Option<&Item>
  where
    K: Ord,
  {
    self.iter().min_by_key(|&x| to_key(x))
  }

  #[inline]
  fn minmax_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<(&Item, &Item)> {
    minmax_by(self.iter(), compare)
  }

  #[inline]
  fn minmax_by_key<K>(&self, to_key: impl FnMut(&Item) -> K) -> Option<(&Item, &Item)>
  where
    K: Ord,
  {
    minmax_by_key(self.iter(), to_key)
  }

  #[inline]
  fn reduce(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item> {
    reduce(self.iter(), function)
  }

  #[inline]
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    subset(self.iter(), elements)
  }

  #[inline]
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    superset(self.iter(), elements)
  }
}

impl<Item: Ord> Collectible<Item> for BTreeSet<Item> {
  type This<I> = BTreeSet<I>;

  #[inline]
  fn add(mut self, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let _ = self.insert(element);
    self
  }

  #[inline]
  fn add_multi(mut self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    for x in elements {
      let _unused = self.insert(x);
    }
    self
  }

  #[inline]
  fn combinations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
  {
    combinations(self.iter(), k)
  }

  #[inline]
  fn delete(mut self, element: &Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let _unused = self.remove(element);
    self
  }

  #[inline]
  fn delete_multi<'a>(mut self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    for element in elements.iterator() {
      let _unused = self.remove(element);
    }
    self
  }

  #[inline]
  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().filter_map(function).collect()
  }

  #[inline]
  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>,
  {
    self.iter().flat_map(function).collect()
  }

  #[inline]
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().map(function).collect()
  }

  #[inline]
  fn partition_map<A, B>(&self, function: impl FnMut(&Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
  where
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>,
  {
    partition_map(self.iter(), function)
  }

  #[inline]
  fn powerset(&self) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized,
  {
    powerset(self.iter())
  }

  #[inline]
  fn substitute(mut self, element: &Item, replacement: Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    if self.remove(element) {
      let _unused = self.insert(replacement);
    }
    self
  }

  fn substitute_multi<'a>(
    mut self, elements: &'a impl Iterable<Item<'a> = &'a Item>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut replacement_values = LinkedList::<Item>::new();
    for (element, replacement) in elements.iterator().zip(replacements) {
      if self.remove(element) {
        replacement_values.push_back(replacement);
      }
    }
    for replacement in replacement_values {
      let _unused = self.insert(replacement);
    }
    self
  }
}
