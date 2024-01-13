use std::fmt::Debug;

use cantrip::{Collectible, Iterable, Traversable};

use crate::extensions::collectible::test_collectible;
use crate::extensions::traversable::test_traversable;

pub fn test_vector<'a, C>()
where
  C: 'a
    + Traversable<i64>
    + Collectible<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + PartialEq
    + Debug,
{
  test_traversable::<C>();
  test_collectible::<C>();
}
