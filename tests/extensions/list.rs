use std::fmt::Debug;

use cantrip::{Iterable, List};

use crate::extensions::util::{assert_seq_equal, Equal};

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
  let a_source = C::from_iter(vec![1, 2, 3]);
  let e_source = C::from_iter(vec![]);
  let a = a_source.clone();
  let e = e_source.clone();

  // first
  assert_eq!(a.first(), Some(&1));
  assert_eq!(e.first(), None);

  // last
  assert_eq!(a.last(), Some(&3));
  assert_eq!(e.last(), None);

  // repeat
  assert_seq_equal(a.repeat(2), vec![1, 2, 3, 1, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(a.repeat(0), vec![]);
  assert_seq_equal(e.repeat(2), vec![]);
}
