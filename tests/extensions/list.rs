use std::fmt::Debug;

use cantrip::{Iterable, List};

use crate::extensions::util::{assert_equal, Equal, from_slice};

pub(crate) fn test_list<'a, C>()
where
  C: List<i64>
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

  // repeat
  assert_equal(repeated.clone().repeat(2), &[1, 2, 2, 3, 1, 2, 2, 3]);
}
