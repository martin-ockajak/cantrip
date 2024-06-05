use std::fmt::Debug;

use cantrip::{Collectible, Iterable, Reversible, Sequence, Slice, Traversable};

use crate::extensions::collectible::test_collectible;
use crate::extensions::reversible::test_reversible;
use crate::extensions::traversable::test_traversable;
use crate::extensions::sequence::test_sequence;
use crate::extensions::slice::test_slice;
use crate::extensions::util::Equal;

pub fn test_collectible_traits<'a, C>()
  where
    C: Traversable<i64>
    + Collectible<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + Equal
    + Debug
    + 'a,
{
  test_traversable::<C>();
  test_collectible::<C>(false);
}

pub fn test_slice_traits<'a, C>()
  where
    C: Traversable<i64>
    + Collectible<i64>
    + Reversible<i64>
    + Slice<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + Equal
    + PartialEq
    + Debug
    + 'a,
{
  test_traversable::<C>();
  test_collectible::<C>(true);
  test_reversible::<C>();
  test_slice::<C>();
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
    + Equal
    + Debug
    + 'a,
{
  test_traversable::<C>();
  test_collectible::<C>(true);
  test_reversible::<C>();
  test_sequence::<C>();
}
