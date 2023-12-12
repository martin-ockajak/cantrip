// use std::collections::HashSet;
// use std::iter;
//
// use crate::extensions::traits::{Collection, Functor, Iterable};
//
// impl<A, R> Functor<A, R> for HashSet<A> {
//   type C<X> = HashSet<R>;
//   fn map<F>(&self, function: F) -> Self::C<R> where F: Fn(&A) -> R {
//     self.iter().map(function).collect()
//   }
// }
//
// impl<A> Iterable<A> for HashSet<A> {
//   fn filter<P>(&self, predicate: P) -> Self where P: Fn(&A) -> bool, A: Clone {
//     self.iter().filter(|&x| predicate(x)).cloned().collect()
//   }
//
//   fn fold<B, F>(&self, init: B, function: F) -> B where F: Fn(B, &A) -> B {
//     self.iter().fold(init, function)
//   }
// }
//
// impl<A: Clone> Collection<A> for HashSet<A> {
//   fn add(&self, value: A) -> Self {
//     self.iter().chain(iter::once(&value)).cloned().collect()
//   }
//
//   fn add_all<I>(&self, values: &I) -> Self where I: Clone + IntoIterator<Item = A> {
//     self.iter().cloned().chain(values.clone().into_iter()).collect()
//   }
//
//   fn remove(&self, value: A) -> Self where A: PartialEq {
//     self.iter().filter(|&x| x != &value).cloned().collect()
//   }
//
//   fn remove_all<I>(&self, values: &I) -> Self where A: PartialEq, I: Clone + IntoIterator<Item = A> {
//     let removed = values.clone().into_iter().collect::<Vec<A>>();
//     self.iter().filter(|&x| removed.contains(&x)).cloned().collect()
//   }
// }
//
//
// #[cfg(test)]
// mod tests {
//   use std::collections::HashSet;
//   use crate::extensions::traits::*;
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
