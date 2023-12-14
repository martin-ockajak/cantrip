use std::hash::Hash;

pub trait MapFunctor<K, V> {
  type C<X, Y>;

  fn map<L, W>(&self, function: impl FnMut((&K, &V)) -> (L, W)) -> Self::C<L, W>
  where
    L: Eq + Hash;
}

pub trait MapMonad<K, V> {
  type C<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V>
  where
    K: Eq + Hash;

  fn flat_map<L, W, R>(&self, function: impl FnMut((&K, &V)) -> R) -> Self::C<L, W>
  where
    L: Eq + Hash,
    R: IntoIterator<Item = (L, W)>;
}

pub trait MapOps<K, V> {
  type C<X, Y>;

  fn add(self, key: K, value: V) -> Self
  where
    K: Eq + Hash;

  fn all(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool;

  fn any(&self, predicate: impl FnMut((&K, &V)) -> bool) -> bool;

  fn delete(self, key: &K) -> Self
  where
    K: Eq + Hash;

  fn diff(self, iterable: impl IntoIterator<Item = K>) -> Self
  where
    K: Eq + Hash;

  fn filter(self, predicate: impl FnMut((&K, &V)) -> bool) -> Self
  where
    K: Eq + Hash;

  fn filter_keys(self, predicate: impl FnMut(&K) -> bool) -> Self
  where
    K: Eq + Hash;

  fn filter_map<L, W>(&self, function: impl FnMut((&K, &V)) -> Option<(L, W)>) -> Self::C<L, W>
  where
    K: Eq + Hash,
    L: Eq + Hash;

  fn filter_values(self, predicate: impl FnMut(&V) -> bool) -> Self
  where
    K: Eq + Hash;

  fn find(&self, predicate: impl FnMut((&K, &V)) -> bool) -> Option<(&K, &V)>;

  fn find_map<B>(&self, function: impl FnMut((&K, &V)) -> Option<B>) -> Option<B>
  where
    K: Eq + Hash,
    B: Eq + Hash;

  fn fold<B>(&self, init: B, function: impl FnMut(B, (&K, &V)) -> B) -> B;

  fn map_keys<L>(self, function: impl FnMut(&K) -> L) -> Self::C<L, V>
  where
    K: Eq + Hash,
    L: Eq + Hash;

  fn map_values<W>(self, function: impl FnMut(&V) -> W) -> Self::C<K, W>
  where
    K: Eq + Hash,
    W: Eq + Hash;

  fn merge(self, iterable: impl IntoIterator<Item = (K, V)>) -> Self
  where
    K: Eq + Hash;

  fn reduce(&self, function: impl FnMut((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)>;

  fn rfold<B>(&self, init: B, function: impl FnMut(B, (&K, &V)) -> B) -> B;
}
