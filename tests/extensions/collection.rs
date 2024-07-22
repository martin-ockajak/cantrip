use std::collections::HashMap;
use std::fmt::Debug;

use cantrip::{Iterable, Collection};

pub(crate) fn test_collection<'a, C>(sequence: bool, a: &C, b: &C, e: &C)
where
  C: Collection<i64> + Iterable<Item<'a> = &'a i64> + Debug + ?Sized + 'a,
{
  // all
  assert!(a.all(|&x| x > 0));
  assert!(e.all(|&x| x > 0));
  assert!(!a.all(|&x| x > 2));

  // any
  assert!(a.any(|&x| x > 0));
  assert!(!a.any(|&x| x > 5));
  assert!(!e.any(|&x| x > 0));

  // count_by
  assert_eq!(a.count_by(|&x| x == 2), 1);
  assert_eq!(a.count_by(|&x| x == 5), 0);
  assert_eq!(e.count_by(|&x| x == 5), 0);

  // disjoint
  assert!(a.disjoint(&vec![4, 5]));
  assert!(a.disjoint(&vec![]));
  assert!(e.disjoint(&vec![]));
  assert!(!a.disjoint(&vec![3, 4]));
  assert!(e.disjoint(&vec![3, 4]));

  // find
  assert_eq!(a.find(|&x| x == 2), Some(&2));
  assert_eq!(a.find(|&x| x == 5), None);
  assert_eq!(e.find(|&x| x == 5), None);

  // find_map_ref
  assert_eq!(a.find_map_ref(|&x| if x % 2 == 0 { Some(x) } else { None }), Some(2));
  assert_eq!(e.find_map_ref(|&x| if x % 2 == 0 { Some(x) } else { None }), None);

  // fold_ref
  assert_eq!(a.fold_ref(0, |acc, x| acc + x), 6);
  assert_eq!(e.fold_ref(0, |acc, x| acc + x), 0);

  // for_each
  let mut acc = 0;
  a.for_each(|&x| acc += x);
  assert_eq!(acc, 6);
  e.for_each(|&x| acc += x);
  assert_eq!(acc, 6);

  // group_fold_ref
  assert_eq!(a.group_fold_ref(|x| x % 2, 0, |acc, &x| acc + x), HashMap::from([(0, 2), (1, 4),]));
  assert_eq!(e.group_fold_ref(|x| x % 2, 0, |acc, &x| acc + x), HashMap::new());

  // group_reduce_ref
  assert_eq!(a.group_reduce_ref(|x| x % 2, |acc, x| acc + x), HashMap::from([(0, 2), (1, 4),]));
  assert_eq!(e.group_reduce_ref(|x| x % 2, |acc, x| acc + x), HashMap::new());

  // max_by
  assert_eq!(b.max_by(i64::cmp), Some(&3));
  assert_eq!(e.max_by(i64::cmp), None);

  // max_by_key
  assert_eq!(a.max_by_key(|x| -x), Some(&1));
  assert_eq!(e.max_by_key(|x| -x), None);

  // max_of
  assert_eq!(b.max_of(), Some(&3));
  assert_eq!(e.max_of(), None);

  // min_by
  assert_eq!(b.min_by(i64::cmp), Some(&1));
  assert_eq!(e.min_by(i64::cmp), None);

  // min_by_key
  assert_eq!(a.min_by_key(|x| -x), Some(&3));
  assert_eq!(e.min_by_key(|x| -x), None);

  // min_of
  assert_eq!(b.min_of(), Some(&1));
  assert_eq!(e.min_of(), None);

  // minmax_by
  assert_eq!(a.minmax_by(|x, y| x.cmp(y)), Some((&1, &3)));
  assert_eq!(e.minmax_by(|x, y| x.cmp(y)), None);

  // minmax_by_key
  assert_eq!(a.minmax_by_key(|x| -x), Some((&3, &1)));
  assert_eq!(e.minmax_by_key(|x| -x), None);

  // minmax_of
  assert_eq!(b.minmax_of(), Some((&1, &3)));
  assert_eq!(e.minmax_of(), None);

  // reduce_ref
  assert_eq!(a.reduce_ref(|&acc, &e| acc + e), Some(6));
  assert_eq!(e.reduce_ref(|&acc, &e| acc + e), None);

  // subset
  assert!(a.subset(&vec![4, 3, 2, 2, 1]));
  assert!(e.subset(&vec![1]));
  assert!(e.subset(&vec![]));
  if sequence {
    assert!(!b.subset(&vec![1, 2, 3]));
  }
  assert!(!a.subset(&vec![3, 4]));

  // superset
  assert!(a.superset(&vec![2, 1]));
  assert!(e.superset(&vec![]));
  if sequence {
    assert!(!b.superset(&vec![1, 1, 2]));
  }
  assert!(!a.superset(&vec![3, 4]));
  assert!(!e.superset(&vec![1]));
}
