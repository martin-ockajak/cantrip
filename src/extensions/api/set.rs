use std::hash::Hash;

pub trait SetFunctor<A, B> {
  type C<X>;

  fn map(&self, function: impl FnMut(&A) -> B) -> Self::C<B>
  where
    B: Eq + Hash;
}

pub trait SetMonad<A, B> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>
  where
    A: Eq + Hash;

  fn flat_map<R>(&self, function: impl FnMut(&A) -> R) -> Self::C<B>
  where
    R: IntoIterator<Item = B>,
    B: Eq + Hash;
}

pub trait SetCollection<A: Eq + Hash> {
  type C<X>;

  fn add(self, value: A) -> Self;

  fn delete(self, value: &A) -> Self;

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self;

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self;

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::C<B>
  where
    B: Eq + Hash;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>
  where
    B: Eq + Hash;

  fn merge(self, iterable: impl IntoIterator<Item = A>) -> Self;
}
