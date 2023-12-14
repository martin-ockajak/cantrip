use std::cmp::Ordering;
use std::iter::Sum;

pub trait Iterable<A> {
  fn all(&self, predicate: impl FnMut(&A) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut(&A) -> bool) -> bool;

  fn count_by(&self, predicate: impl FnMut(&A) -> bool) -> usize;

  fn find(&self, predicate: impl FnMut(&A) -> bool) -> Option<&A>;

  fn fold<B>(&self, init: B, function: impl FnMut(B, &A) -> B) -> B;

  fn max_by(&self, compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A>;

  fn min_by(&self, compare: impl FnMut(&A, &A) -> Ordering) -> Option<&A>;

  fn reduce(&self, function: impl FnMut(&A, &A) -> A) -> Option<A>;
}
