use std::iter;
use std::iter::Cycle;
use std::slice::Iter;

use crate::extensions::traits::{Collection, Functor, Iterable, Monad};

impl<A, R> Functor<A, R> for Vec<A> {
  type C<X> = Vec<X>;

  fn map<F>(&self, function: F) -> Self::C<R> where F: Fn(&A) -> R {
    self.iter().map(function).collect()
  }
}

impl<A, R> Monad<A, R> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(self, value: A) -> Self::C<A> where A: Clone {
    iter::once(value).collect()
  }

  fn flat_map<F>(&self, function: F) -> Self::C<R> where F: Fn(&A) -> Self::C<R> {
    self.iter().flat_map(function).collect()
  }
}

impl<A> Iterable<A> for Vec<A> {
  type C<X> = Vec<X>;

  fn all<P>(&self, predicate: P) -> bool where P: Fn(&A) -> bool {
    self.iter().all(predicate)
  }

  fn any<P>(&self, predicate: P) -> bool where P: Fn(&A) -> bool {
    self.iter().any(predicate)
  }

  fn cycle(&self) -> Cycle<Iter<A>> {
    self.iter().cycle()
  }

  fn zip<I>(&self, other: &I) -> Self::C<(A, I::Item)> where I: Clone + IntoIterator, A: Clone {
    self.iter().cloned().zip(other.clone().into_iter()).collect()
  }

  fn zip_with_index(&self) -> Self::C<(A, usize)> where A: Clone {
    self.iter().cloned().zip(0..self.len()).collect()
  }

  fn filter<P>(&self, predicate: P) -> Self where P: Fn(&A) -> bool, A: Clone {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  fn find<P>(&self, predicate: P) -> Option<&A> where P: Fn(&A) -> bool, A: Clone {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B, F>(&self, init: B, function: F) -> B where F: Fn(B, &A) -> B {
    self.iter().fold(init, function)
  }

  fn rfold<B, F>(&self, init: B, function: F) -> B where F: Fn(B, &A) -> B {
    self.iter().rfold(init, function)
  }
}

impl<A: Clone> Collection<A> for Vec<A> {
  fn add(&self, value: A) -> Self {
    self.iter().chain(iter::once(&value)).cloned().collect()
  }

  fn add_seq<I>(&self, other: &I) -> Self where I: Clone + IntoIterator<Item = A> {
    self.iter().cloned().chain(other.clone().into_iter()).collect()
  }

  fn remove(&self, value: A) -> Self where A: PartialEq {
    self.iter().filter(|&x| x != &value).cloned().collect()
  }

  fn remove_seq<I>(&self, other: &I) -> Self where I: Clone + IntoIterator<Item = A>, A: PartialEq {
    let removed = other.clone().into_iter().collect::<Vec<A>>();
    self.iter().filter(|&x| removed.contains(&x)).cloned().collect()
  }
}


#[cfg(test)]
mod tests {
  use crate::extensions::traits::*;

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
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.filter(predicate);
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
    assert_eq!(1, 1)
  }
}
