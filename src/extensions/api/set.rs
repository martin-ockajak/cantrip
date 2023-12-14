use std::hash::Hash;

pub trait SetFunctor<A, B> {
  type C<X>;

  fn map(&self, function: impl Fn(&A) -> B) -> Self::C<B>
  where
    B: Eq + Hash;
}

pub trait SetMonad<A, B> {
  type C<X>;

  fn unit(value: A) -> Self::C<A>
  where
    A: Clone + Eq + Hash;

  fn flat_map<R>(&self, function: impl Fn(&A) -> R) -> Self::C<B>
  where
    R: IntoIterator<Item = B> + Clone,
    B: Eq + Hash;
}

pub trait SetCollection<A: Eq + Hash + Clone> {
  type C<X>;

  fn add(&self, value: A) -> Self;

  fn delete(&self, value: &A) -> Self;

  fn diff(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self;

  fn filter(&self, predicate: impl Fn(&A) -> bool) -> Self;

  fn filter_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Self::C<B>
  where
    B: Eq + Hash;

  fn find_map<B>(&self, function: impl Fn(&A) -> Option<B>) -> Option<B>
  where
    B: Eq + Hash;

  fn merge(&self, iterable: &(impl IntoIterator<Item = A> + Clone)) -> Self;
}
