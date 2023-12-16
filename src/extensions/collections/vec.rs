use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::RangeBounds;

use crate::extensions::util::multimap::MultiMap;
use crate::extensions::*;

impl<T> Iterable for Vec<T> {
  type Item<'c> = &'c T
    where
      T: 'c;

  type Iterator<'c> = Iter<'c, T>
    where
      T: 'c;

  fn iterator<'c>(&'c self) -> Self::Iterator<'c> {
    Iter { collection: self }
  }
}

impl<Item> Traversable<Item> for Vec<Item> {
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    all(self.iter(), predicate)
  }

  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool {
    any(self.iter(), predicate)
  }

  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize {
    count_by(self.iter(), predicate)
  }

  fn find(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B {
    fold(self.iter(), init, function)
  }

  fn max_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn reduce(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item> {
    reduce(self.iter(), function)
  }
}

impl<Item> Ordered<Item> for Vec<Item> {
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().position(predicate)
  }

  fn rfind(&self, mut predicate: impl FnMut(&Item) -> bool) -> Option<&Item> {
    self.iter().rev().find(|&x| predicate(x))
  }

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B {
    self.iter().rfold(init, function)
  }

  fn rposition(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize> {
    self.iter().rposition(predicate)
  }
}

impl<Item> Aggregable<Item> for Vec<Item> {}

impl<Item> Collectible<Item> for Vec<Item> {
  type This<I> = Vec<I>;
}

impl<Item> List<Item> for Vec<Item> {
  type This<I> = Vec<I>;

  fn exclude(self, value: &Item) -> Self
  where
    Item: PartialEq,
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
    Item: Eq + Hash,
  {
    let mut occurred: HashSet<&Item> = HashSet::new();
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

  fn distinct_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
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

  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(function).collect()
  }

  fn group_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
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

  fn interleave(self, iterable: impl IntoIterator<Item = Item>) -> Self {
    let mut result: Vec<Item> = Vec::new();
    for (item1, item2) in self.into_iter().zip(iterable) {
      result.push(item1);
      result.push(item2);
    }
    result
  }

  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B> {
    self.iter().map(function).collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B> {
    self.iter().map_while(predicate).collect()
  }

  fn put(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.insert(index, element);
    result
  }

  fn rev(self) -> Self {
    self.into_iter().rev().collect()
  }

  fn replace(self, range: impl RangeBounds<usize>, replace_with: Self) -> Self
  where
    Self: IntoIterator<Item = Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.splice(range, replace_with);
    result
  }

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B> {
    self.iter().scan(init, function).collect()
  }

  fn sorted(self) -> Self
  where
    Item: Ord,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort();
    result
  }

  fn sorted_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by(compare);
    result
  }

  fn sorted_unstable(self) -> Self
  where
    Item: Ord,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable();
    result
  }

  fn sorted_unstable_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable_by(compare);
    result
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
