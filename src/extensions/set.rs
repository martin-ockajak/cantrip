use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

pub trait Set<Item> {
  type This<T>;

  fn add(self, value: Item) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn concat(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Item: Eq + Hash,
    B: Eq + Hash;

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>
  where
    Item: Eq + Hash,
    B: Eq + Hash;

  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>;

  fn flatten<B>(self) -> Self::This<B>
  where
    Item: IntoIterator<Item = B>,
    B: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  fn exclude(self, value: &Item) -> Self
  where
    Item: Eq + Hash;

  fn group_by<K>(self, to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Item: Eq + Hash,
    K: Eq + Hash,
    Self: Sized;

  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    B: Eq + Hash;

  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
    Item: Eq + Hash,
    Self: Sized + Default + Extend<Item> + IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().partition(predicate)
  }

  fn unit(value: Item) -> Self
  where
    Item: Eq + Hash,
    Self: FromIterator<Item>,
  {
    iter::once(value).collect()
  }
}
