use std::collections::HashMap;
use std::fmt::Debug;

use cantrip::{Iterable, Map};

use crate::extensions::util::{assert_map_equal, Equal};

pub(crate) fn test_map<'a, C>()
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
  let a_source = C::from_iter(vec![(1, 1), (2, 2), (3, 3)]);
  let e_source = C::from_iter(vec![]);

  // add
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.add(4, 4), HashMap::from([
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
  ]));
  let a = a_source.clone();
  assert_map_equal(a.add(1, 4), HashMap::from([
    (1, 4),
    (2, 2),
    (3, 3),
  ]));
  assert_map_equal(e, HashMap::new());

  // // all
  // assert!(distinct.all(|(&k, &v)| k >= 0 && v >= 0));
  // assert!(!distinct.all(|(&k, &v)| k == 1 && v >= 0));
  // assert!(empty.all(|(&k, &v)| k == 0 && v == 0));
  //
  // // replace
  // assert_map_equal(distinct.clone().substitute(&0, 0, 1), vec![(0, 1), (1, 1), (2, 2)]);
  // // assert_map_equal(distinct.clone().replace(&0, 1, 2), vec![(1, 2), (2, 2)]);
  // assert_map_equal(empty.clone().substitute(&0, 0, 1), vec![]);
}
