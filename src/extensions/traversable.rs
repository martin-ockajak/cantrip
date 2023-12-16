use std::cmp::Ordering;

pub trait Traversable<Item> {
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize;

  fn find(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  fn fold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B;

  fn max_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  fn min_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  fn reduce(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item>;
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

pub(crate) fn reduce<'a, A: 'a>(
  mut iterator: impl Iterator<Item = &'a A>, mut function: impl FnMut(&A, &A) -> A,
) -> Option<A> {
  iterator
    .next()
    .and_then(|value1| iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function(&r, x))))
}
