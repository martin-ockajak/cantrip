use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::extensions::*;

impl<A> Iterable<A> for Vec<A> {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    any(self.iter(), predicate)
  }

  fn count_by(&self, predicate: impl FnMut(&A) -> bool) -> usize {
    count_by(self.iter(), predicate)
  }

  fn find(&self, mut predicate: impl FnMut(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    fold(self.iter(), init, function)
  }

  fn max_by(&self, mut compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn reduce(&self, function: impl FnMut(&A, &A) -> A) -> Option<A> {
    reduce(self.iter(), function)
  }
}

impl<A> Ordered<A> for Vec<A> {
  fn position(&self, predicate: impl FnMut(&A) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  fn rfind(&self, mut predicate: impl FnMut(&A) -> bool) -> Option<&A> {
    self.iter().rev().find(|&x| predicate(x))
  }

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    self.iter().rfold(init, function)
  }

  fn rposition(&self, predicate: impl FnMut(&A) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<A> Aggregable<A> for Vec<A> {}

impl<A> Collectible<A> for Vec<A> {
  type Root<X> = Vec<X>;
}

impl<A> List<A> for Vec<A> {
  type Root<X> = Vec<X>;

  fn exclude(self, value: &A) -> Self
  where
    A: PartialEq,
  {
    let mut removed = false;
    self
      .into_iter()
      .filter(|x| {
        if removed {
          true
        } else {
          removed = true;
          x != value
        }
      })
      .collect()
  }

  fn distinct(self) -> Self
  where
    A: Eq + Hash,
  {
    let mut occurred: HashSet<&A> = HashSet::new();
    let mut indices: HashSet<usize> = HashSet::new();
    unsafe {
      for index in 0..self.len() {
        let value = self.get_unchecked(index);
        if !occurred.contains(value) {
          indices.insert(index);
          occurred.insert(value);
        }
      }
    }
    self
      .into_iter()
      .enumerate()
      .filter_map(|(index, value)| if indices.contains(&index) { Some(value) } else { None })
      .collect()
  }

  fn distinct_by<K>(self, mut to_key: impl FnMut(&A) -> K) -> Self
  where
    K: Eq + Hash,
  {
    let mut occurred: HashSet<K> = HashSet::new();
    let mut indices: HashSet<usize> = HashSet::new();
    unsafe {
      for index in 0..self.len() {
        let key = to_key(self.get_unchecked(index));
        if !occurred.contains(&key) {
          indices.insert(index);
          occurred.insert(key);
        }
      }
    }
    self
      .into_iter()
      .enumerate()
      .filter_map(|(index, value)| if indices.contains(&index) { Some(value) } else { None })
      .collect()
  }

  fn enumerate(self) -> Self::Root<(usize, A)> {
    self.into_iter().enumerate().collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::Root<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::Root<B>
  where
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }

  fn flatten<B>(self) -> Self::Root<B>
  where
    A: IntoIterator<Item = B>,
  {
    self.into_iter().flatten().collect()
  }

  fn group_by<K>(self, mut to_key: impl FnMut(&A) -> K) -> HashMap<K, Self>
  where
    K: Eq + Hash,
    Self: Sized,
  {
    HashMap::from_pairs(self.into_iter().map(|x| (to_key(&x), x)))
  }

  fn init(self) -> Self {
    let mut iterator = self.into_iter().rev();
    iterator.next();
    iterator.rev().collect()
  }

  fn interleave(self, iterable: impl IntoIterator<Item = A>) -> Self {
    let mut result: Vec<A> = Vec::new();
    for (item1, item2) in self.into_iter().zip(iterable) {
      result.push(item1);
      result.push(item2);
    }
    result
  }

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::Root<B> {
    self.iter().map(function).collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&A) -> Option<B>) -> Self::Root<B> {
    self.iter().map_while(predicate).collect()
  }

  fn put(self, index: usize, element: A) -> Self
    where
      Self: IntoIterator<Item = A>,
  {
    let mut result: Vec<A> = self.into_iter().collect();
    result.insert(index, element);
    result
  }

  fn rev(self) -> Self {
    self.into_iter().rev().collect()
  }

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &A) -> Option<B>) -> Self::Root<B> {
    self.iter().scan(init, function).collect()
  }

  fn sorted(self) -> Self
  where
    A: Ord,
  {
    let mut result = self.into_iter().collect::<Vec<A>>();
    result.sort();
    result
  }

  fn sorted_by(self, compare: impl FnMut(&A, &A) -> Ordering) -> Self {
    let mut result = self.into_iter().collect::<Vec<A>>();
    result.sort_by(compare);
    result
  }

  fn unzip<B, C>(self) -> (Self::Root<B>, Self::Root<C>)
  where
    Self: IntoIterator<Item = (B, C)>,
  {
    self.into_iter().unzip()
  }

  fn zip<I>(self, iterable: I) -> Self::Root<(A, I::Item)>
  where
    I: IntoIterator,
  {
    self.into_iter().zip(iterable).collect()
  }
}

// #[cfg(test)]
// mod tests {
//   use std::collections::HashMap;
//
//   use crate::extensions::*;
//
//   #[quickcheck]
//   fn map(data: Vec<i32>) -> bool {
//     let function = |x: &i32| *x as i64;
//     let result = data.map(function);
//     let expected = data.iter().map(function).collect::<Vec<i64>>();
//     result == expected
//   }
//
//   #[quickcheck]
//   fn filter(data: Vec<i32>) -> bool {
//     let predicate = |x: &i32| x % 2 == 0;
//     let result = data.clone().filter(predicate);
//     let expected = data.iter().filter(|&x| predicate(x)).cloned().collect::<Vec<i32>>();
//     result == expected
//   }
//
//   #[quickcheck]
//   fn group(data: Vec<i32>) -> bool {
//     let key = |x: &i32| x % 2;
//     let result: HashMap<i32, Vec<i32>> = data.clone().group_by(key);
//     let expected = {
//       let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
//       for item in data {
//         let key = key(&item);
//         map.entry(key).and_modify(|mut values| values.push(item)).or_insert(Vec::new());
//       }
//       map
//     };
//     result == expected
//   }
//
//   #[quickcheck]
//   fn fold(data: Vec<i32>) -> bool {
//     let function = |i: i32, x: &i32| i.saturating_add(*x);
//     let result = data.fold(0, function);
//     let expected = data.iter().fold(0, function);
//     result == expected
//   }
//
//   #[test]
//   fn test_x() {
//     [1, 2, 3];
//     &[1, 2, 3][0..];
//     "Test";
//     "Test".to_string();
//     assert_eq!(1, 1)
//   }
// }
