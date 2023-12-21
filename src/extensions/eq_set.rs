use std::collections::HashMap;
use std::hash::Hash;

use crate::extensions::util::multi_map::MultiMap;

pub trait EqSet<Item> {
  type This<T>;

  #[inline]
  fn grouped_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    HashMap::group_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }
}
