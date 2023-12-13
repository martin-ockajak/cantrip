use std::hash::Hash;

pub trait Functor<K, V, L: Eq + Hash, W: Eq + Hash> {
  type C<X, Y>;

  fn map(&self, function: impl Fn((&K, &V)) -> (L, W)) -> Self::C<L, W>;
}
