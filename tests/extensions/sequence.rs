use std::fmt::Debug;

use crate::extensions::util::{assert_equal, from_slice, Equal};
use cantrip::{Iterable, Sequence};

pub(crate) fn test_sequence<'a, C>()
where
  C: Sequence<i64>
    + FromIterator<i64>
    + IntoIterator<Item = i64>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + Equal
    + Debug
    + 'a,
{
  // FIXME - implement test for all trait methods
  let repeated = from_slice::<C>(&[1, 2, 2, 3]);
  // let empty = from::<C>(&[]);

  // tail
  assert_equal(repeated.tail(), &[2, 2, 3]);
}
