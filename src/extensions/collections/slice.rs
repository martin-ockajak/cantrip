use crate::extensions::{all, any, count_by, fold, Iterable, reduce};
use crate::extensions::{Ordered, Slice};
use std::cmp::{max, min, Ordering};

impl<A> Iterable<A> for [A] {
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

impl<A> Ordered<A> for [A] {
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

impl<A> Slice<A> for [A] {
  fn init(&self) -> &Self {
    &self[0..max(self.len() - 1, 0)]
  }

  fn skip_while(&self, mut predicate: impl FnMut(&A) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[min(index, self.len())..self.len()],
      None => &self[0..0],
    }
  }

  fn tail(&self) -> &Self {
    &self[min(1, self.len())..self.len()]
  }

  fn take_while(&self, mut predicate: impl FnMut(&A) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[0..min(index, self.len())],
      None => &self,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::extensions::*;

  #[quickcheck]
  fn skip(source: Vec<i32>) -> bool {
    let data = source.as_slice();
    data.skip_while(|&x| x == 0);
    true
    // let function = |x: &i32| *x as i64;
    // let result = data.map(function);
    // let expected = data.iter().map(function).collect::<Vec<i64>>();
    // result == expected
  }
}
