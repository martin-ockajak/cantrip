use std::collections::BTreeMap;
use std::iter;

use crate::extensions::util::multi_map::MultiMap;

pub trait OrdSet<Item> {
  type This<T>;

  fn add(self, value: Item) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Item: Ord,
    B: Ord;

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>
  where
    Item: Ord,
    B: Ord;

  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    B: Ord,
    R: IntoIterator<Item = B>;

  fn flat<B>(self) -> Self::This<B>
  where
    Item: IntoIterator<Item = B>,
    B: Ord,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  fn exclude(self, value: &Item) -> Self
  where
    Item: Ord;

  fn grouped_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> BTreeMap<K, Self>
  where
    Item: Ord,
    K: Ord,
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    BTreeMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    B: Ord;

  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
    where
      Item: Ord,
      Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
    Item: Ord,
    Self: Sized + Default + Extend<Item> + IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().partition(predicate)
  }

  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item> + Sized,
  {
    iter::once(value).collect()
  }
}
