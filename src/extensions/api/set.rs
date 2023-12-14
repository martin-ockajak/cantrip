use std::hash::Hash;

pub trait SetFunctor<A> {
  type C<X>;

  fn map<B>(&self, function: impl FnMut(&A) -> B) -> Self::C<B>
  where
    B: Eq + Hash;
}

pub trait SetMonad<A> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>
  where
    A: Eq + Hash;

  fn flat_map<B, R>(&self, function: impl FnMut(&A) -> R) -> Self::C<B>
  where
    B: Eq + Hash,
    R: IntoIterator<Item = B>;
}

pub trait SetOps<A> {
  type C<X>;

  fn add(self, value: A) -> Self
  where
    A: Eq + Hash;

  fn concat(self, iterable: impl IntoIterator<Item = A>) -> Self
    where
      A: Eq + Hash;

  fn delete(self, value: &A) -> Self
  where
    A: Eq + Hash;

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash;

  fn filter(self, predicate: impl FnMut(&A) -> bool) -> Self
  where
    A: Eq + Hash;

  fn filter_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Self::C<B>
  where
    A: Eq + Hash,
    B: Eq + Hash;

  fn find_map<B>(&self, function: impl FnMut(&A) -> Option<B>) -> Option<B>
  where
    A: Eq + Hash,
    B: Eq + Hash;

  fn intersect(self, iterable: impl IntoIterator<Item = A>) -> Self
    where
      A: Eq + Hash;
}
