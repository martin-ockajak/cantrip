use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

use crate::extensions::traits::{EqFunctor, EqMonad, AggregateIterable, EqIterable};

impl<A: Eq + Hash, B: Eq + Hash> EqFunctor<A, B> for HashSet<A> {
  type C<X> = HashSet<B>;

  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B> {
    self.iter().map(function).collect()
  }
}

impl<A: Eq + Hash, B: Eq + Hash> EqMonad<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(value: A) -> Self::C<A> where A: Clone {
    iter::once(value).collect()
  }

  fn flat_map(&self, function: impl Fn(&A) -> Self::C<B>) -> Self::C<B> {
    self.iter().flat_map(function).collect()
  }
}

impl<A> AggregateIterable<A> for HashSet<A> {
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

  fn rfold<B>(&self, init: B, function: impl Fn(B, &A) -> B) -> B {
    let values = self.iter().collect::<Vec<&A>>();
    values.iter().rfold(init, |r, x| function(r, x))
  }
}

impl<A: Eq + Hash + Clone> EqIterable<A> for HashSet<A> {
  type C<X> = HashSet<X>;

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  fn filter_map<B: Eq + Hash>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B: Eq + Hash>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn map_while<B: Eq + Hash>(&self, predicate: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().map_while(predicate).collect()
  }

  fn partition(&self, predicate: impl Fn(&A) -> bool) -> (Self, Self) where Self: Sized {
    self.iter().cloned().partition(predicate)
  }
}

// #[cfg(test)]
// mod tests {
//   use std::collections::HashSet;
//   use crate::extensions::*;
//
//   #[quickcheck]
//   fn test_map_hash_set(data: HashSet<i32>) -> bool {
//     let function = |x: &i32| *x as i64;
//     let result = data.map(function);
//     let expected = data.iter().map(function).collect::<HashSet<i64>>();
//     result == expected
//   }
//
//   #[quickcheck]
//   fn test_filter_hash_set(data: HashSet<i32>) -> bool {
//     let predicate = |x: &i32| x % 2 == 0;
//     let function = |i: i32, x: &i32| i.saturating_add(*x);
//     let result = data.filter(predicate);
//     let expected = data.iter().filter(|&x| predicate(x)).cloned().collect::<HashSet<i32>>();
//     result == expected
//   }
//
//   #[quickcheck]
//   fn test_fold_hash_set(data: HashSet<i32>) -> bool {
//     let function = |i: i32, x: &i32| i.saturating_add(*x);
//     let result = data.fold(0, function);
//     let expected = data.iter().fold(0, function);
//     result == expected
//   }
//
//   #[test]
//   fn test_x() {
//     assert_eq!(1, 1)
//   }
// }
