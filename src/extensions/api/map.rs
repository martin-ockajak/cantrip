use std::hash::Hash;

pub trait Functor<K, V, L: Eq + Hash, W: Eq + Hash> {
  type C<X, Y>;

  fn map(&self, function: impl Fn((&K, &V)) -> (L, W)) -> Self::C<L, W>;
}

pub trait Monad<K, V, L: Eq + Hash, W: Eq + Hash> {
  type C<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V>;

  fn flat_map(&self, function: impl Fn((&K, &V)) -> Self::C<L, W>) -> Self::C<L, W>;
}
