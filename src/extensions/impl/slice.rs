use std::cmp::{max, min};
use crate::extensions::api::iterable::IterableOps;
use crate::extensions::{OrderedOps, SliceOps};

impl<A> IterableOps<A> for [A] {
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

  fn rfold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B {
    self.iter().rfold(init, function)
  }
}

impl<A> OrderedOps<A> for [A] {
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
}

impl<A> SliceOps<A> for [A] {
  fn init(&self) -> &Self {
    &self[0..max(self.len() - 1, 0)]
  }

  fn tail(&self) -> &Self {
    &self[min(1, self.len())..self.len()]
  }
}

#[cfg(test)]
mod tests {}
