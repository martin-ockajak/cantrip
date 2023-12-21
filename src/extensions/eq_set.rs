use std::collections::HashMap;
use std::hash::Hash;

use crate::extensions::util::multi_map::MultiMap;

pub trait EqSet<Item> {
  type This<T>;

  fn filter_map<B: Eq + Hash>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  fn find_map<B: Eq + Hash>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>;

  fn flat_map<B: Eq + Hash, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>;

  #[inline]
  fn grouped_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    HashMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn map<B: Eq + Hash>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>;
}
