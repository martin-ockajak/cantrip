use std::collections::VecDeque;
use std::hash::Hash;

use crate::Iterable;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Item> Collection<Item> for VecDeque<Item> {}

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
    self.extend(elements);
    self
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
  fn equivalent<'a>(&'a self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a,
  {
    equivalent(self.iter(), iterable)
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
