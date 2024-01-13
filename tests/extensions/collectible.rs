use std::fmt::Debug;

use cantrip::{Collectible, Iterable};

use crate::extensions::util::{assert_equal, from};

pub fn test_collectible<'a, C>()
where
  C: Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  let values = from::<C>(&[1, 2, 3]);
  let empty = from::<C>(&[]);

  // add
  assert_equal(values.clone().add(1), &[1, 2, 3, 1]);
  assert_equal(empty.clone().add(1), &[1]);

  // diff
  assert_equal(values.clone().diff(&vec![1, 2, 4]), &[3]);
  assert_equal(values.clone().diff(&vec![]), &[1, 2, 3]);
  assert_equal(empty.clone().diff(&vec![1, 2, 4]), &[]);

  // exclude
  assert_equal(values.clone().exclude(&1), &[2, 3]);
  assert_equal(values.clone().exclude(&4), &[1, 2, 3]);
  assert_equal(empty.clone().exclude(&1), &[]);

  // filter
  assert_equal(values.clone().filter(|&x| x > 1), &[2, 3]);
  assert_equal(values.clone().filter(|&x| x >= 0), &[1, 2, 3]);
  assert_equal(empty.clone().filter(|&x| x == 1), &[]);

  // filter_map
  // assert_eq!(values.clone().filter_map(|x| if x > 1 { Some(x) } else { None }), &[2, 3].into_iter().collect());
  // assert_eq!(values.clone().filter_map(|x| None), &[]);
  // assert_eq!(empty.clone().filter_map(|x| Some(x)), &[]);

}
