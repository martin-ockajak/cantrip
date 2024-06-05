use std::fmt::Debug;

use cantrip::{Collectible, Iterable};

use crate::extensions::util::{assert_equal, from};

pub fn test_collectible<'a, C>(sequence: bool)
where
  C: Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  let distinct = from::<C>(&[1, 2, 3]);
  let repeated = from::<C>(&[1, 2, 2, 3]);
  let empty = from::<C>(&[]);

  // add
  assert_equal(distinct.clone().add(4), &[1, 2, 3, 4]);
  assert_equal(empty.clone().add(0), &[0]);
  if sequence {
    assert_equal(distinct.clone().add(1), &[1, 2, 3, 1]);
  }

  // delete
  assert_equal(distinct.clone().delete(&2), &[1, 3]);
  assert_equal(distinct.clone().delete(&4), &[1, 2, 3]);
  assert_equal(empty.clone().delete(&0), &[]);
  if sequence {
    assert_equal(repeated.clone().delete(&2), &[1, 2, 3]);
  }

  // delete_all
  assert_equal(distinct.clone().delete_all(&vec![1, 2, 4]), &[3]);
  assert_equal(distinct.clone().delete_all(&vec![]), &[1, 2, 3]);
  assert_equal(empty.clone().delete_all(&vec![0, 1]), &[]);
  if sequence {
    assert_equal(repeated.clone().delete_all(&vec![1, 2, 4]), &[2, 3]);
  }

  // filter
  assert_equal(distinct.clone().filter(|&x| x > 1), &[2, 3]);
  assert_equal(distinct.clone().filter(|&x| x >= 0), &[1, 2, 3]);
  assert_equal(empty.clone().filter(|&x| x == 0), &[]);

  // filter_map
  // assert_eq!(values.clone().filter_map(|x| if x > 1 { Some(x) } else { None }), &[2, 3].into_iter().collect());
  // assert_eq!(values.clone().filter_map(|x| None), &[]);
  // assert_eq!(empty.clone().filter_map(|x| Some(x)), &[]);
}
