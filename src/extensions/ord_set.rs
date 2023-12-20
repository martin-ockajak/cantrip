use std::collections::BTreeMap;
use std::iter;

use crate::extensions::util::multi_map::MultiMap;

pub trait OrdSet<Item: Ord> {
  type This<T>;

  fn add(self, value: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B: Ord>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  fn find_map<B: Ord>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>;

  fn flat_map<B: Ord, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>;

  fn flat<B: Ord>(self) -> Self::This<B>
  where
    Item: IntoIterator<Item = B>,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  fn exclude(self, value: &Item) -> Self;

  fn grouped_by<K: Ord>(self, mut to_key: impl FnMut(&Item) -> K) -> BTreeMap<K, Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    BTreeMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn map<B: Ord>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>;

  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
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
