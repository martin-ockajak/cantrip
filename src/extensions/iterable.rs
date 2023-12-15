use std::cmp::Ordering;

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

pub(crate) fn all<'a, A: 'a>(mut iterator: impl Iterator<Item = &'a A>, predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.all(predicate)
}

pub(crate) fn any<'a, A: 'a>(mut iterator: impl Iterator<Item = &'a A>, predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.any(predicate)
}

pub(crate) fn count_by<'a, A: 'a>(
  iterator: impl Iterator<Item = &'a A>, mut predicate: impl FnMut(&A) -> bool,
) -> usize {
  iterator.filter(|&x| predicate(x)).count()
}

pub(crate) fn fold<'a, A: 'a, B>(
  iterator: impl Iterator<Item = &'a A>, init: B, function: impl FnMut(B, &A) -> B,
) -> B {
  iterator.fold(init, function)
}

pub(crate) fn reduce<'a, A: 'a>(mut iterator: impl Iterator<Item = &'a A>, mut function: impl FnMut(&A, &A) -> A) -> Option<A> {
  match iterator.next() {
    Some(value1) => match iterator.next() {
      Some(value2) => Some(iterator.fold(function(value1, value2), |r, x| function(&r, x))),
      _ => None,
    },
    _ => None,
  }
}
