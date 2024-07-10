use std::fmt::Debug;

use cantrip::{Collectible, Iterable, Map, Ordered, Reversible, Sequence, Slice, Traversable};

use crate::extensions::collectible::test_collectible;
use crate::extensions::map::test_map;
use crate::extensions::ordered::test_ordered;
use crate::extensions::reversible::test_reversible;
use crate::extensions::sequence::test_sequence;
use crate::extensions::slice::test_slice;
use crate::extensions::traversable::test_traversable;
use crate::extensions::util::Equal;

pub(crate) fn test_collectible_traits<'a, C>()
where
  C:
    Traversable<i64> + Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  test_traversable::<C>();
  test_collectible::<C>(false);
}

#[allow(dead_code)]
pub(crate) fn test_slice_traits<'a, C>()
where
  C: Traversable<i64>
    + Collectible<i64>
    + Ordered<i64>
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
  test_ordered::<C>();
  test_reversible::<C>();
  test_slice::<C>();
}

pub(crate) fn test_sequence_traits<'a, C>()
where
  C: Traversable<i64>
    + Collectible<i64>
    + Ordered<i64>
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
  test_ordered::<C>();
  test_reversible::<C>();
  test_sequence::<C>();
}

pub(crate) fn test_map_traits<'a, C>()
where
  C: Map<i64, i64>
    + FromIterator<(i64, i64)>
    + IntoIterator<Item = (i64, i64)>
    + Iterable<Item<'a> = (&'a i64, &'a i64)>
    + Clone
    + Equal
    + Debug
    + 'a,
{
  test_map::<C>();
}
