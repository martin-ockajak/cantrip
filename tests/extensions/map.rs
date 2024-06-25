use std::fmt::Debug;

use cantrip::{Iterable, Map};

use crate::extensions::util::{assert_map_equal, from_map_slice, Equal};

pub fn test_map<'a, C>()
where
  C: Map<i64, i64>
    + FromIterator<(i64, i64)>
    + IntoIterator<Item = (i64, i64)>
    + Iterable<Item<'a> = (&'a i64, &'a i64)>
    + Clone
    + Equal
    + Debug
    + 'a,
{
  // FIXME - implement test for all trait methods
  let distinct = from_map_slice::<C>(&[(0, 0), (1, 1), (2, 2)]);
  let empty = from_map_slice::<C>(&[]);

  // add
  assert_map_equal(distinct.clone().add(3, 3), &[(0, 0), (1, 1), (2, 2), (3, 3)]);
  assert_map_equal(empty.clone().add(0, 0), &[(0, 0)]);
  assert_map_equal(empty.clone(), &[]);

  // all
  assert!(distinct.all(|(&k, &v)| k >= 0 && v >= 0));
  assert!(!distinct.all(|(&k, &v)| k == 1 && v >= 0));
  assert!(empty.all(|(&k, &v)| k == 0 && v == 0));
  
  // replace
  assert_map_equal(distinct.clone().replace(&0, 0, 1), &[(0, 1), (1, 1), (2, 2)]);
  // assert_map_equal(distinct.clone().replace(&0, 1, 2), &[(1, 2), (2, 2)]);
  assert_map_equal(empty.clone().replace(&0, 0, 1), &[]);
}
