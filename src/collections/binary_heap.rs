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
}
