use std::collections::HashMap;
use std::fmt::Debug;

use cantrip::{Iterable, Map};

use crate::extensions::util::{assert_map_equal, Equal};

pub(crate) fn test_map<'a, C>(a_source: &C, b_source: &C, e_source: &C)
where
  C: Map<i64, i64>
    + FromIterator<(i64, i64)>
    + IntoIterator<Item = (i64, i64)>
    + Iterable<Item<'a> = (&'a i64, &'a i64)>
    + Default
    + Extend<(i64, i64)>
    + Clone
    + Equal
    + Debug
    + 'a,
{
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

  // filter
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.filter(|(&k, &v)| k != 2 && v != 2), HashMap::from([(1, 1), (3, 3)]));
  assert_map_equal(e.filter(|(&k, &v)| k != 2 && v != 2), HashMap::new());

  // filter_keys
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.filter_keys(|&k| k != 2), HashMap::from([(1, 1), (3, 3)]));
  assert_map_equal(e.filter_keys(|&k| k != 2), HashMap::new());

  // filter_Values
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.filter_values(|&v| v != 2), HashMap::from([(1, 1), (3, 3)]));
  assert_map_equal(e.filter_values(|&v| v != 2), HashMap::new());

  // filter_map - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(a.filter_map(|(&k, &v)| if k < 2 { Some((k, v + 1)) } else { None }), HashMap::from([(1, 2)]));
  // assert_map_equal(e.filter_map(|(&k, &v)| if k < 2 { Some((k, v + 1)) } else { None }), HashMap::new());

  // filter_map_to - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_to_equal(a.filter_map_to(|(k, v)| if k < 2 { Some((k, v + 1)) } else { None }), HashMap::from([(1, 2)]));
  // assert_map_to_equal(e.filter_map_to(|(k, v)| if k < 2 { Some((k, v + 1)) } else { None }), HashMap::new());

  // find
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.find(|(&k, &v)| k == 2 && v == 2), Some((&2, &2)));
  assert_eq!(a.find(|(&k, _)| k == 5), None);
  assert_eq!(e.find(|(&k, _)| k == 5), None);

  // find_map
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.find_map(|(&k, &v)| if k == 2 { Some(v) } else { None }), Some(2));
  assert_eq!(e.find_map(|(&k, &v)| if k == 2 { Some(v) } else { None }), None);

  // find_map_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.find_map_to(|(k, v)| if k == 2 { Some(v) } else { None }), Some(2));
  assert_eq!(e.find_map_to(|(k, v)| if k == 2 { Some(v) } else { None }), None);

  // flat_map - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(
  //   a.flat_map(|(&k, &v)| vec![(-k, v), (k, v)]),
  //   HashMap::from([(-1, 1), (-2, 2), (-3, 3), (1, 1), (2, 2), (3, 3)]),
  // );
  // assert_map_equal(e.flat_map(|(&k, &v)| vec![(-k, v), (k, v)]), HashMap::new());

  // flat_map_to - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(
  //   a.flat_map(|(k, v)| vec![(-k, v), (k, v)]),
  //   HashMap::from([(-1, 1), (-2, 2), (-3, 3), (1, 1), (2, 2), (3, 3)]),
  // );
  // assert_map_equal(e.flat_map(|(k, v)| vec![(-k, v), (k, v)]), HashMap::new());

  // fold
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.fold(0, |acc, (&k, &v)| acc + k + v), 12);
  assert_eq!(e.fold(0, |acc, (&k, &v)| acc + k + v), 0);

  // fold_to
  assert_eq!(a.fold_to(0, |acc, (k, v)| acc + k + v), 12);
  assert_eq!(e.fold_to(0, |acc, (k, v)| acc + k + v), 0);

  // for_each
  let a = a_source.clone();
  let e = e_source.clone();
  let mut acc = 0;
  a.for_each(|(&k, &v)| acc += k + v);
  assert_eq!(acc, 12);
  e.for_each(|(&k, &v)| acc += k + v);
  assert_eq!(acc, 12);

  // intersect
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.intersect(&vec![(4, 4), (2, 2), (3, 4), (4, 5)]), HashMap::from([(2, 2)]));
  assert_map_equal(e.intersect(&vec![(1, 1)]), HashMap::new());

  // map - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(a.map(|(&k, &v)| (k, k + v)), HashMap::from([(1, 2), (2, 4), (3, 6)]));
  // assert_map_equal(e.map(|(&k, &v)| (k, k + v)), HashMap::new());

  // map_to - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(a.map_to(|(k, v)| (k, k + v)), HashMap::from([(1, 2), (2, 4), (3, 6)]));
  // assert_map_equal(e.map_to(|(k, v)| (k, k + v)), HashMap::new());

  // map_keys - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(a.map_keys(|&k| k + 1), HashMap::from([(2, 1), (3, 2), (4, 3),]));
  // assert_map_equal(e.map_keys(|&k| k + 1), HashMap::new());

  // map_values - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_map_equal(a.map_values(|&v| v + 1), HashMap::from([(1, 2), (2, 3), (3, 4),]));
  // assert_map_equal(e.map_values(|&v| v + 1), HashMap::new());

  // max_by
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.max_by(|x, y| x.0.cmp(y.0)), Some((&3, &3)));
  assert_eq!(e.max_by(|x, y| x.0.cmp(y.0)), None);

  // max_by_key
  assert_eq!(a.max_by_key(|(k, _)| -k), Some((&1, &1)));
  assert_eq!(e.max_by_key(|(k, _)| -k), None);

  // max_of
  assert_eq!(a.max_of(), Some((&3, &3)));
  assert_eq!(e.max_of(), None);

  // min_by
  assert_eq!(a.min_by(|x, y| x.0.cmp(y.0)), Some((&1, &1)));
  assert_eq!(e.min_by(|x, y| x.0.cmp(y.0)), None);

  // min_by_key
  assert_eq!(a.min_by_key(|(k, _)| -k), Some((&3, &3)));
  assert_eq!(e.min_by_key(|(k, _)| -k), None);

  // min_of
  assert_eq!(a.min_of(), Some((&1, &1)));
  assert_eq!(e.min_of(), None);

  // minmax_by
  assert_eq!(a.minmax_by(|x, y| x.0.cmp(y.0)), Some(((&1, &1), (&3, &3))));
  assert_eq!(e.minmax_by(|x, y| x.0.cmp(y.0)), None);

  // minmax_by_key
  assert_eq!(a.minmax_by_key(|(k, _)| -k), Some(((&3, &3), (&1, &1))));
  assert_eq!(e.minmax_by_key(|(k, _)| -k), None);

  // minmax_of
  assert_eq!(a.minmax_of(), Some(((&1, &1), (&3, &3))));
  assert_eq!(e.minmax_of(), None);

  // partition
  let a = a_source.clone();
  let e = e_source.clone();
  let (a_even, a_odd) = a.partition(|(&k, _)| k % 2 == 0);
  assert_map_equal(a_even, HashMap::from([(2, 2)]));
  assert_map_equal(a_odd, HashMap::from([(1, 1), (3, 3)]));
  let (e_even, e_odd) = e.partition(|(&k, _)| k % 2 == 0);
  assert_map_equal(e_even, HashMap::new());
  assert_map_equal(e_odd, HashMap::new());

  // partition_map - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // let (a_even, a_odd) = a.partition_map(|(&k, &v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  // assert_map_equal(a_even, HashMap::from([(5, 2),]));
  // assert_map_equal(a_odd, HashMap::from([(1, 1), (3, 3),]));
  // let (e_even, e_odd) = a.partition_map(|(&k, &v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  // assert_map_equal(e_even, HashMap::new());
  // assert_map_equal(e_odd, HashMap::new());

  // partition_map_to - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // let (a_even, a_odd) = a.partition_map_to(|(k, v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  // assert_map_equal(a_even, HashMap::from([(5, 2),]));
  // assert_map_equal(a_odd, HashMap::from([(1, 1), (3, 3),]));
  // let (e_even, e_odd) = a.partition_map_to(|(k, v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  // assert_map_equal(e_even, HashMap::new());
  // assert_map_equal(e_odd, HashMap::new());

  // product_keys
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.product_keys(), 6);
  assert_eq!(e.product_keys(), 1);

  // product_values
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.product_values(), 6);
  assert_eq!(e.product_values(), 1);

  // reduce
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.reduce(|(&a, &b), (&k, &v)| (a + k, b + v)), Some((6, 6)));
  assert_eq!(e.reduce(|(&a, &b), (&k, &v)| (a + k, b + v)), None);

  // reduce_to
  assert_eq!(a.reduce_to(|(a, b), (k, v)| (a + k, b + v)), Some((6, 6)));
  assert_eq!(e.reduce_to(|(a, b), (k, v)| (a + k, b + v)), None);

  // subset
  let a = a_source.clone();
  let e = e_source.clone();
  assert!(a.subset(&vec![4, 3, 2, 2, 1]));
  assert!(e.subset(&vec![1]));
  assert!(!a.subset(&vec![1, 2]));
  assert!(!a.subset(&vec![]));

  // substitute
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.substitute(&3, 4, 4), HashMap::from([(1, 1), (2, 2), (4, 4)]));
  assert_map_equal(e.substitute(&3, 4, 4), HashMap::new());

  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.substitute_multi(&vec![2, 3], vec![(4, 4), (5, 5)]), HashMap::from([(1, 1), (4, 4), (5, 5)]));
  assert_map_equal(e.substitute_multi(&vec![2, 3], vec![(4, 4), (5, 5)]), HashMap::new());

  // superset
  let a = a_source.clone();
  let e = e_source.clone();
  assert!(a.superset(&vec![3, 1]));
  assert!(a.superset(&vec![]));
  assert!(!a.superset(&vec![1, 2, 3, 4]));
  assert!(!a.superset(&vec![1, 2, 2]));
  assert!(!a.superset(&vec![3, 4]));
  assert!(!e.superset(&vec![1]));

  // sum_keys
  assert_eq!(a.sum_keys(), 6);
  assert_eq!(e.sum_keys(), 0);

  // sum_values
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.sum_values(), 6);
  assert_eq!(e.sum_values(), 0);

  // unit
  assert_map_equal(HashMap::unit(1, 1), HashMap::from([(1, 1),]));
}
