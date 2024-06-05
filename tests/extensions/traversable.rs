use std::fmt::Debug;

use cantrip::{Iterable, Traversable};

use crate::extensions::util::from_slice;

pub fn test_traversable<'a, C>()
where
  C: Traversable<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Debug + 'a,
{
  let values = from_slice::<C>(&[0, 0, 1, 2]);
  let empty = from_slice::<C>(&[]);

  // all
  assert_eq!(values.all(|&x| x >= 0), true);
  assert_eq!(values.all(|&x| x == 1), false);
  assert_eq!(empty.all(|&x| x == 0), true);

  // any
  assert_eq!(values.any(|&x| x == 1), true);
  assert_eq!(values.any(|&x| x < 0), false);
  assert_eq!(empty.any(|&x| x == 0), false);

  // count_by
  assert_eq!(values.count_by(|&x| x % 2 == 1), 1);
  assert_eq!(values.count_by(|&x| x == 1), 1);
  assert_eq!(empty.count_by(|&x| x == 0), 0);

  // find
  assert_eq!(values.find(|&x| x == 1), Some(&1));
  assert_eq!(values.find(|&x| x < 0), None);
  assert_eq!(empty.find(|&x| x == 0), None);

  // fold
  assert_eq!(values.fold(0, |r, x| r + x), 3);
  assert_eq!(empty.fold(0, |r, x| r + x), 0);

  // max_by
  assert_eq!(values.max_by(i64::cmp), Some(&2));
  assert_eq!(empty.max_by(i64::cmp), None);

  // max_item
  assert_eq!(values.max_item(), Some(&2));
  assert_eq!(empty.max_item(), None);

  // min_by
  assert_eq!(values.min_by(i64::cmp), Some(&0));
  assert_eq!(empty.min_by(i64::cmp), None);

  // min_item
  assert_eq!(values.min_item(), Some(&0));
  assert_eq!(empty.min_item(), None);

  //minmax_item
  assert_eq!(values.minmax_item(), Some((&0, &2)));
  assert_eq!(empty.minmax_item(), None);
}
