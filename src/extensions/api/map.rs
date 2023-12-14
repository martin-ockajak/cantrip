use std::hash::Hash;

pub trait MapFunctor<K, V, L, W> {
  type C<X, Y>;

  fn map(&self, function: impl Fn((&K, &V)) -> (L, W)) -> Self::C<L, W>
  where
    L: Eq + Hash;
}

pub trait MapMonad<K, V, L, W> {
  type C<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V>
  where
    K: Eq + Hash;

  fn flat_map<R>(&self, function: impl Fn((&K, &V)) -> R) -> Self::C<L, W>
  where
    R: IntoIterator<Item = (L, W)> + Clone,
    L: Eq + Hash;
}

pub trait MapIterable<K, V> {
  fn all(&self, predicate: impl Fn((&K, &V)) -> bool) -> bool;

  fn any(&self, predicate: impl Fn((&K, &V)) -> bool) -> bool;

  fn find(&self, predicate: impl Fn((&K, &V)) -> bool) -> Option<(&K, &V)>
  where
    K: Clone,
    V: Clone;

  fn fold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B;

  fn reduce(&self, function: impl Fn((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)>
  where
    K: Clone,
    V: Clone;

  fn rfold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B;
}

pub trait MapCollection<K: Eq + Hash + Clone, V: Clone> {
  type C<X, Y>;

  fn add(self, key: K, value: V) -> Self;

  fn delete(self, key: &K) -> Self;

  fn diff(self, iterable: impl IntoIterator<Item = K>) -> Self;

  fn filter(&self, predicate: impl Fn((&K, &V)) -> bool) -> Self;

  fn filter_map<L, W>(&self, function: impl Fn((&K, &V)) -> Option<(L, W)>) -> Self::C<L, W>
  where
    L: Eq + Hash;

  fn find_map<B>(&self, function: impl Fn((&K, &V)) -> Option<B>) -> Option<B>
  where
    B: Eq + Hash;

  fn map_keys<L>(&self, function: impl Fn(&K) -> L) -> Self::C<L, V>
  where
    L: Eq + Hash;

  fn map_values<W>(&self, function: impl Fn(&V) -> W) -> Self::C<K, W>
  where
    W: Eq + Hash;

  fn merge(&self, iterable: &(impl IntoIterator<Item = (K, V)> + Clone)) -> Self;
}
