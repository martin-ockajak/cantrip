use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub trait Collectible<A> {
  type Root<X>;

  fn diff(self, iterable: impl IntoIterator<Item = A>) -> Self
    where
      A: Eq + Hash,
      Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    let mut removed: HashSet<A> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  fn intersect(self, iterable: impl IntoIterator<Item = A>) -> Self
  where
    A: Eq + Hash,
    Self: IntoIterator<Item = A> + Sized + FromIterator<A>,
  {
    let mut retained: HashSet<A> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }
}
