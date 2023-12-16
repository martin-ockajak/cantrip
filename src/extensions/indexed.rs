use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use std::ops::RangeBounds;

use crate::extensions::Iterable;

pub trait Indexed<Item> {
  type This<I>;

  fn distinct(self) -> Self
  where
    Item: Eq + Hash;

  fn distinct_by<K>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    K: Eq + Hash;

  fn put(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item>;

  // FIXME - make the moving work
  // fn x_put(self, index: usize, element: A) -> Self
  // where
  //   Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  // {
  //   let mut iterator = self.into_iter();
  //   let mut value = Rc::new(element);
  //   unfold((0 as usize, false), |(current, done)| {
  //     if !*done && *current == index {
  //       *done = true;
  //       None
  //       // Rc::into_inner(value)
  //     } else {
  //       *current += 1;
  //       iterator.next()
  //     }
  //   })
  //   .collect()
  // }

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
