use crate::extensions::traits::{Functor, Iterable};

impl<A, R> Functor<A, R> for Vec<A> {
  type C<X> = Vec<R>;
  fn map<F>(self, f: F) -> Self::C<R> where F: Fn(&A) -> R {
    self.iter().map(f).collect()
  }
}

impl<A> Iterable<A> for Vec<A> {
  fn filter<F>(self, f: F) -> Self where F: Fn(&A) -> bool, A: Clone {
    self.iter().filter(|&x| f(x)).map(|x| x.clone()).collect()
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
  use super::*;

  #[quickcheck]
  fn test_functor_vec(data: Vec<i32>) -> bool {
    let function = |x: &i32| *x as i64;
    let result = data.clone().map(function);
    let expected = data.clone().iter().map(function).collect::<Vec<i64>>();
    result == expected
  }

  #[quickcheck]
  fn test_iterable_vec(data: Vec<i32>) -> bool {
    let function = |x: &i32| x % 2 == 0;
    let result = data.clone().filter(function);
    let expected = data.clone().iter().filter(|&x| function(x)).map(|x| x.clone()).collect::<Vec<i32>>();
    result == expected
  }

  #[test]
  fn test_x() {
    assert_eq!(1, 1)
  }
}
