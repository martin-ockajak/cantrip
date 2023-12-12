use crate::extensions::traits::{Functor, Iterable};

impl<A, R> Functor<A, R> for Vec<A> {
  type C<X> = Vec<R>;
  fn map<F>(&self, function: F) -> Self::C<R> where F: Fn(&A) -> R {
    self.iter().map(function).collect()
  }
}

impl<A> Iterable<A> for Vec<A> {
  fn filter<P>(&self, predicate: P) -> Self where P: Fn(&A) -> bool, A: Clone {
    self.iter().filter(|&x| predicate(x)).cloned().collect()
  }

  fn fold<B, F>(&self, init: B, function: F) -> B where F: Fn(B, &A) -> B {
    self.iter().fold(init, function)
  }
}

pub fn add_vec<T>(values: &[T], value: &T) -> Vec<T>
  where
    T: Clone,
{
  let mut result = values.to_vec();
  result.push(value.clone());
  result
}

pub fn remove_vec<T>(values: &[T], value: &T) -> Vec<T>
  where
    T: Clone + PartialEq,
{
  values.iter().filter(|x| x != &value).cloned().collect()
}

pub fn merge_vec<T>(values1: &[T], values2: &[T]) -> Vec<T>
  where
    T: Clone,
{
  [values1, values2].concat().to_vec()
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
