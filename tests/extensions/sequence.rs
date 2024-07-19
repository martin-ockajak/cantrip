use cantrip::{Iterable, Sequence};
use std::fmt::Debug;
use std::panic;
use std::panic::UnwindSafe;

use crate::extensions::util::{assert_seq_equal, assert_vec_seq_equal, Equal};

#[allow(box_pointers)]
pub(crate) fn test_sequence<'a, C, I>()
where
  I: DoubleEndedIterator<Item = i64>,
  C: Sequence<i64>
    + FromIterator<i64>
    + IntoIterator<Item = i64, IntoIter = I>
    + Iterable<Item<'a> = &'a i64>
    + Clone
    + Equal
    + Debug
    + 'a,
  for<'c> &'c C: UnwindSafe,
{
  // FIXME - implement test for all trait methods
  let a_source = C::from_iter(vec![1, 2, 3]);
  let b_source = C::from_iter(vec![1, 2, 2, 3]);
  let e_source = C::from_iter(vec![]);

  // add_at
  let b = b_source.clone();
  let e = e_source.clone();
  assert_seq_equal(b.add_at(0, 3), vec![3, 1, 2]);
  let b = b_source.clone();
  assert_seq_equal(b.add_at(1, 3), vec![1, 3, 2]);
  let b = b_source.clone();
  assert_seq_equal(b.add_at(2, 3), vec![1, 2, 3]);
  assert_seq_equal(e.add_at(0, 1), vec![1]);
  let b = b_source.clone();
  assert_seq_equal(b.add_at(3, 3), vec![1, 2]);
  let e = e_source.clone();
  assert_seq_equal(e.add_at(1, 1), vec![]);

  // add_at_multi
  let b = b_source.clone();
  let e = e_source.clone();
  assert_seq_equal(b.add_at_multi(0, vec![3, 4]), vec![3, 4, 1, 2]);
  let b = b_source.clone();
  assert_seq_equal(b.add_at_multi(1, vec![3, 4]), vec![1, 3, 4, 2]);
  let b = b_source.clone();
  assert_seq_equal(b.add_at_multi(2, vec![3, 4]), vec![1, 2, 3, 4]);
  assert_seq_equal(e.add_at_multi(0, vec![1, 2]), vec![1, 2]);
  let b = b_source.clone();
  assert_seq_equal(b.add_at_multi(3, vec![3, 4]), vec![1, 2]);
  let e = e_source.clone();
  assert_seq_equal(e.add_at_multi(1, vec![1, 2]), vec![]);

  // cartesian_product
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.cartesian_product(0), vec![vec![]]);
  assert_vec_seq_equal(a.cartesian_product(1), vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(
    a.cartesian_product(2),
    vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 1], vec![2, 2], vec![2, 3], vec![3, 1], vec![3, 2], vec![3, 3]],
  );
  assert_vec_seq_equal(e.cartesian_product(2), vec![]);

  // chunked
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.chunked(3), vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked(2), vec![vec![1, 2], vec![3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked(1), vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(e.chunked(1), vec![]);
  assert!(panic::catch_unwind(|| {
    let a = a_source.clone();
    a.chunked(0)
  }).is_err());

  // chunked_by
  let a = a_source.clone();
  let e = e_source.clone();
  let chunked_by = a.chunked_by(|&p, &n| p > 0 && n > 2);
  assert_vec_seq_equal(chunked_by, vec![vec![1, 2], vec![3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_by(|_, _| false), vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_by(|_, _| true), vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(e.chunked_by(|_, _| true), vec![]);

  // chunked_exact
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.chunked_exact(2), vec![vec![1, 2]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_exact(3), vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_exact(1), vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(e.chunked_exact(1), vec![]);
  assert!(panic::catch_unwind(|| {
    let a = a_source.clone();
    a.chunked_exact(0)
  }).is_err());

  let c = vec![1, 1, 2, 1, 2, 2, 3];
  let e = e_source.clone();
  assert_seq_equal(c.coalesce(|p, n| if p % 2 == n % 2 { Ok(p + n) } else { Err((p, n)) }), vec![4, 1, 4, 3]);
  assert_seq_equal(e.coalesce(|p, n| if p % 2 == n % 2 { Ok(p + n) } else { Err((p, n)) }), vec![]);

  // combinations_multi
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.combinations_multi(0), vec![vec![]]);
  assert_vec_seq_equal(a.combinations_multi(1), vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(
    a.combinations_multi(2),
    vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 2], vec![2, 3], vec![3, 3]],
  );
  assert_vec_seq_equal(
    a.combinations_multi(3),
    vec![
      vec![1, 1, 1],
      vec![1, 1, 2],
      vec![1, 1, 3],
      vec![1, 2, 2],
      vec![1, 2, 3],
      vec![1, 3, 3],
      vec![2, 2, 2],
      vec![2, 2, 3],
      vec![2, 3, 3],
      vec![3, 3, 3],
    ],
  );
  assert_vec_seq_equal(e.combinations_multi(2), vec![]);

  // delete_at
  let a = a_source.clone();
  assert_seq_equal(a.delete_at(0), vec![2, 3]);
  let a = a_source.clone();
  assert_seq_equal(a.delete_at(1), vec![1, 3]);
  let a = a_source.clone();
  assert_seq_equal(a.delete_at(2), vec![1, 2]);
  assert!(panic::catch_unwind(|| {
    let e = e_source.clone();
    e.delete_at(0)
  }).is_err());

  // delete_at_multi
  let a = a_source.clone();
  assert_seq_equal(a.delete_at_multi(vec![0, 2]), vec![2]);
  let a = a_source.clone();
  assert_seq_equal(a.delete_at_multi(vec![0, 1, 2]), vec![]);
  assert!(panic::catch_unwind(|| {
    let e = e_source.clone();
    e.delete_at_multi(vec![0])
  }).is_err());

  // // rev
  // assert_equal(repeated.clone().rev(), vec![3, 2, 2, 1]);
  // assert_equal(empty.clone().rev(), vec![]);
  //
  // // tail
  // assert_equal(repeated.clone().tail(), vec![2, 2, 3]);
}
