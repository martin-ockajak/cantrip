use cantrip::{Iterable, List};

use crate::extensions::util::{assert_seq_equal, TestCollection};

pub(crate) fn test_list<'a, C>(a_source: &C, e_source: &C)
where
  C: List<i64> + TestCollection<i64> + IntoIterator<Item = i64> + Iterable<Item<'a> = &'a i64> + 'a,
{
  // first
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.first(), Some(&1));
  assert_eq!(e.first(), None);

  // last
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.last(), Some(&3));
  assert_eq!(e.last(), None);

  // repeat
  assert_seq_equal(a.repeat(2), vec![1, 2, 3, 1, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(a.repeat(0), vec![]);
  assert_seq_equal(e.repeat(2), vec![]);
}
