use std::iter;
use std::iter::Cycle;
use std::slice::Iter;

use crate::extensions::traits::{Collection, Functor, Iterable, Monad};

impl<A, R> Functor<A, R> for Vec<A> {
  type C<X> = Vec<X>;

  fn map(&self, function: impl Fn(&A) -> R) -> Self::C<R> {
    self.iter().map(function).collect()
  }
}

impl<A, R> Monad<A, R> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(self, value: A) -> Self::C<A> where A: Clone {
    iter::once(value).collect()
  }

  fn flat_map(&self, function: impl Fn(&A) -> Self::C<R>) -> Self::C<R> {
    self.iter().flat_map(function).collect()
  }
}

impl<A> Iterable<A> for Vec<A> {
  type Item = A;
  type C<X> = Vec<X>;

  fn all<P>(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any<P>(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn cycle(&self) -> Cycle<Iter<A>> {
    self.iter().cycle()
  }

  fn zip<I>(&self, iterable: &I) -> Self::C<(A, I::Item)> where I: Clone + IntoIterator, A: Clone {
    self.iter().cloned().zip(iterable.clone().into_iter()).collect()
  }

  fn zip_with_index(&self) -> Self::C<(A, usize)> where A: Clone {
    self.iter().cloned().zip(0..self.len()).collect()
  }

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self where A: Clone {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A> where A: Clone {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().rfold(init, function)
  }
}

impl<A: Clone> Collection<A> for Vec<A> {
  fn add(&self, value: A) -> Self {
    self.iter().chain(iter::once(&value)).cloned().collect()
  }

  fn add_seq(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self {
    self.iter().cloned().chain(iterable.clone().into_iter()).collect()
  }

  fn remove(&self, value: A) -> Self where A: PartialEq {
    self.iter().filter(|&x| x != &value).cloned().collect()
  }

  fn remove_seq(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self where A: PartialEq {
    let removed = iterable.clone().into_iter().collect::<Vec<A>>();
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
