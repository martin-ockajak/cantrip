use std::panic::UnwindSafe;

use cantrip::{Collection, CollectionTo, Convert, List, Map, SequenceTo};

use crate::extensions::collection::test_collection;
use crate::extensions::collection_to::test_collection_to;
use crate::extensions::convert::test_convert;
use crate::extensions::list::test_list;
use crate::extensions::map::test_map;
use crate::extensions::sequence::test_sequence;
use crate::extensions::sequence_to::test_sequence_to;
use crate::extensions::slice::test_slice;
use crate::extensions::util::{TestCollectible, TestCollection, TestMap, TestSequence};

pub(crate) fn test_list_traits<'a, C, D, G, I>(a: &C, b: &C, c: &C, d: &D, g: &G, e: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: List<i64> + TestSequence<'a, i64, I> + TestCollectible<'a, i64> + UnwindSafe,
  <C as CollectionTo<i64>>::This<i64>: TestCollection<i64>,
  <C as SequenceTo<i64>>::This<i64>: TestCollection<i64>,
  <C as SequenceTo<i64>>::This<(i64, i64)>: TestCollection<(i64, i64)>,
  <C as SequenceTo<i64>>::This<(usize, i64)>: TestCollection<(usize, i64)>,
  D: TestCollectible<'a, Vec<i64>>,
  D::This<i64>: TestCollection<i64>,
  G: SequenceTo<(i64, i64)> + CollectionTo<(i64, i64)> + TestCollectible<'a, (i64, i64)>,
  <G as SequenceTo<(i64, i64)>>::This<i64>: TestCollection<i64>,
  for<'c> &'c C: UnwindSafe,
{
  test_collection(true, a, b, e);
  test_collection_to(true, a, b, d, e);
  test_sequence(a, b, e);
  test_sequence_to(a, b, c, g, e);
  test_list(a, e);
  test_convert(true, a, g, e);
}

pub(crate) fn test_map_traits<'a, C>(a: &C, b: &C, e: &C)
where
  C: TestMap<'a, i64, i64> + Convert<(i64, i64)>,
  <C as Map<i64, i64>>::This<i64, i64>: TestCollection<(i64, i64)>,
{
  test_map(a, b, e);
}

pub(crate) fn test_set_traits<'a, C, D, G>(a: &C, b: &C, d: &D, g: &G, e: &C)
where
  C: Collection<i64> + TestCollectible<'a, i64>,
  <C as CollectionTo<i64>>::This<i64>: TestCollection<i64>,
  D: TestCollectible<'a, Vec<i64>>,
  D::This<i64>: TestCollection<i64>,
  G: TestCollectible<'a, (i64, i64)>,
{
  test_collection(false, a, b, e);
  test_collection_to(false, a, b, d, e);
  test_convert(false, a, g, e);
}

pub(crate) fn test_sequence_traits<'a, C, D, G, I>(a: &C, b: &C, c: &C, d: &D, g: &G, e: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: TestSequence<'a, i64, I> + TestCollectible<'a, i64> + UnwindSafe,
  <C as CollectionTo<i64>>::This<i64>: TestCollection<i64>,
  <C as SequenceTo<i64>>::This<i64>: TestCollection<i64>,
  <C as SequenceTo<i64>>::This<(i64, i64)>: TestCollection<(i64, i64)>,
  <C as SequenceTo<i64>>::This<(usize, i64)>: TestCollection<(usize, i64)>,
  D: TestCollectible<'a, Vec<i64>>,
  D::This<i64>: TestCollection<i64>,
  G: SequenceTo<(i64, i64)> + CollectionTo<(i64, i64)> + TestCollectible<'a, (i64, i64)>,
  <G as SequenceTo<(i64, i64)>>::This<i64>: TestCollection<i64>,
  for<'c> &'c C: UnwindSafe,
{
  test_collection(true, a, b, e);
  test_collection_to(true, a, b, d, e);
  test_sequence(a, b, e);
  test_sequence_to(a, b, c, g, e);
  test_convert(true, a, g, e);
}

pub(crate) fn test_slice_traits(a: &[i64], b: &[i64], e: &[i64]) {
  test_collection(true, a, b, e);
  test_sequence(a, b, e);
  test_slice(a, b, e);
}
