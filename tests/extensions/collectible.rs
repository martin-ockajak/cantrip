use std::fmt::Debug;
use cantrip::{Collectible, Iterable};

use crate::extensions::util::{assert_equal, assert_vec_set_equal, assert_set_equal, Equal};

pub(crate) fn test_collectible<'a, C>(sequence: bool)
where
  C: Collectible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
  C::This<i64>: FromIterator<i64> + Equal + Debug,
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

  // combinations
  let a = a_source.clone();
  assert_vec_set_equal(a.combinations(0), vec![vec![]]);
  assert_vec_set_equal(a.combinations(1), vec![vec![1], vec![2], vec![3]]);
  assert_vec_set_equal(a.combinations(2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
  assert_vec_set_equal(a.combinations(3), vec![vec![1, 2, 3]]);
  assert_vec_set_equal(a.combinations(4), vec![]);
  let e = e_source.clone();
  assert_vec_set_equal(e.combinations(2), vec![]);

  // delete
  if sequence {
    assert_equal(b.delete(&2), vec![1, 2, 3]);
    let b = b_source.clone();
    assert_equal(b.delete(&4), vec![1, 2, 2, 3]);
  } else {
    let a = a_source.clone();
    assert_equal(a.delete(&2), vec![1, 3]);
    let a = a_source.clone();
    assert_set_equal(a.delete(&4), vec![1, 2, 3]);
  }
  let e = e_source.clone();
  assert_equal(e.delete(&2), vec![]);

  // delete_multi
  if sequence {
    let b = b_source.clone();
    assert_equal(b.delete_multi(&vec![1, 2]), vec![2, 3]);
    let b = b_source.clone();
    assert_equal(b.delete_multi(&vec![4]), vec![1, 2, 2, 3]);
  } else {
    let a = a_source.clone();
    assert_equal(a.delete_multi(&vec![1, 2]), vec![3]);
    let a = a_source.clone();
    assert_equal(a.delete_multi(&vec![4]), vec![1, 2, 3]);
  }
  let e = e_source.clone();
  assert_equal(e.delete_multi(&vec![1]), vec![]);

  // fill_with
  if sequence {
    assert_equal(C::fill_with(|| 1, 2), vec![1, 1]);
  } else {
    assert_equal(C::fill_with(|| 1, 2), vec![1]);
  }
  assert_equal(C::fill_with(|| 1, 0), vec![]);

  // filter
  let a = a_source.clone();
  assert_equal(a.filter(|&x| x > 1), vec![2, 3]);
  let e = e_source.clone();
  assert_equal(e.filter(|&x| x > 1), vec![]);

  // filter_map
  let a = a_source.clone();
  assert_equal(a.filter_map(|&x| if x % 2 == 0 { Some(x + 1) } else { None } ), vec![3]);
  let e = e_source.clone();
  assert_equal(e.filter_map(|&x| if x % 2 == 0 { Some(x + 1) } else { None } ), vec![]);

  // filter_map_to
  let a = a_source.clone();
  assert_equal(a.filter_map_to(|x| if x % 2 == 0 { Some(x + 1) } else { None } ), vec![3]);
  let e = e_source.clone();
  assert_equal(e.filter_map_to(|x| if x % 2 == 0 { Some(x + 1) } else { None } ), vec![]);

  // find_map_to
  let a = a_source.clone();
  assert_eq!(a.find_map_to(|x| if x % 2 == 0 { Some(x) } else { None } ), Some(2));
  let e = e_source.clone();
  assert_eq!(e.find_map_to(|x| if x % 2 == 0 { Some(x) } else { None } ), None);

  // flat - FIXME
  // let c = D::from_iter(vec![C::from_iter(vec![1, 2]), C::from_iter(vec![3])]);
  // assert_equal(c.flat(), vec![1, 2, 3]);
  // let e = e_source.clone();
  // assert_equal(e.map(|&x| C::from_iter(vec![x, -x])).flat(), vec![]);

  // flat_map - FIXME
  // let a = a_source.clone();
  // assert_equal(a.flat_map(|&x| vec![x, -x]), vec![1, -1, 2, -2, 3, -3]);
  // let e = e_source.clone();
  // assert_equal(e.flat_map(|&x| vec![x, -x]), vec![]);

  // flat_map_to - FIXME
  // let a = a_source.clone();
  // assert_equal(a.flat_map_to(|x| vec![x, -x]), vec![1, -1, 2, -2, 3, -3]);
  // let e = e_source.clone();
  // assert_equal(e.flat_map_to(|x| vec![x, -x]), vec![]);

  // fold_to
  let a = a_source.clone();
  assert_eq!(a.fold_to(0, |acc, x| acc + x), 6);
  let e = e_source.clone();
  assert_eq!(e.fold_to(0, |acc, x| acc + x), 0);
}
