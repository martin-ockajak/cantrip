use std::collections::BTreeMap;

use crate::extensions::util::multi_map::MultiMap;

pub trait OrdSet<Item> {
  type This<T>;

  fn filter_map<B: Ord>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  fn find_map<B: Ord>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>;

  fn flat_map<B: Ord, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>;

  #[inline]
  fn grouped_by<K: Ord>(self, mut to_key: impl FnMut(&Item) -> K) -> BTreeMap<K, Self>
    where
      Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    BTreeMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }
}
