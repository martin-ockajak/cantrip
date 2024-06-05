use std::fmt::Debug;

use cantrip::{Collectible, Iterable, Reversible, Sequence, Traversable};

use crate::extensions::collectible::test_collectible;
use crate::extensions::reversible::test_reversible;
use crate::extensions::traversable::test_traversable;
use crate::extensions::sequence::test_sequence;

pub fn test_collectible_traits<'a, C>()
  where
    C: Traversable<i64>
    + Collectible<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + PartialEq
    + Debug
    + 'a,
{
  test_traversable::<C>();
  // test_collectible::<C>();
}

pub fn test_sequence_traits<'a, C>()
where
  C: Traversable<i64>
    + Collectible<i64>
    + Reversible<i64>
    + Sequence<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + PartialEq
    + Debug
    + 'a,
{
  test_traversable::<C>();
  test_collectible::<C>();
  test_reversible::<C>();
  test_sequence::<C>();
}
