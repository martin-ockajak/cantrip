use std::cmp::Ordering;
use std::hash::Hash;

// FIXME - find out how to generalize these for all sequences
/// Indexed collection operations.
///
/// Methods have the following properties:
///
/// - Requires an efficient way to access the collection elements by index
/// - Consumes the collection and its elements
/// - Creates a new collection
///
pub trait Indexed<Item> {
  type This<I>;

  fn distinct(self) -> Self
  where
    Item: Eq + Hash;

  fn distinct_by<K: Eq + Hash>(self, to_key: impl FnMut(&Item) -> K) -> Self;

  fn sorted(self) -> Self
  where
    Item: Ord;

  fn sorted_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self;

  fn sorted_unstable(self) -> Self
  where
    Item: Ord;

  fn sorted_unstable_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self;
}
