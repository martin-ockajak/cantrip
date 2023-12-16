use std::collections::HashMap;
use std::hash::Hash;
use std::iter;

pub trait Set<A> {
  type This<Item>;

  fn add(self, value: A) -> Self
  where
    A: Eq + Hash,
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn concat(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash,
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().chain(iterable).collect()
  }

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    A: Eq + Hash,
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::This<B>
  where
    A: Eq + Hash,
    B: Eq + Hash;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>
  where
    A: Eq + Hash,
    B: Eq + Hash;

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::This<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>;

  fn flatten<B>(self) -> Self::This<B>
  where
    A: IntoIterator<Item = B>,
    B: Eq + Hash,
    Self: IntoIterator<Item = A> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  fn exclude(self, value: &A) -> Self
  where
    A: Eq + Hash;

  fn group_by<K>(self, to_key: impl FnMut(&A) -> K) -> HashMap<K, Self>
  where
    A: Eq + Hash,
    K: Eq + Hash,
    Self: Sized;

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::This<B>
  where
    B: Eq + Hash;

  fn partition(self, predicate: impl FnMut(&A) -> bool) -> (Self, Self)
  where
    A: Eq + Hash,
    Self: Sized + Default + Extend<A> + IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().partition(predicate)
  }

  fn unit(value: A) -> Self
  where
    A: Eq + Hash,
    Self: FromIterator<A>,
  {
    iter::once(value).collect()
  }
}
