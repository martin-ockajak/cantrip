use std::hash::Hash;

pub trait MapFunctor<K, V, L: Eq + Hash, W> {
  type C<X, Y>;

  fn map(&self, function: impl Fn((&K, &V)) -> (L, W)) -> Self::C<L, W>;
}

pub trait MapMonad<K, V, L: Eq + Hash, W> {
  type C<X, Y>;

  fn unit(key: K, value: V) -> Self::C<K, V>
    where
     K: Eq + Hash;

  fn flat_map(&self, function: impl Fn((&K, &V)) -> Self::C<L, W>) -> Self::C<L, W>;
}

pub trait MapIterable<K, V> {
  // fn all(&self, predicate: impl Fn(&A) -> bool) -> bool;
  //
  // fn any(&self, predicate: impl Fn(&A) -> bool) -> bool;
  //
  // fn find(&self, predicate: impl Fn(&A) -> bool) -> Option<&A>
  //   where
  //     A: Clone;

  fn fold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B;

  fn reduce(&self, function: impl Fn((&K, &V), (&K, &V)) -> (K, V)) -> Option<(K, V)>
    where
      K: Clone,
      V: Clone;

  fn rfold<B>(&self, init: B, function: impl Fn(B, (&K, &V)) -> B) -> B;
}
