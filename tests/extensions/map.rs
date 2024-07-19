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
  let b_source = HashMap::from([(1, 1), (2, 2), (3, 1)]);
  let e_source = C::from_iter(vec![]);

  // add
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.add(4, 4), HashMap::from([(1, 1), (2, 2), (3, 3), (4, 4)]));
  let a = a_source.clone();
  assert_map_equal(a.add(1, 4), HashMap::from([(1, 4), (2, 2), (3, 3)]));
  assert_map_equal(e.add(1, 1), HashMap::from([(1, 1)]));

  // add_multi
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.add_multi(vec![(4, 4), (5, 5)]), HashMap::from([(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]));
  let a = a_source.clone();
  assert_map_equal(a.add_multi(vec![(1, 4), (5, 5)]), HashMap::from([(1, 4), (2, 2), (3, 3), (5, 5)]));
  assert_map_equal(e.add_multi(vec![(1, 1), (2, 2)]), HashMap::from([(1, 1), (2, 2)]));

  // all
  let a = a_source.clone();
  let e = e_source.clone();
  assert!(a.all(|(&k, &v)| k > 0 && v > 0));
  assert!(e.all(|(&k, _)| k > 0));
  assert!(!a.all(|(&k, _)| k > 2));

  // any
  let a = a_source.clone();
  let e = e_source.clone();
  assert!(a.any(|(&k, &v)| k > 0 && v > 1));
  assert!(!a.any(|(&k, _)| k > 5));
  assert!(!e.any(|(&k, _)| k > 0));

  // count_by
  assert_eq!(a.count_by(|(&k, &v)| k == 2 && v == 2), 1);
  assert_eq!(a.count_by(|(&k, _)| k == 5), 0);
  assert_eq!(e.count_by(|(&k, _)| k == 5), 0);

  // count_unique
  let b = b_source.clone();
  assert_eq!(a.count_unique(), 3);
  assert_eq!(b.count_unique(), 2);
  assert_eq!(e.count_unique(), 0);

  // delete
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.delete(&2), HashMap::from([(1, 1), (3, 3)]));
  assert_map_equal(e.delete(&2), HashMap::new());

  // delete_multi
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.delete_multi(&vec![1, 3]), HashMap::from([(2, 2)]));
  assert_map_equal(e.delete_multi(&vec![1]), HashMap::new());

  // disjoint
  let a = a_source.clone();
  let e = e_source.clone();
  assert!(a.disjoint(&vec![4, 5]));
  assert!(a.disjoint(&vec![]));
  assert!(!a.disjoint(&vec![3, 4]));
  assert!(e.disjoint(&vec![1]));

  // fill_with
  assert_map_equal(HashMap::fill_with(|| (1, 1), 1), HashMap::from([(1, 1)]));
  assert_map_equal(HashMap::fill_with(|| (1, 1), 0), HashMap::new());

  // // replace
  // assert_map_equal(distinct.clone().substitute(&0, 0, 1), vec![(0, 1), (1, 1), (2, 2)]);
  // // assert_map_equal(distinct.clone().replace(&0, 1, 2), vec![(1, 2), (2, 2)]);
  // assert_map_equal(empty.clone().substitute(&0, 0, 1), vec![]);
}
