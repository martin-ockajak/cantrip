use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

use crate::extensions::api::iterable::Iterable;
use crate::extensions::api::set::{SetCollection, SetFunctor, SetMonad};

impl<A, B> SetFunctor<A, B> for HashSet<A> {
  type C<X> = HashSet<B>;

  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B>
  where
    B: Eq + Hash,
  {
    self.iter().map(function).collect()
  }
}

impl<A, B> SetMonad<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(value: A) -> Self::C<A>
  where
    A: Clone + Eq + Hash,
  {
    iter::once(value).collect()
  }

  fn flat_map<R>(&self, function: impl Fn(&A) -> R) -> Self::C<B>
  where
    R: IntoIterator<Item = B> + Clone,
    B: Eq + Hash,
  {
    self.iter().flat_map(function).collect()
  }
}

impl<A> Iterable<A> for HashSet<A> {
  fn all(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn reduce(&self, function: impl Fn(&A, &A) -> A) -> Option<A>
  where
    A: Clone,
  {
    let mut iterator = self.iter();
    iterator.next().and_then(|head| Some(iterator.fold(head.clone(), |r, x| function(&r, x))))
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    let values = self.iter().collect::<Vec<&A>>();
    values.iter().rfold(init, |r, x| function(r, x))
  }
}

impl<A: Eq + Hash + Clone> SetCollection<A> for HashSet<A> {
  type C<X> = HashSet<X>;

  fn add(self, value: A) -> Self {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn delete(self, value: &A) -> Self {
    self.into_iter().filter(|x| x == value).collect()
  }

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self {
    let mut removed: HashSet<A> = HashSet::new();
    removed.extend(iterable.into_iter());
    self.into_iter().filter(|x| removed.contains(x)).collect()
  }

  fn filter(self, predicate: impl Fn(&A) -> bool) -> Self {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B>
  where
    B: Eq + Hash,
  {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B>
    where
      B: Eq + Hash,
  {
    self.iter().find_map(function)
  }

  fn merge(self, iterable: impl IntoIterator<Item = A>) -> Self {
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
