use std::collections::{BTreeSet, LinkedList};
use std::hash::Hash;

use crate::Iterable;
#[allow(clippy::wildcard_imports)]
use crate::extensions::*;

impl<Item> Collection<Item> for BTreeSet<Item> {}

impl<Item: Ord> CollectionTo<Item> for BTreeSet<Item> {
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
    self.extend(elements);
    self
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
