use std::fmt::Debug;

use cantrip::{Iterable, Traversable};

use crate::extensions::util::from_slice;

pub(crate) fn test_traversable<'a, C>()
where
  C: Traversable<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let repeated = from_slice::<C>(&[0, 0, 1, 2]);
  let empty = from_slice::<C>(&[]);

  // all
  assert!(repeated.all(|&x| x >= 0));
  assert!(!repeated.all(|&x| x == 1));
  assert!(empty.all(|&x| x == 0));

  // any
  assert!(repeated.any(|&x| x == 1));
  assert!(!repeated.any(|&x| x < 0));
  assert!(!empty.any(|&x| x == 0));

  // count_by
  assert_eq!(repeated.count_by(|&x| x % 2 == 1), 1);
  assert_eq!(repeated.count_by(|&x| x == 1), 1);
  assert_eq!(empty.count_by(|&x| x == 0), 0);

  // find
  assert_eq!(repeated.find(|&x| x == 1), Some(&1));
  assert_eq!(repeated.find(|&x| x < 0), None);
  assert_eq!(empty.find(|&x| x == 0), None);

  // max_by
  assert_eq!(repeated.max_by(i64::cmp), Some(&2));
  assert_eq!(empty.max_by(i64::cmp), None);

  // max_of
  assert_eq!(repeated.max_of(), Some(&2));
  assert_eq!(empty.max_of(), None);

  // min_by
  assert_eq!(repeated.min_by(i64::cmp), Some(&0));
  assert_eq!(empty.min_by(i64::cmp), None);

  // min_of
  assert_eq!(repeated.min_of(), Some(&0));
  assert_eq!(empty.min_of(), None);

  //minmax_of
  assert_eq!(repeated.minmax_of(), Some((&0, &2)));
  assert_eq!(empty.minmax_of(), None);
}
