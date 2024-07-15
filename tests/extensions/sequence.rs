use std::fmt::Debug;

use cantrip::{Iterable, Sequence};

use crate::extensions::util::Equal;

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
  // let a_source = C::from_iter(vec![1, 2, 3]);
  // let b_source = C::from_iter(vec![1, 2, 2, 3]);
  // let e_source = C::from_iter(vec![]);
  // let a = a_source.clone();
  // let b = b_source.clone();
  // let e = e_source.clone();

  // // rev
  // assert_equal(repeated.clone().rev(), vec![3, 2, 2, 1]);
  // assert_equal(empty.clone().rev(), vec![]);
  //
  // // tail
  // assert_equal(repeated.clone().tail(), vec![2, 2, 3]);
}
