use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub trait Collectible<Aff> {
  type This<I>;

  fn diff(self, iterable: impl IntoIterator<Item = Aff>) -> Self
    where
      Aff: Eq + Hash,
      Self: IntoIterator<Item = Aff> + Sized + FromIterator<Aff>,
  {
    let mut removed: HashSet<Aff> = HashSet::new();
    removed.extend(iterable);
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  fn intersect(self, iterable: impl IntoIterator<Item = Aff>) -> Self
  where
    Aff: Eq + Hash,
    Self: IntoIterator<Item = Aff> + Sized + FromIterator<Aff>,
  {
    let mut retained: HashSet<Aff> = HashSet::new();
    retained.extend(iterable);
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }
}
