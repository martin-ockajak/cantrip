use std::fmt::Debug;

use cantrip::{Iterable, List};

use crate::extensions::util::{assert_equal, from_slice, Equal};

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
  let a = from_slice::<C>(&[1, 2, 3]);
  let e = from_slice::<C>(&[]);

  // first
  assert_eq!(a.clone().first(), Some(&1));
  assert_eq!(e.clone().first(), None);

  // last
  assert_eq!(a.clone().last(), Some(&3));
  assert_eq!(e.clone().last(), None);

  // repeat
  assert_equal(a.clone().repeat(2), &[1, 2, 3, 1, 2, 3]);
  assert_equal(a.clone().repeat(0), &[]);
  assert_equal(e.clone().repeat(2), &[]);
}
