use std::cmp::{max, min};
use crate::extensions::api::iterable::Iterable;
use crate::extensions::{Ordered, Slice};

impl<A> Iterable<A> for [A] {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    self.iter().all(predicate)
  }

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool {
    self.iter().any(predicate)
  }

  fn find(&self, mut predicate: impl FnMut(&A) -> bool) -> Option<&A> {
    self.iter().find(|&x| predicate(x))
  }

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    self.iter().fold(init, function)
  }

  fn reduce(&self, mut function: impl FnMut(&A, &A) -> A) -> Option<A>
  {
    let mut iterator = self.iter();
    match iterator.next() {
      Some(value1) => {
        match iterator.next() {
          Some(value2) => {
            Some(iterator.fold(function(value1, value2), |r, x| function(&r, x)))
          },
          _ => None
        }
      },
      _ => None
    }
  }
}

impl<A> Ordered<A> for [A] {
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

impl<A> Slice<A> for [A] {
  fn init(&self) -> &Self {
    &self[0..max(self.len() - 1, 0)]
  }

  fn skip(&self, n: usize) -> &Self {
    &self[min(n, self.len())..self.len()]
  }

  fn skip_while(&self, mut predicate: impl FnMut(&A) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[min(index, self.len())..self.len()],
      None => &self[0..0]
    }
  }

  fn tail(&self) -> &Self {
    &self[min(1, self.len())..self.len()]
  }

  // fn take(&self, n: usize) -> &Self {
  //   &self[0..min(n, self.len())]
  // }

  fn take_while(&self, mut predicate: impl FnMut(&A) -> bool) -> &Self {
    match self.iter().position(|x| !predicate(x)) {
      Some(index) => &self[0..min(index, self.len())],
      None => &self
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::extensions::*;

  #[quickcheck]
  fn skip(source: Vec<i32>) -> bool {
    let data = source.as_slice();
    data.skip(1);
    true
    // let function = |x: &i32| *x as i64;
    // let result = data.map(function);
    // let expected = data.iter().map(function).collect::<Vec<i64>>();
    // result == expected
  }
}
