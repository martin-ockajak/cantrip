use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::RangeBounds;
use std::rc::Rc;

use crate::extensions::util::unfold::unfold;

// FIXME - find out how to generalize these for all sequences
/// Collection operations with the following properties:
///
/// - Requires an efficient way to access the collection elements by index
/// - Consumes the collection and its elements
/// - Creates a new collection
///
pub trait Indexed<Item> {
  type This<I>;

  fn distinct(self) -> Self
  where
    Item: Eq + Hash;

  fn distinct_by<K: Eq + Hash>(self, to_key: impl FnMut(&Item) -> K) -> Self;

  fn put(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item>;

  // FIXME - make moving of the element work
  fn x_put(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut value = Rc::new(element);
    unfold((0 as usize, false), |(current, done)| {
      if !*done && *current == index {
        *done = true;
        None
        // Rc::into_inner(value)
      } else {
        *current += 1;
        iterator.next()
      }
    })
    .collect()
  }

  fn replace(self, range: impl RangeBounds<usize>, replace_with: Self) -> Self
  where
    Self: IntoIterator<Item = Item>;

  fn sorted(self) -> Self
  where
    Item: Ord;

  fn sorted_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self;

  fn sorted_unstable(self) -> Self
  where
    Item: Ord;

  fn sorted_unstable_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self;
}
