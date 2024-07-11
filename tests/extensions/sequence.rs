use std::fmt::Debug;
use crate::extensions::util::{assert_equal, from_slice, Equal};
use cantrip::{Iterable, Sequence};

pub(crate) fn test_sequence<'a, C, I>()
where
  I: DoubleEndedIterator<Item = i64>,
  C: Sequence<i64>
    + FromIterator<i64>
    + IntoIterator<Item = i64, IntoIter = I>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + Equal
    + Debug
    + 'a,
{
  // FIXME - implement test for all trait methods
  let repeated = from_slice::<C>(&[1, 2, 2, 3]);
  let empty = from_slice::<C>(&[]);

  // rev
  assert_equal(repeated.clone().rev(), &[3, 2, 2, 1]);
  assert_equal(empty.clone().rev(), &[]);

  // tail
  assert_equal(repeated.clone().tail(), &[2, 2, 3]);
}
