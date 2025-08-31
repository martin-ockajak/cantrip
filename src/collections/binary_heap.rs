use std::collections::BinaryHeap;

#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Item> Collection<Item> for BinaryHeap<Item> {}

impl<Item: Ord> CollectionTo<Item> for BinaryHeap<Item> {
  type This<I> = BinaryHeap<I>;

  #[inline]
  fn add(mut self, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.push(element);
    self
  }

  #[inline]
  fn add_multi(mut self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    elements.into_iter().for_each(|x| {
      self.push(x);
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
}
