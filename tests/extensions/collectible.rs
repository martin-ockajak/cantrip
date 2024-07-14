use std::fmt::Debug;

use cantrip::{Collectible, Iterable};

use crate::extensions::util::{assert_equal, assert_set_equal, Equal};

pub(crate) fn test_collectible<'a, C>(sequence: bool)
where
  C: Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let a_source = C::from_iter(vec![1, 2, 3]);
  let b_source = C::from_iter(vec![1, 2, 2, 3]);
  let e_source = C::from_iter(vec![]);
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();

  // add
  if sequence {
    assert_equal(a.add(3), vec![1, 2, 3, 3]);
  } else {
    assert_set_equal(a.add(3), vec![1, 2, 3]);
  }
  assert_equal(e.add(1), vec![1]);

  // add_multi
  let a = a_source.clone();
  if sequence {
    assert_equal(a.add_multi(vec![3, 4]), vec![1, 2, 3, 3, 4]);
  } else {
    assert_set_equal(a.add_multi(vec![3, 4]), vec![1, 2, 3, 4]);
  }
  let e = e_source.clone();
  assert_equal(e.add(1), vec![1]);

  // delete
  if sequence {
    assert_equal(b.delete(&2), vec![1, 2, 3]);
  } else {
    let a = a_source.clone();
    assert_equal(a.delete(&2), vec![1, 3]);
  }
  if sequence {
    let b = b_source.clone();
    assert_equal(b.delete(&4), vec![1, 2, 2, 3]);
  } else {
    let a = a_source.clone();
    assert_set_equal(a.delete(&4), vec![1, 2, 3]);
  }
  let e = e_source.clone();
  assert_equal(e.delete(&2), vec![]);

  // combinations
  // assert_equal(a.combinations(0), vec![vec![]]);
  // assert_equal(a.combinations(1), vec![vec![1], vec![2], vec![3]]);
  // assert_equal(a.combinations(2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
  // assert_equal(a.combinations(3), vec![vec![1, 2, 3]]);
  // assert_equal(a.combinations(4), e);
  // assert_equal(e.combinations(2), e);

  // assert_equal(a.clone().delete_multi(&vec![0, 1]), &[2]);
  // assert_equal(a.clone().delete_multi(&vec![]), &[0, 1, 2]);
  // assert_equal(e.clone().delete_multi(&vec![0, 1]), &[]);
  // if sequence {
  //   assert_equal(b.clone().delete_multi(&vec![0, 1, 3]), &[1, 2]);
  // }
  //
  // // filter
  // assert_equal(a.clone().filter(|&x| x > 0), &[1, 2]);
  // assert_equal(a.clone().filter(|&x| x >= 0), &[0, 1, 2]);
  // assert_equal(e.clone().filter(|&x| x == 0), &[]);
  //
  // // filter_map
  // // assert_eq!(values.clone().filter_map(|x| if x > 1 { Some(x) } else { None }), &[2, 3].into_iter().collect());
  // // assert_eq!(values.clone().filter_map(|x| None), &[]);
  // // assert_eq!(e.clone().filter_map(|x| Some(x)), &[]);
  //
  // // fold
  // // assert_eq!(b.clone().fold(0, |r, x| r + x), 4);
  // assert_eq!(e.clone().fold_to(0, |r, x| r + x), 0);
}
