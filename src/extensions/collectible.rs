use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub trait Collectible<Item> {
  type This<I>;

  fn diff(self, iterable: impl IntoIterator<Item = Item>) -> Self
    where
      Item: Eq + Hash,
      Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut removed: HashSet<Item> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  fn intersect(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut retained: HashSet<Item> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }

  fn reduce(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item>
    where
      Self: IntoIterator<Item = Item> + Sized,
  {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }
}
