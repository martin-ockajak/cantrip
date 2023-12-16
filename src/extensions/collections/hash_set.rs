use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::extensions::*;

impl<A> Iterable<A> for HashSet<A> {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    any(self.iter(), predicate)
  }

  fn count_by(&self, predicate: impl FnMut(&A) -> bool) -> usize {
    count_by(self.iter(), predicate)
  }

  fn find(&self, mut predicate: impl FnMut(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    fold(self.iter(), init, function)
  }

  fn max_by(&self, mut compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn reduce(&self, function: impl FnMut(&A, &A) -> A) -> Option<A> {
    reduce(self.iter(), function)
  }
}

impl<A> Aggregable<A> for HashSet<A> {}

impl<A> Collectible<A> for HashSet<A> {
  type Root<X> = HashSet<X>;
}

impl<A> Set<A> for HashSet<A> {
  type Root<X> = HashSet<X>;

  fn exclude(self, value: &A) -> Self
  where
    A: Eq + Hash,
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    self.into_iter().filter(|x| x != value).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::Root<B>
  where
    A: Eq + Hash,
    B: Eq + Hash,
  {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>
  where
    A: Eq + Hash,
    B: Eq + Hash,
  {
    self.iter().find_map(function)
  }

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::Root<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }

  fn flatten<B>(self) -> Self::Root<B>
  where
    A: IntoIterator<Item = B>,
    B: Eq + Hash,
  {
    self.into_iter().flatten().collect()
  }

  fn group_by<K>(self, mut to_key: impl FnMut(&A) -> K) -> HashMap<K, Self>
    where
      A: Eq + Hash,
      K: Eq + Hash,
      Self: Sized,
  {
    HashMap::from_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::Root<B>
  where
    B: Eq + Hash,
  {
    self.iter().map(function).collect()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use crate::extensions::*;

  #[quickcheck]
  fn map(data: HashSet<i32>) -> bool {
    let function = |x: &i32| *x as i64;
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<HashSet<i64>>();
    result == expected
  }

  #[quickcheck]
  fn filter(data: HashSet<i32>) -> bool {
    let predicate = |x: &i32| x % 2 == 0;
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.clone().filter(predicate);
    let expected = data.iter().filter(|&x| predicate(x)).cloned().collect::<HashSet<i32>>();
    result == expected
  }

  #[quickcheck]
  fn fold(data: HashSet<i32>) -> bool {
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.fold(0, function);
    let expected = data.iter().fold(0, function);
    result == expected
  }
}
