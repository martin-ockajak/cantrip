use std::panic::UnwindSafe;

use cantrip::{Collectible, List, Map, Sequence, Traversable};

use crate::extensions::collectible::test_collectible;
use crate::extensions::list::test_list;
use crate::extensions::map::test_map;
use crate::extensions::ordered::test_ordered;
use crate::extensions::sequence::test_sequence;
use crate::extensions::slice::test_slice;
use crate::extensions::traversable::test_traversable;
use crate::extensions::util::{TestCollectible, TestCollection, TestMap, TestSequence};

pub(crate) fn test_set_traits<'a, C, D>(a: &C, b: &C, d: &D, e: &C)
where
  C: Traversable<i64> + TestCollectible<'a, i64>,
  <C as Collectible<i64>>::This<i64>: TestCollection<i64>,
  D: TestCollectible<'a, Vec<i64>>,
  D::This<i64>: TestCollection<i64>,
{
  test_traversable(false, a, b, e);
  test_collectible(false, a, b, d, e);
}

pub(crate) fn test_slice_traits(a: &[i64], b: &[i64], e: &[i64]) {
  test_traversable(true, a, b, e);
  test_ordered(a, b, e);
  test_slice(a, b, e);
}

pub(crate) fn test_sequence_traits<'a, C, D, G, I>(a: &C, b: &C, c: &C, d: &D, g: &G, e: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: TestSequence<'a, i64, I> + TestCollectible<'a, i64> + UnwindSafe,
  <C as Collectible<i64>>::This<i64>: TestCollection<i64>,
  <C as Sequence<i64>>::This<i64>: TestCollection<i64>,
  <C as Sequence<i64>>::This<(i64, i64)>: TestCollection<(i64, i64)>,
  <C as Sequence<i64>>::This<(usize, i64)>: TestCollection<(usize, i64)>,
  D: TestCollectible<'a, Vec<i64>>,
  D::This<i64>: TestCollection<i64>,
  G: Sequence<(i64, i64)> + TestCollectible<'a, (i64, i64)>,
  <G as Sequence<(i64, i64)>>::This<i64>: TestCollection<i64>,
  for<'c> &'c C: UnwindSafe,
{
  test_traversable(true, a, b, e);
  test_collectible(true, a, b, d, e);
  test_ordered(a, b, e);
  test_sequence(a, b, c, g, e);
}

pub(crate) fn test_list_traits<'a, C, D, G, I>(a: &C, b: &C, c: &C, d: &D, g: &G, e: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: List<i64> + TestSequence<'a, i64, I> + TestCollectible<'a, i64> + UnwindSafe,
  <C as Collectible<i64>>::This<i64>: TestCollection<i64>,
  <C as Sequence<i64>>::This<i64>: TestCollection<i64>,
  <C as Sequence<i64>>::This<(i64, i64)>: TestCollection<(i64, i64)>,
  <C as Sequence<i64>>::This<(usize, i64)>: TestCollection<(usize, i64)>,
  D: TestCollectible<'a, Vec<i64>>,
  D::This<i64>: TestCollection<i64>,
  G: Sequence<(i64, i64)> + TestCollectible<'a, (i64, i64)>,
  <G as Sequence<(i64, i64)>>::This<i64>: TestCollection<i64>,
  for<'c> &'c C: UnwindSafe,
{
  test_traversable(true, a, b, e);
  test_collectible(true, a, b, d, e);
  test_ordered(a, b, e);
  test_sequence(a, b, c, g, e);
  test_list(a, e);
}

pub(crate) fn test_map_traits<'a, C>(a: &C, b: &C, e: &C)
where
  C: TestMap<'a, i64, i64>,
  <C as Map<i64, i64>>::This<i64, i64>: TestCollection<(i64, i64)>,
{
  test_map(a, b, e);
}
