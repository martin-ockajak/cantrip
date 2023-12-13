use std::iter;

use crate::extensions::traits::{Collection, Functor, AggregateIterable, Monad, Iterable};

impl<A, B> Functor<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B> {
    self.iter().map(function).collect()
  }
}

impl<A, B> Monad<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(value: A) -> Self::C<A> where A: Clone {
    iter::once(value).collect()
  }

  fn flat_map(&self, function: impl Fn(&A) -> Self::C<B>) -> Self::C<B> {
    self.iter().flat_map(function).collect()
  }
}

impl<A> AggregateIterable<A> for Vec<A> {
  fn all(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl Fn(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A> where A: Clone {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn reduce(&self, function: impl Fn(&A, &A) -> A) -> Option<A> where A: Clone {
    let mut iterator = self.iter();
    iterator.next().and_then(|init| Some(iterator.fold(init.clone(), |r, x| function(&r, x))))
  }

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    self.iter().rfold(init, function)
  }
}

impl<A: Clone> Iterable<A> for Vec<A> {
  type C<X> = Vec<X>;

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  fn enumerate(&self) -> Self::C<(usize, A)> {
    (0..self.len()).zip(self.iter().cloned()).collect()
  }

  fn filter_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn map_while<B>(&self, predicate: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().map_while(predicate).collect()
  }

  fn partition(&self, predicate: impl Fn(&A) -> bool) -> (Self, Self) where Self: Sized {
    self.iter().cloned().partition(predicate)
  }

  fn zip<I>(&self, iterable: &I) -> Self::C<(A, I::Item)> where I: Clone + IntoIterator {
    self.iter().cloned().zip(iterable.clone().into_iter()).collect()
  }
}

impl<A: Clone> Collection<A> for Vec<A> {
  fn add(&self, value: A) -> Self {
    self.iter().chain(iter::once(&value)).cloned().collect()
  }

  fn add_all(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self {
    self.iter().cloned().chain(iterable.clone().into_iter()).collect()
  }

  fn remove(&self, value: A) -> Self where A: PartialEq {
    self.iter().filter(|&x| x != &value).cloned().collect()
  }

  fn remove_all(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self where A: PartialEq {
    let removed = iterable.clone().into_iter().collect::<Vec<A>>();
    self.iter().filter(|&x| removed.contains(&x)).cloned().collect()
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
