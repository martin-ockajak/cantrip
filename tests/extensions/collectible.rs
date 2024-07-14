use std::fmt::Debug;

use cantrip::{Collectible, Iterable};

use crate::extensions::util::{assert_equal, Equal, from_slice};

pub(crate) fn test_collectible<'a, C>(sequence: bool)
where
  C: Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let a_source = from_slice::<C>(&[1, 2, 3]);
  // let b_source = from_slice::<C>(&[1, 2, 2, 3]);
  let e_source = from_slice::<C>(&[]);
  let a = a_source.clone();
  // let b = b_source.clone();
  let e = e_source.clone();

  // add
  if sequence {
    assert_equal(a.add(3), &[1, 2, 3, 3]);
  } else {
    // FIXME - handle sets
    // assert_equal(a.add(3), &[1, 2, 3]);
  }
  assert_equal(e.clone().add(1), &[1]);

  // // delete
  // assert_equal(a.clone().delete(&1), &[0, 2]);
  // assert_equal(a.clone().delete(&3), &[0, 1, 2]);
  // assert_equal(e.clone().delete(&0), &[]);
  // if sequence {
  //   assert_equal(b.clone().delete(&1), &[0, 1, 2]);
  // }
  //
  // // delete_all
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
