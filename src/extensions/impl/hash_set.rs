use std::collections::HashSet;
use std::hash::Hash;
use std::iter;

use crate::extensions::api::base::Iterable;
use crate::extensions::api::set::{SetFunctor, SetMonad, SetCollection};

impl<A, B: Eq + Hash> SetFunctor<A, B> for HashSet<A> {
  type C<X> = HashSet<B>;

  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B> {
    self.iter().map(function).collect()
  }
}

impl<A, B: Eq + Hash> SetMonad<A, B> for Vec<A> {
  type C<X> = Vec<X>;

  fn unit(value: A) -> Self::C<A> where A: Clone + Eq + Hash {
    iter::once(value).collect()
  }

  fn flat_map<R>(&self, function: impl Fn(&A) -> R) -> Self::C<B> where R: IntoIterator<Item = B> + Clone {
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
    let values = self.iter().collect::<Vec<&A>>();
    values.iter().rfold(init, |r, x| function(r, x))
  }
}

impl<A: Eq + Hash + Clone> SetCollection<A> for HashSet<A> {
  type C<X> = HashSet<X>;

  fn add(&self, value: A) -> Self {
    self.iter().chain(iter::once(&value)).cloned().collect()
  }

  fn delete(&self, value: A) -> Self {
    self.iter().filter(|&x| x != &value).cloned().collect()
  }

  fn diff(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self {
    let mut result = self.clone();
    for item in iterable.clone().into_iter() {
      result.remove(&item);
    }
    result
  }

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  fn filter_map<B: Eq + Hash>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B: Eq + Hash>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  // fn merge(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self {
  //   let other = iterable.clone().into_iter().collect();
  //   self.union(&other).collect()
  // }
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
