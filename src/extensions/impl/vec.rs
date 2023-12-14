use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use crate::extensions::{Aggregable, MultiMap, List, Ordered};
use crate::extensions::api::iterable::Iterable;

impl<A> Iterable<A> for Vec<A> {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn count_by(&self, mut predicate: impl FnMut(&A) -> bool) -> usize {
    self.iter().filter(|&x| predicate(x)).count()
  }

  fn find(&self, mut predicate: impl FnMut(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn max_by(&self, mut compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A> {
    self.iter().max_by(|&x, &y| compare(x, y))
  }

  fn min_by(&self, mut compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A> {
    self.iter().min_by(|&x, &y| compare(x, y))
  }

  fn reduce(&self, mut function: impl FnMut(&A, &A) -> A) -> Option<A> {
    let mut iterator = self.iter();
    match iterator.next() {
      Some(value1) => match iterator.next() {
        Some(value2) => Some(iterator.fold(function(value1, value2), |r, x| function(&r, x))),
        _ => None,
      },
      _ => None,
    }
  }
}

impl<A> Ordered<A> for Vec<A> {
  fn head(&self) -> Option<&A> {
    self.get(0)
  }

  fn last(&self) -> Option<&A> {
    self.get(self.len() - 1)
  }

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

impl<A> Aggregable<A> for Vec<A> {
  fn sum(self) -> A
  where
    A: Sum,
  {
    self.into_iter().sum()
  }

  fn product(self) -> A
  where
    A: Product,
  {
    self.into_iter().product()
  }
}

impl<A> List<A> for Vec<A> {
  type C<X> = Vec<X>;

  fn add(self, value: A) -> Self {
    self.into_iter().chain(iter::once(value)).collect()
  }

  fn concat(self, iterable: impl IntoIterator<Item = A>) -> Self {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn delete(self, value: &A) -> Self
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

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash,
  {
    let mut removed: HashSet<A> = HashSet::new();
    removed.extend(iterable.into_iter());
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  fn distinct(self) -> Self
  where
    A: Eq + Hash,
  {
    let mut occured: HashSet<&A> = HashSet::new();
    let mut indices: HashSet<usize> = HashSet::new();
    unsafe {
      for index in 0..self.len() {
        let value = self.get_unchecked(index);
        if !occured.contains(value) {
          indices.insert(index);
        } else {
          occured.insert(value);
        }
      }
    }
    self
      .into_iter()
      .enumerate()
      .filter_map(|(index, value)| if indices.contains(&index) { Some(value) } else { None })
      .collect()
  }

  fn enumerate(self) -> Self::C<(usize, A)> {
    (0..self.len()).zip(self.into_iter()).collect()
  }

  fn group_by<K, V>(self, mut group_key: impl FnMut(&A) -> K) -> V
  where
    K: Eq + Hash,
    V: MultiMap<K, Self::C<A>> + Default,
  {
    let mut result: V = V::default();
    for item in self.into_iter() {
      let key = group_key(&item);
      result.add(key, item);
    }
    result
  }

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self {
    self.into_iter().filter(predicate).collect()
  }

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::C<B> {
    self.iter().filter_map(function).collect()
  }

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B> {
    self.iter().find_map(function)
  }

  fn flat_map<B, R>(&self, mut function: impl FnMut(&A) -> R) -> Self::C<B>
  where
    R: IntoIterator<Item = B>,
  {
    self.iter().flat_map(|x| function(x).into_iter()).collect()
  }

  fn flatten<B>(self) -> Self::C<B>
  where
    A: IntoIterator<Item = B>,
  {
    self.into_iter().flatten().collect()
  }

  fn init(self) -> Self {
    let mut iterator = self.into_iter().rev();
    iterator.next();
    iterator.rev().collect()
  }

  fn intersect(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash,
  {
    let mut retained: HashSet<A> = HashSet::new();
    retained.extend(iterable.into_iter());
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::C<B> {
    self.iter().map(function).collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&A) -> Option<B>) -> Self::C<B> {
    self.iter().map_while(predicate).collect()
  }

  fn partition(self, predicate: impl FnMut(&A) -> bool) -> (Self, Self)
  where
    Self: Sized,
  {
    self.into_iter().partition(predicate)
  }

  fn rev(self) -> Self {
    self.into_iter().rev().collect()
  }

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &A) -> Option<B>) -> Self::C<B> {
    self.iter().scan(init, function).collect()
  }

  fn skip(self, n: usize) -> Self {
    self.into_iter().skip(n).collect()
  }

  fn skip_while(self, predicate: impl FnMut(&A) -> bool) -> Self {
    self.into_iter().skip_while(predicate).collect()
  }

  fn tail(self) -> Self {
    let mut iterator = self.into_iter();
    iterator.next();
    iterator.collect()
  }

  fn take(self, n: usize) -> Self {
    self.into_iter().take(n).collect()
  }

  fn take_while(self, predicate: impl FnMut(&A) -> bool) -> Self {
    self.into_iter().take_while(predicate).collect()
  }

  fn unit(value: A) -> Self::C<A> {
    iter::once(value).collect()
  }

  fn unzip<B, C, FromB, FromC>(self) -> (Self::C<B>, Self::C<C>)
  where
    Self: IntoIterator<Item = (B, C)>,
  {
    self.into_iter().unzip()
  }

  fn zip<I>(self, iterable: I) -> Self::C<(A, I::Item)>
  where
    I: IntoIterator,
  {
    self.into_iter().zip(iterable.into_iter()).collect()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::extensions::*;

  #[quickcheck]
  fn map(data: Vec<i32>) -> bool {
    let function = |x: &i32| *x as i64;
    let result = data.map(function);
    let expected = data.iter().map(function).collect::<Vec<i64>>();
    result == expected
  }

  #[quickcheck]
  fn filter(data: Vec<i32>) -> bool {
    let predicate = |x: &i32| x % 2 == 0;
    let result = data.clone().filter(predicate);
    let expected = data.iter().filter(|&x| predicate(x)).cloned().collect::<Vec<i32>>();
    result == expected
  }

  #[quickcheck]
  fn group(data: Vec<i32>) -> bool {
    let key = |x: &i32| x % 2;
    let result: HashMap<i32, Vec<i32>> = data.clone().group_by(key);
    let expected = {
      let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
      for item in data.into_iter() {
        let key = key(&item);
        map.entry(key).and_modify(|mut values| values.push(item)).or_insert(Vec::new());
      }
      map
    };
    result == expected
  }

  #[quickcheck]
  fn fold(data: Vec<i32>) -> bool {
    let function = |i: i32, x: &i32| i.saturating_add(*x);
    let result = data.fold(0, function);
    let expected = data.iter().fold(0, function);
    result == expected
  }

  #[test]
  fn test_x() {
    [1, 2, 3];
    &[1, 2, 3][0..];
    "Test";
    "Test".to_string();
    assert_eq!(1, 1)
  }
}
