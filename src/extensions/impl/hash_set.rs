use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

use crate::extensions::api::iterable::IterableOps;
use crate::extensions::api::set::{SetFunctor, SetMonad, SetOps};

impl<A> SetFunctor<A> for HashSet<A> {
  type C<X> = HashSet<X>;

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::C<B>
  where
    B: Eq + Hash,
  {
    self.iter().map(function).collect()
  }
}

impl<A> SetMonad<A> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(value: A) -> Self::C<A>
  where
    A: Eq + Hash,
  {
    iter::once(value).collect()
  }

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::C<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }
}

impl<A> IterableOps<A> for HashSet<A> {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn find(&self, mut predicate: impl FnMut(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn reduce(&self, mut function: impl FnMut(&A, &A) -> A) -> Option<A> {
    let mut iterator = self.iter();
    match iterator.next() {
      Some(value1) => match iterator.next() {
        Some(value2) => Some(iterator.fold(function(value1, value2), |r, x| function(&r, x))),
        _ => None,
      },
      _ => None,
    }
  }

  fn rfold<B>(&self, init: B, mut function: impl FnMut(B, &A) -> B) -> B {
    let values = self.iter().collect::<Vec<&A>>();
    values.iter().rfold(init, |r, x| function(r, x))
  }
}

impl<A> SetOps<A> for HashSet<A> {
  type C<X> = HashSet<X>;

  fn add(self, value: A) -> Self
  where
    A: Eq + Hash,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn delete(self, value: &A) -> Self
  where
    A: Eq + Hash,
  {
    self.into_iter().filter(|x| x == value).collect()
  }

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash,
  {
    let mut removed: HashSet<A> = HashSet::new();
    removed.extend(iterable.into_iter());
    self.into_iter().filter(|x| removed.contains(x)).collect()
  }

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    A: Eq + Hash,
  {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::C<B>
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

  fn merge(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }
}

#[cfg(test)]
mod tests {
  use crate::extensions::*;
  use std::collections::HashSet;

  #[quickcheck]
  fn test_map_hash_set(data: HashSet<i32>) -> bool {
    let function = |x: &i32| *x as i64;
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<HashSet<i64>>();
    result == expected
  }

  #[quickcheck]
  fn test_filter_hash_set(data: HashSet<i32>) -> bool {
    let predicate = |x: &i32| x % 2 == 0;
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.clone().filter(predicate);
    let expected = data.iter().filter(|&x| predicate(x)).cloned().collect::<HashSet<i32>>();
    result == expected
  }

  #[quickcheck]
  fn test_fold_hash_set(data: HashSet<i32>) -> bool {
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.fold(0, function);
    let expected = data.iter().fold(0, function);
    result == expected
  }
}
