use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, LinkedList};
use std::hash::Hash;
use std::iter;

use crate::Iterable;
use crate::core::unfold::unfold;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Item> Collection<Item> for LinkedList<Item> {}

impl<Item> CollectionTo<Item> for LinkedList<Item> {
  type This<I> = LinkedList<I>;

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
}

impl<Item> Sequence<Item> for LinkedList<Item> {
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

impl<Item> SequenceTo<Item> for LinkedList<Item> {
  type This<I> = LinkedList<I>;

  #[inline]
  fn add_at(self, index: usize, element: Item) -> Self {
    self.add_at_multi(index, iter::once(element))
  }

  fn add_at_multi(self, index: usize, elements: impl IntoIterator<Item = Item>) -> Self {
    let size = self.len();
    assert!(index <= size, "addition index (is {index:?}) should be <= len (is {size:?})");
    let mut iterator = self.into_iter();
    let mut added = elements.into_iter();
    let mut current_index = 0_usize;
    unfold(|| {
      if current_index >= index {
        added.next().or_else(|| {
          current_index += 1;
          iterator.next()
        })
      } else {
        current_index += 1;
        iterator.next()
      }
    })
    .collect()
  }

  #[inline]
  fn delete_at(self, index: usize) -> Self {
    let size = self.len();
    assert!(index < size, "removal index (is {index:?}) should be < len (is {size:?})");
    self.into_iter().enumerate().filter_map(|(i, x)| if i == index { None } else { Some(x) }).collect()
  }

  #[inline]
  fn delete_at_multi(self, indices: impl IntoIterator<Item = usize>) -> Self {
    let size = self.len();
    let positions = indices
      .into_iter()
      .inspect(|&index| {
        assert!(index < size, "removal index (is {index:?}) should be < len (is {size:?})");
      })
      .collect::<BTreeSet<_>>();
    self.into_iter().enumerate().filter_map(|(i, x)| if positions.contains(&i) { None } else { Some(x) }).collect()
  }

  #[inline]
  fn init(mut self) -> Self {
    let _unused = self.pop_back();
    self
  }

  fn move_at(self, source_index: usize, target_index: usize) -> Self {
    let size = self.len();
    assert!(source_index < size, "source index (is {source_index:?}) should be < len (is {size:?})");
    assert!(target_index < size, "target index (is {target_index:?}) should be < len (is {size:?})");
    if source_index == target_index {
      return self;
    }
    let mut iterator = self.into_iter();
    let mut index = 0_usize;
    if source_index <= target_index {
      let mut source_item = None;
      unfold(|| {
        if index == source_index
          && let Some(value) = iterator.next()
        {
          source_item = Some(value);
        }
        let new_item = if index == target_index { source_item.take() } else { iterator.next() };
        index += 1;
        new_item
      })
      .collect()
    } else {
      let mut stored = LinkedList::<Item>::new();
      unfold(|| match index.cmp(&target_index) {
        Ordering::Less => {
          index += 1;
          iterator.next()
        }
        Ordering::Equal => {
          for _ in index..source_index {
            if let Some(item) = iterator.next() {
              stored.push_back(item);
            } else {
              break;
            }
          }
          iterator.next().or_else(|| stored.pop_front())
        }
        Ordering::Greater => stored.pop_front().or_else(|| iterator.next()),
      })
      .collect()
    }
  }

  #[inline]
  fn substitute_at(self, index: usize, replacement: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.substitute_at_multi(index..=index, iter::once(replacement))
  }

  fn substitute_at_multi(
    self, indices: impl IntoIterator<Item = usize>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut index_replacements = indices.into_iter().zip(replacements).collect::<BTreeMap<_, _>>();
    let mut index = 0_usize;
    let result = self
      .into_iter()
      .map(|item| {
        let new_item = index_replacements.remove(&index).unwrap_or(item);
        index += 1;
        new_item
      })
      .collect();
    if let Some(unused_index) = index_replacements.keys().next() {
      panic!("index (is {unused_index:?}) should be < len (is {index:?})");
    }
    result
  }

  fn swap_at(self, source_index: usize, target_index: usize) -> Self {
    let size = self.len();
    assert!(source_index < size, "source index (is {source_index:?}) should be < len (is {size:?})");
    assert!(target_index < size, "target index (is {target_index:?}) should be < len (is {size:?})");
    if source_index == target_index {
      return self;
    }
    let (source, target) =
      if source_index <= target_index { (source_index, target_index) } else { (target_index, source_index) };
    let mut iterator = self.into_iter();
    let mut index = 0_usize;
    let mut stored = LinkedList::<Item>::new();
    let mut source_item = None;
    unfold(|| {
      let new_item = match index.cmp(&source) {
        Ordering::Less => iterator.next(),
        Ordering::Equal => {
          source_item = iterator.next();
          for _ in (index + 1)..target {
            if let Some(item) = iterator.next() {
              stored.push_back(item);
            } else {
              break;
            }
          }
          iterator.next().or_else(|| stored.pop_front())
        }
        Ordering::Greater => {
          if index == target {
            source_item.take()
          } else {
            stored.pop_front().or_else(|| iterator.next())
          }
        }
      };
      index += 1;
      new_item
    })
    .collect()
  }

  #[inline]
  fn tail(mut self) -> Self {
    let _unused = self.pop_front();
    self
  }
}

impl<Item> List<Item> for LinkedList<Item> {
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
