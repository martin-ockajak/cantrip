use std::hash::Hash;
use crate::extensions::MultiMap;

pub trait Set<A> {
  type Root<X>;

  fn add(self, value: A) -> Self
  where
    A: Eq + Hash;

  fn concat(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

  fn delete(self, value: &A) -> Self
  where
    A: Eq + Hash;

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    A: Eq + Hash;

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::Root<B>
  where
    A: Eq + Hash,
    B: Eq + Hash;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>
  where
    A: Eq + Hash,
    B: Eq + Hash;

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::Root<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>;

  fn flatten<B>(self) -> Self::Root<B>
  where
    A: IntoIterator<Item = B>,
    B: Eq + Hash;
  //
  // fn group_by<K, M>(self, group_key: impl FnMut(&A) -> K) -> M
  //   where
  //     K: Eq + Hash,
  //     M: MultiMap<K, Self::Root<A>>;

  fn intersect(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::Root<B>
  where
    B: Eq + Hash;

  fn unit(value: A) -> Self
  where
    A: Eq + Hash;
}
