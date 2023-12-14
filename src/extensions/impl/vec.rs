use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

use crate::extensions::api::iterable::Iterable;
use crate::extensions::{ListCollection, ListFunctor, ListMonad, Ordered};

impl<A, B> ListFunctor<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B> {
    self.iter().map(function).collect()
  }
}

impl<A, B> ListMonad<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(value: A) -> Self::C<A>
  {
    iter::once(value).collect()
  }

  fn flat_map<R>(&self, function: impl Fn(&A) -> R) -> Self::C<B>
  where
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(|x| function(x).into_iter()).collect()
  }
}

impl<A> Iterable<A> for Vec<A> {
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
  {
    let mut iterator = self.iter();
    match iterator.next() {
      Some(value1) => {
        match iterator.next() {
          Some(value2) => {
            Some(iterator.fold(function(value1, value2), |r, x| function(&r, x)))
          },
          _ => None
        }
      },
      _ => None
    }
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().rfold(init, function)
  }
}

impl<A> ListCollection<A> for Vec<A> {
  type C<X> = Vec<X>;

  fn add(self, value: A) -> Self {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn delete(self, value: &A) -> Self
  where
    A: PartialEq,
  {
    self.into_iter().filter(|x| x == value).collect()
  }

  fn diff(self, iterable: (impl IntoIterator<Item = A>)) -> Self
  where
    A: Eq + Hash,
  {
    let mut removed: HashSet<A> = HashSet::new();
    removed.extend(iterable.into_iter());
    self.into_iter().filter(|x| removed.contains(x)).collect()
  }

  fn enumerate(self) -> Self::C<(usize, A)> {
    (0..self.len()).zip(self.into_iter()).collect()
  }

  fn filter(self, predicate: impl Fn(&A) -> bool) -> Self {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn merge(self, iterable: impl IntoIterator<Item = A>) -> Self {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn map_while<B>(&self, predicate: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().map_while(predicate).collect()
  }

  fn partition(self, predicate: impl Fn(&A) -> bool) -> (Self, Self)
  where
    Self: Sized,
  {
    self.into_iter().partition(predicate)
  }

  fn skip(self, n: usize) -> Self {
    self.into_iter().skip(n).collect()
  }

  fn take(self, n: usize) -> Self {
    self.into_iter().take(n).collect()
  }

  fn zip<I>(self, iterable: I) -> Self::C<(A, I::Item)>
  where
    I: IntoIterator,
  {
    self.into_iter().zip(iterable.into_iter()).collect()
  }
}

impl<A> Ordered<A> for Vec<A> {
  fn position(&self, predicate: impl Fn(&A) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  fn rfind(&self, predicate: impl Fn(&A) -> bool) -> Option<&A> {
    self.iter().rev().find(|&x| predicate(x))
  }
}

#[cfg(test)]
mod tests {
  use crate::extensions::*;

  #[quickcheck]
  fn test_map_vec(data: Vec<i32>) -> bool {
    let function = |x: &i32| *x as i64;
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<Vec<i64>>();
    result == expected
  }

  #[quickcheck]
  fn test_filter_vec(data: Vec<i32>) -> bool {
    let predicate = |x: &i32| x % 2 == 0;
    let result = data.clone().filter(predicate);
    let expected = data.iter().filter(|&x| predicate(x)).cloned().collect::<Vec<i32>>();
    result == expected
  }

  #[quickcheck]
  fn test_fold_vec(data: Vec<i32>) -> bool {
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.fold(0, function);
    let expected = data.iter().fold(0, function);
    result == expected
  }

  #[test]
  fn test_x() {
    [1, 2, 3];
    &[1, 2, 3][0..];
    "Test";
    "Test".to_string();
    assert_eq!(1, 1)
  }
}
