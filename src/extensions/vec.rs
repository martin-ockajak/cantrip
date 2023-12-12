use crate::extensions::traits::Functor;

impl<A, R> Functor<A, R> for &Vec<A> {
  type T<X> = Vec<R>;
  fn map<F>(self, f: F) -> Self::T<R>
    where
      F: Fn(&A) -> R
  {
    self.iter().map(f).collect()
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
    // let function = |x| *x as i64;
    let result = data.map(|x| *x as i64);
    let expected = data.iter().map(|x| *x as i64).collect::<Vec<i64>>();
    result == expected
  }

  #[test]
  fn test_x() {
    assert_eq!(1, 1)
  }
}
