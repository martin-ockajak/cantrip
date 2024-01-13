use std::fmt::Debug;

use cantrip::{Iterable, Traversable};

use crate::extensions::util::from;

pub fn test_traversable<'a, C>()
where
  C: 'a + Traversable<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug,
{
  let values = from::<C>(&[1, 2, 3]);
  let empty = from::<C>(&[]);

  // all
  assert_eq!(values.all(|&x| x > 0), true);
  assert_eq!(values.all(|&x| x == 1), false);
  assert_eq!(empty.all(|&x| x > 0), true);

  // any
  assert_eq!(values.any(|&x| x == 1), true);
  assert_eq!(values.any(|&x| x == 0), false);
  assert_eq!(empty.any(|&x| x == 1), false);

  // count_by
  assert_eq!(values.count_by(|&x| x > 1), 2);
  assert_eq!(values.count_by(|&x| x == 0), 0);
  assert_eq!(empty.count_by(|&x| x > 1), 0);

  // find
  assert_eq!(values.find(|&x| x == 1), Some(&1));
  assert_eq!(values.find(|&x| x == 0), None);
  assert_eq!(empty.find(|&x| x == 1), None);

  // fold
  assert_eq!(values.fold(0, |r, x| r + x), 6);
  assert_eq!(empty.fold(0, |r, x| r + x), 0);

  // max_by
  assert_eq!(values.max_by(i64::cmp), Some(&3));
  assert_eq!(empty.max_by(i64::cmp), None);

  // max_item
  assert_eq!(values.max_item(), Some(&3));
  assert_eq!(empty.max_item(), None);

  // min_by
  assert_eq!(values.min_by(i64::cmp), Some(&1));
  assert_eq!(empty.min_by(i64::cmp), None);

  // min_item
  assert_eq!(values.min_item(), Some(&1));
  assert_eq!(empty.min_item(), None);
}
