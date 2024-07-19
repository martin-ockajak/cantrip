use std::fmt::Debug;
use std::panic::UnwindSafe;

use cantrip::{Collectible, Iterable, List, Map, Ordered, Sequence, Traversable};

use crate::extensions::collectible::test_collectible;
use crate::extensions::list::test_list;
use crate::extensions::map::test_map;
use crate::extensions::ordered::test_ordered;
use crate::extensions::sequence::test_sequence;
use crate::extensions::slice::test_slice;
use crate::extensions::traversable::test_traversable;
use crate::extensions::util::Equal;

pub(crate) fn test_set_traits<'a, C>(a: &C, b: &C, e: &C)
where
  C: Traversable<i64>
    + Collectible<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Default
    + Extend<i64>
    + Clone
    + Equal
    + Debug
    + 'a,
  <C as Collectible<i64>>::This<i64>: FromIterator<i64> + Default + Extend<i64> + Equal + Debug,
{
  test_traversable(false, a, b, e);
  test_collectible(false, a, b, e);
}

#[allow(dead_code)]
pub(crate) fn test_slice_traits(a: &[i64], b: &[i64], e: &[i64])
{
  test_traversable(true, a, b, e);
  test_ordered(a, b, e);
  test_slice(a, b, e);
}

pub(crate) fn test_sequence_traits<'a, C, I>(a: &C, b: &C, c: &C, e: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: Traversable<i64>
    + Collectible<i64>
    + Ordered<i64>
    + Sequence<i64>
    + FromIterator<i64>
    + IntoIterator<Item = i64, IntoIter = I>
    + Iterable<Item<'a> = &'a i64>
    + Default
    + Extend<i64>
    + Clone
    + Equal
    + UnwindSafe
    + Debug
    + 'a,
  <C as Collectible<i64>>::This<i64>: FromIterator<i64> + Default + Extend<i64> + Equal + Debug,
  <C as Sequence<i64>>::This<i64>: FromIterator<i64> + Equal + Debug,
  <C as Sequence<i64>>::This<(i64, i64)>: FromIterator<(i64, i64)> + Equal + Debug,
  <C as Sequence<i64>>::This<(usize, i64)>: FromIterator<(usize, i64)> + Equal + Debug,
  for<'c> &'c C: UnwindSafe,
{
  test_traversable(true, a, b, e);
  test_collectible(true, a, b, e);
  test_ordered(a, b, e);
  test_sequence(a, b, c, e);
}

pub(crate) fn test_list_traits<'a, C, I>(a: &C, b: &C, c: &C, e: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: Traversable<i64>
    + Collectible<i64>
    + Ordered<i64>
    + Sequence<i64>
    + List<i64>
    + FromIterator<i64>
    + IntoIterator<Item = i64, IntoIter = I>
    + Iterable<Item<'a> = &'a i64>
    + Default
    + Extend<i64>
    + Clone
    + Equal
    + UnwindSafe
    + Debug
    + 'a,
  <C as Collectible<i64>>::This<i64>: FromIterator<i64> + Default + Extend<i64> + Equal + Debug,
  <C as Sequence<i64>>::This<i64>: FromIterator<i64> + Equal + Debug,
  <C as Sequence<i64>>::This<(i64, i64)>: FromIterator<(i64, i64)> + Equal + Debug,
  <C as Sequence<i64>>::This<(usize, i64)>: FromIterator<(usize, i64)> + Equal + Debug,
  for<'c> &'c C: UnwindSafe,
{
  test_traversable(true, a, b, e);
  test_collectible(true, a, b, e);
  test_ordered(a, b, e);
  test_sequence(a, b, c, e);
  test_list(a, e);
}

pub(crate) fn test_map_traits<'a, C>(a: &C, b: &C, e: &C)
where
  C: Map<i64, i64>
    + FromIterator<(i64, i64)>
    + IntoIterator<Item = (i64, i64)>
    + Iterable<Item<'a> = (&'a i64, &'a i64)>
    + Default
    + Extend<(i64, i64)>
    + Clone
    + Equal
    + Debug
    + 'a,
  <C as Map<i64, i64>>::This<i64, i64>: FromIterator<(i64, i64)> + Default + Extend<(i64, i64)> + Equal + Debug,
{
  test_map(a, b, e);
}
