pub fn add(left: i32, right: i32) -> i32 {
  left + right
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::HashMap;
use std::hash::Hash;

trait Functor<A, R> {
  type T<X>;
  fn map<F>(self, f: F) -> Self::T<R>
    where
      F: Fn(&A) -> R;
}

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

// pub fn add_set<T>(values: &HashSet<T>, value: &T) -> HashSet<T>
// where
//   T: Clone + Eq + Hash,
// {
//   let mut result = values.clone();
//   result.insert(value.clone());
//   result
// }
//
// pub fn remove_set<T>(values: &HashSet<T>, value: &T) -> HashSet<T>
// where
//   T: Clone + Eq + Hash,
// {
//   values.iter().filter(|x| x != &value).cloned().collect()
// }
//
// pub fn add_map<K, V>(values: &HashMap<K, V>, key: &K, value: &V) -> HashMap<K, V>
// where
//   K: Clone + Eq + Hash,
//   V: Clone,
// {
//   values.iter().map(|(k, v)| (k.clone(), v.clone())).chain([(key.clone(), value.clone())].into_iter()).collect()
// }

pub fn remove_all_map<K, V>(values: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
  where
    K: Clone + Eq + Hash,
    V: Clone,
{
  values.iter().filter(|(k, _)| !keys.contains(k)).map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn remove_map<K, V>(values: &HashMap<K, V>, key: &K) -> HashMap<K, V>
  where
    K: Clone + Eq + Hash,
    V: Clone,
{
  values.iter().filter(|(k, _)| k != &key).map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn merge_map<K, V>(values1: &HashMap<K, V>, values2: &HashMap<K, V>) -> HashMap<K, V>
  where
    K: Clone + Eq + Hash,
    V: Clone,
{
  values1.iter().chain(values2.iter()).map(|(k, v)| (k.clone(), v.clone())).collect()
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
