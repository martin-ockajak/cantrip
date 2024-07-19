use std::collections::HashMap;
use std::fmt::Debug;

use cantrip::{Collectible, Iterable};
use crate::extensions::util::{assert_map_equal, assert_map_vec_equivalent, assert_seq_equal, assert_set_equal, assert_vec_seq_equivalent, Equal};

pub(crate) fn test_collectible<'a, C>(sequence: bool)
where
  C: Collectible<i64>
    + FromIterator<i64>
    + Iterable<Item<'a> = &'a i64>
    + Default
    + Extend<i64>
    + Clone
    + Equal
    + Debug
    + 'a,
  C::This<i64>: FromIterator<i64> + Default + Extend<i64> + Equal + Debug,
{
  let a_source = C::from_iter(vec![1, 2, 3]);
  let b_source = C::from_iter(vec![1, 2, 2, 3]);
  let e_source = C::from_iter(vec![]);

  // add
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_seq_equal(a.add(3), vec![1, 2, 3, 3]);
  } else {
    assert_set_equal(a.add(3), vec![1, 2, 3]);
  }
  assert_seq_equal(e.add(1), vec![1]);

  // add_multi
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_seq_equal(a.add_multi(vec![3, 4]), vec![1, 2, 3, 3, 4]);
  } else {
    assert_set_equal(a.add_multi(vec![3, 4]), vec![1, 2, 3, 4]);
  }
  assert_seq_equal(e.add(1), vec![1]);

  // combinations
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equivalent(a.combinations(0), vec![vec![]]);
  assert_vec_seq_equivalent(a.combinations(1), vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equivalent(a.combinations(2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
  assert_vec_seq_equivalent(a.combinations(3), vec![vec![1, 2, 3]]);
  assert_vec_seq_equivalent(a.combinations(4), vec![]);
  assert_vec_seq_equivalent(e.combinations(2), vec![]);

  // delete
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_seq_equal(b.delete(&2), vec![1, 2, 3]);
    let b = b_source.clone();
    assert_seq_equal(b.delete(&4), vec![1, 2, 2, 3]);
  } else {
    assert_seq_equal(a.delete(&2), vec![1, 3]);
    let a = a_source.clone();
    assert_set_equal(a.delete(&4), vec![1, 2, 3]);
  }
  assert_seq_equal(e.delete(&2), vec![]);

  // delete_multi
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_seq_equal(b.delete_multi(&vec![1, 2]), vec![2, 3]);
    let b = b_source.clone();
    assert_seq_equal(b.delete_multi(&vec![4]), vec![1, 2, 2, 3]);
  } else {
    assert_seq_equal(a.delete_multi(&vec![1, 2]), vec![3]);
    let a = a_source.clone();
    assert_seq_equal(a.delete_multi(&vec![4]), vec![1, 2, 3]);
  }
  assert_seq_equal(e.delete_multi(&vec![1]), vec![]);

  // fill_with
  let b = b_source.clone();
  if sequence {
    assert_seq_equal(C::fill_with(|| 1, 2), vec![1, 1]);
  } else {
    assert_seq_equal(C::fill_with(|| 1, 2), vec![1]);
  }
  assert_seq_equal(C::fill_with(|| 1, 0), vec![]);

  // filter
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.filter(|&x| x > 1), vec![2, 3]);
  assert_seq_equal(e.filter(|&x| x > 1), vec![]);

  // filter_map
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.filter_map(|&x| if x % 2 == 0 { Some(x + 1) } else { None }), vec![3]);
  assert_seq_equal(e.filter_map(|&x| if x % 2 == 0 { Some(x + 1) } else { None }), vec![]);

  // filter_map_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.filter_map_to(|x| if x % 2 == 0 { Some(x + 1) } else { None }), vec![3]);
  assert_seq_equal(e.filter_map_to(|x| if x % 2 == 0 { Some(x + 1) } else { None }), vec![]);

  // find_map_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.find_map_to(|x| if x % 2 == 0 { Some(x) } else { None }), Some(2));
  assert_eq!(e.find_map_to(|x| if x % 2 == 0 { Some(x) } else { None }), None);

  // flat - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // let c = D::from_iter(vec![C::from_iter(vec![1, 2]), C::from_iter(vec![3])]);
  // assert_equal(c.flat(), vec![1, 2, 3]);
  // assert_equal(e.map(|&x| C::from_iter(vec![x, -x])).flat(), vec![]);

  // flat_map - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_equal(a.flat_map(|&x| vec![x, -x]), vec![1, -1, 2, -2, 3, -3]);
  // assert_equal(e.flat_map(|&x| vec![x, -x]), vec![]);

  // flat_map_to - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_equal(a.flat_map_to(|x| vec![x, -x]), vec![1, -1, 2, -2, 3, -3]);
  // assert_equal(e.flat_map_to(|x| vec![x, -x]), vec![]);

  // fold_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.fold_to(0, |acc, x| acc + x), 6);
  assert_eq!(e.fold_to(0, |acc, x| acc + x), 0);

  // group_by
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    // FIXME - fix the test case
    // let b = a_source.clone();
    // assert_map_vec_equivalent(b.group_by(|x| x % 2), HashMap::from([(0, vec![2, 2]), (1, vec![1, 3])]));
  } else {
    assert_map_vec_equivalent(a.group_by(|x| x % 2), HashMap::from([
      (0, vec![2]),
      (1, vec![1, 3])
    ]));
  }
  assert_map_vec_equivalent(e.group_by(|x| x % 2), HashMap::new());

  // group_fold_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_map_equal(a.group_fold_to(|x| x % 2, 0, |acc, x| acc + x), HashMap::from([
    (0, 2),
    (1, 4)
  ]));
  assert_map_equal(e.group_fold_to(|x| x % 2, 0, |acc, x| acc + x), HashMap::new());

  // group_reduce_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.group_reduce_to(|x| x % 2, |acc, x| acc + x), HashMap::from([
    (0, 2),
    (1, 4),
  ]));
  assert_eq!(e.group_reduce_to(|x| x % 2, |acc, x| acc + x), HashMap::new());

  // intersect
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_seq_equal(b.intersect(&vec![4, 3, 2, 2, 5]), vec![2, 2, 3]);
  } else {
    assert_seq_equal(a.intersect(&vec![4, 3, 2, 2, 5]), vec![2, 3]);
  }
  assert_seq_equal(e.intersect(&vec![1]), vec![]);

  // map
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.map(|&x| x + 1), vec![2, 3, 4]);
  assert_seq_equal(e.map(|&x| x + 1), vec![]);

  // map_to
  assert_seq_equal(a.map(|x| x + 1), vec![2, 3, 4]);
  assert_seq_equal(e.map(|x| x + 1), vec![]);

  // largest
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.largest(2), vec![3, 2]);
  let a = a_source.clone();
  assert_seq_equal(a.largest(4), vec![3, 2, 1]);
  assert_seq_equal(e.largest(3), vec![]);

  // partition
  let a = a_source.clone();
  let e = e_source.clone();
  let (a_even, a_odd) = a.partition(|n| n % 2 == 0);
  assert_seq_equal(a_even, vec![2]);
  assert_seq_equal(a_odd, vec![1, 3]);
  let (e_even, e_odd) = e.partition(|n| n % 2 == 0);
  assert_seq_equal(e_even, vec![]);
  assert_seq_equal(e_odd, vec![]);

  // partition_map
  let a = a_source.clone();
  let e = e_source.clone();
  let (a_even, a_odd) = a.partition_map(|&x| if x % 2 == 0 { Ok(x + 3) } else { Err(x) });
  assert_seq_equal(a_even, vec![5]);
  assert_seq_equal(a_odd, vec![1, 3]);
  let (e_even, e_odd) = e.partition_map(|&x| if x % 2 == 0 { Ok(x + 3) } else { Err(x) });
  assert_seq_equal(e_even, vec![]);
  assert_seq_equal(e_odd, vec![]);

  // partition_map_to
  let a = a_source.clone();
  let e = e_source.clone();
  let (a_even, a_odd) = a.partition_map_to(|n| if n % 2 == 0 { Ok(n + 3) } else { Err(n) });
  assert_seq_equal(a_even, vec![5]);
  assert_seq_equal(a_odd, vec![1, 3]);
  let (e_even, e_odd) = e.partition_map_to(|x| if x % 2 == 0 { Ok(x + 3) } else { Err(x) });
  assert_seq_equal(e_even, vec![]);
  assert_seq_equal(e_odd, vec![]);

  // powerset
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equivalent(a.powerset(), vec![
    vec![],
    vec![1], vec![2], vec![3],
    vec![1, 2], vec![1, 3], vec![2, 3],
    vec![1, 2, 3]]
  );
  assert_vec_seq_equivalent(e.powerset(), vec![vec![]]);

  // product
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(b.product(), 12);
  } else {
    assert_eq!(a.product(), 6);
  }
  assert_eq!(e.product(), 1);

  // reduce_to
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.reduce_to(|acc, e| acc + e), Some(6));
  assert_eq!(e.reduce_to(|acc, e| acc + e), None);

  // smallest
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.smallest(2), vec![1, 2]);
  let a = a_source.clone();
  assert_seq_equal(a.smallest(4), vec![1, 2, 3]);
  assert_seq_equal(e.smallest(3), vec![]);
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();

  // substitute
  if sequence {
    assert_seq_equal(b.substitute(&2, 4), vec![1, 4, 2, 3]);
    let b = b_source.clone();
    assert_set_equal(b.substitute(&4, 5), vec![1, 2, 2, 3]);
  } else {
    assert_set_equal(a.substitute(&2, 4), vec![1, 4, 3]);
    let a = a_source.clone();
    assert_seq_equal(a.substitute(&4, 5), vec![1, 2, 3]);
  }
  assert_seq_equal(e.substitute(&1, 2), vec![]);

  // substitute_multi
  let a = a_source.clone();
  let b = b_source.clone();
  // let e = e_source.clone();
  if sequence {
    assert_seq_equal(b.substitute_multi(&vec![2, 3], vec![4, 5]), vec![1, 4, 2, 5]);
    let b = b_source.clone();
    assert_seq_equal(b.substitute_multi(&vec![2, 2], vec![4, 5]), vec![1, 4, 5, 3]);
    // let b = b_source.clone();
    // assert_seq_equal(b.substitute_multi(&vec![2, 4], vec![4, 5]), vec![1, 4, 2, 3]);
    let b = b_source.clone();
    assert_seq_equal(b.substitute_multi(&vec![4, 5], vec![1, 1]), vec![1, 2, 2, 3]);
  } else {
    assert_set_equal(a.substitute_multi(&vec![2, 3], vec![4, 5]), vec![1, 4, 5]);
    // FIXME - fix the test case
    // let a = a_source.clone();
    // assert_set_equal(a.substitute_multi(&vec![2, 2], vec![4, 5]), vec![1, 5, 3]);
    // let a = a_source.clone();
    // assert_set_equal(a.substitute_multi(&vec![2, 4], vec![4, 5]), vec![1, 4, 3]);
    let a = a_source.clone();
    assert_set_equal(a.substitute_multi(&vec![4, 5], vec![1, 1]), vec![1, 2, 3]);
  }
  // assert_seq_equal(e.substitute_multi(&vec![1], vec![2]), vec![]);

  // sum
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(b.sum(), 8);
  } else {
    assert_eq!(a.sum(), 6);
  }
  assert_eq!(e.sum(), 0);
  
  // unit
  assert_seq_equal(C::unit(1), vec![1]);
}
