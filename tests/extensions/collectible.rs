use std::fmt::Debug;

use cantrip::{Collectible, Iterable};

use crate::extensions::util::{assert_equal, Equal, from_slice};

pub(crate) fn test_collectible<'a, C>(sequence: bool)
where
  C: Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let distinct = from_slice::<C>(&[0, 1, 2]);
  let repeated = from_slice::<C>(&[0, 1, 1, 2]);
  let empty = from_slice::<C>(&[]);

  // add
  assert_equal(distinct.clone().add(3), &[0, 1, 2, 3]);
  assert_equal(empty.clone().add(0), &[0]);
  if sequence {
    assert_equal(distinct.clone().add(0), &[0, 1, 2, 0]);
  }

  // delete
  assert_equal(distinct.clone().delete(&1), &[0, 2]);
  assert_equal(distinct.clone().delete(&3), &[0, 1, 2]);
  assert_equal(empty.clone().delete(&0), &[]);
  if sequence {
    assert_equal(repeated.clone().delete(&1), &[0, 1, 2]);
  }

  // delete_all
  assert_equal(distinct.clone().delete_all(&vec![0, 1]), &[2]);
  assert_equal(distinct.clone().delete_all(&vec![]), &[0, 1, 2]);
  assert_equal(empty.clone().delete_all(&vec![0, 1]), &[]);
  if sequence {
    assert_equal(repeated.clone().delete_all(&vec![0, 1, 3]), &[1, 2]);
  }

  // filter
  assert_equal(distinct.clone().filter(|&x| x > 0), &[1, 2]);
  assert_equal(distinct.clone().filter(|&x| x >= 0), &[0, 1, 2]);
  assert_equal(empty.clone().filter(|&x| x == 0), &[]);

  // filter_map
  // assert_eq!(values.clone().filter_map(|x| if x > 1 { Some(x) } else { None }), &[2, 3].into_iter().collect());
  // assert_eq!(values.clone().filter_map(|x| None), &[]);
  // assert_eq!(empty.clone().filter_map(|x| Some(x)), &[]);

  // fold
  // assert_eq!(repeated.clone().fold(0, |r, x| r + x), 4);
  assert_eq!(empty.clone().fold_to(0, |r, x| r + x), 0);
}
