use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

use crate::Iterable;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Item> Collection<Item> for VecDeque<Item> {
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
  fn find_map_ref<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  #[inline]
  fn fold_ref<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().fold(initial_value, function)
  }

  #[inline]
  fn for_each(&self, function: impl FnMut(&Item)) {
    self.iter().for_each(function);
  }

  #[inline]
  fn group_fold_ref<K, B>(
    &self, to_key: impl FnMut(&Item) -> K, initial_value: B, function: impl FnMut(B, &Item) -> B,
  ) -> HashMap<K, B>
  where
    K: Eq + Hash,
    B: Clone,
  {
    group_fold(self.iter(), to_key, initial_value, function)
  }

  #[inline]
  fn group_reduce_ref<K>(
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
  fn reduce_ref(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item> {
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

impl<Item> CollectionTo<Item> for VecDeque<Item> {
  type This<I> = VecDeque<I>;

  #[inline]
  fn add(mut self, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.push_back(element);
    self
  }

  #[inline]
  fn add_multi(mut self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    elements.into_iter().for_each(|x| {
      self.push_back(x);
    });
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
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    if let Some(index) = self.iter().position(|x| x == element) {
      let _unused = self.remove(index);
    }
    self
  }

  #[inline]
  fn delete_multi<'a>(mut self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    for element in elements.iterator() {
      if let Some(index) = self.iter().position(|x| x == element) {
        let _unused = self.remove(index);
      }
    }
    self
  }

  #[inline]
  fn filter_map_ref<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().filter_map(function).collect()
  }

  #[inline]
  fn filter_ref(&self, mut predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Item: Clone,
  {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  #[inline]
  fn flat_map_ref<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>,
  {
    self.iter().flat_map(function).collect()
  }

  #[inline]
  fn map_ref<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().map(function).collect()
  }

  #[inline]
  fn partitions(&self) -> Vec<Vec<Self>>
  where
    Item: Clone,
    Self: Sized,
  {
    partitions(self.iter())
  }

  #[inline]
  fn partition_map_ref<A, B>(&self, function: impl FnMut(&Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
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
    if let Some(index) = self.iter().position(|x| x == element) {
      self[index] = replacement;
    }
    self
  }
}

impl<Item> Sequence<Item> for VecDeque<Item> {
  #[inline]
  fn common_prefix_length<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> usize
  where
    Item: PartialEq + 'a,
  {
    common_prefix_length(self.iter(), elements)
  }

  #[inline]
  fn common_suffix_length<'a, I>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item, Iterator<'a> = I>) -> usize
  where
    I: DoubleEndedIterator<Item = &'a Item>,
    Item: PartialEq + 'a,
  {
    common_suffix_length(self.iter().rev(), elements)
  }

  #[inline]
  fn count_unique(&self) -> usize
  where
    Item: Eq + Hash,
  {
    count_unique(self.iter())
  }

  #[inline]
  fn equivalent<'a>(&'a self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    equivalent(self.iter(), iterable)
  }

  #[inline]
  fn find_position(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<(usize, &Item)> {
    self.iter().enumerate().find(|(_, x)| predicate(x))
  }

  #[inline]
  fn frequencies<'a>(&'a self) -> HashMap<&'a Item, usize>
  where
    Item: Eq + Hash + 'a,
  {
    frequencies(self.iter())
  }

  #[inline]
  fn frequencies_by<K: Eq + Hash>(&self, to_key: impl FnMut(&Item) -> K) -> HashMap<K, usize> {
    frequencies_by(self.iter(), to_key)
  }

  #[inline]
  fn joined(&self, separator: &str) -> String
  where
    Item: Display,
  {
    joined(self.iter(), separator)
  }

  #[inline]
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  #[inline]
  fn position_multi(&self, predicate: impl FnMut(&Item) -> bool) -> Vec<usize> {
    positions(self.iter(), predicate)
  }

  #[inline]
  fn position_sequence<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Option<usize>
  where
    Item: PartialEq + 'a,
  {
    position_sequence(self.iter(), elements)
  }

  #[inline]
  fn rfind(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().rev().find(|&x| predicate(x))
  }

  #[inline]
  fn rfold_ref<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().rfold(initial_value, function)
  }

  #[inline]
  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> SequenceTo<Item> for VecDeque<Item> {
  type This<I> = VecDeque<I>;

  #[inline]
  fn add_at(mut self, index: usize, element: Item) -> Self {
    let size = self.len();
    assert!(index <= size, "index (is {index:?}) should be <= len (is {size:?})");
    self.insert(index, element);
    self
  }

  #[inline]
  fn add_at_multi(mut self, index: usize, elements: impl IntoIterator<Item = Item>) -> Self {
    let size = self.len();
    assert!(index <= size, "index (is {index:?}) should be <= len (is {size:?})");
    for (offset, element) in elements.into_iter().enumerate() {
      self.insert(index + offset, element);
    }
    self
  }

  #[inline]
  fn cartesian_product(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized,
  {
    cartesian_product(self.iter(), k)
  }

  #[inline]
  fn combinations_multi(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized,
  {
    combinations_multi(self.iter(), k)
  }

  #[inline]
  fn delete_at(mut self, index: usize) -> Self {
    if self.remove(index).is_none() {
      let size = self.len();
      panic!("index (is {index:?}) should be < len (is {size:?})")
    }
    self
  }

  fn delete_at_multi(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
    let mut deleted_indices = Vec::<usize>::from_iter(indices);
    let mut last = usize::MAX;
    deleted_indices.sort_unstable();
    for index in deleted_indices.into_iter().rev() {
      if index != last {
        if self.remove(index).is_none() {
          let size = self.len();
          panic!("index (is {index:?}) should be < len (is {size:?})")
        }
        last = index;
      }
    }
    self
  }

  #[inline]
  fn init(mut self) -> Self {
    if !self.is_empty() {
      let _unused = self.remove(self.len() - 1);
    }
    self
  }

  #[inline]
  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().map_while(predicate).collect()
  }

  fn move_at(mut self, source_index: usize, target_index: usize) -> Self {
    if source_index == target_index {
      let size = self.len();
      assert!(source_index < size, "source index (is {source_index:?}) should be < len (is {size:?})");
    } else if let Some(item) = self.remove(source_index) {
      self.insert(target_index, item);
    }
    self
  }

  #[inline]
  fn scan_ref<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
  {
    self.iter().scan(init, function).collect()
  }

  #[inline]
  fn substitute_at(mut self, index: usize, replacement: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self[index] = replacement;
    self
  }

  #[inline]
  fn substitute_at_multi(
    mut self, indices: impl IntoIterator<Item = usize>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    for (index, replacement) in indices.into_iter().zip(replacements) {
      self[index] = replacement;
    }
    self
  }

  #[inline]
  fn swap_at(mut self, source_index: usize, target_index: usize) -> Self {
    self.swap(source_index, target_index);
    self
  }

  #[inline]
  fn tail(mut self) -> Self {
    if !self.is_empty() {
      let _unused = self.remove(0);
    }
    self
  }

  #[inline]
  fn variations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized,
  {
    variations(self.iter(), k)
  }

  #[inline]
  fn windowed(&self, size: usize, step: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    windowed(self.iter(), size, step)
  }

  #[inline]
  fn windowed_circular(&self, size: usize, step: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    windowed_circular(self.iter(), size, step)
  }
}

impl<Item> List<Item> for VecDeque<Item> {
  #[inline]
  fn first(&self) -> Option<&Item> {
    self.front()
  }

  #[inline]
  fn last(&self) -> Option<&Item> {
    self.back()
  }

  #[inline]
  fn repeat(self, n: usize) -> Self
  where
    Item: Clone,
  {
    repeat(self.iter(), n)
  }
}
