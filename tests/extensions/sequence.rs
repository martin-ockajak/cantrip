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
    + UnwindSafe
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
  })
  .is_err());

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
  })
  .is_err());

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
  })
  .is_err());

  // delete_at_multi
  let a = a_source.clone();
  assert_seq_equal(a.delete_at_multi(vec![0, 2]), vec![2]);
  let a = a_source.clone();
  assert_seq_equal(a.delete_at_multi(vec![0, 1, 2]), vec![]);
  assert!(panic::catch_unwind(|| {
    let e = e_source.clone();
    e.delete_at_multi(vec![0])
  })
  .is_err());

  // divide
  let a = a_source.clone();
  assert_vec_seq_equal(a.divide(&2), vec![vec![1], vec![3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.divide(&0), vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.divide(&1), vec![vec![], vec![2, 3]]);
  let b = b_source.clone();
  assert_vec_seq_equal(b.divide(&2), vec![vec![1], vec![], vec![3]]);

  // divide_by
  let b = b_source.clone();
  assert_vec_seq_equal(b.divide_by(|x| x % 2 == 0), vec![vec![1], vec![], vec![3]]);

  // duplicates
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.duplicates(), vec![2]);
  assert_seq_equal(e.duplicates(), vec![]);

  // duplicates
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.duplicates(), vec![2]);
  assert_seq_equal(e.duplicates(), vec![]);

  // duplicates_by - FIXME - fix the failing test case
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_seq_equal(a.duplicates_by(|x| x % 2), vec![1, 3]);
  // assert_seq_equal(e.duplicates_by(|x| x % 2), vec![]);

  // enumerate - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_seq_equal(a.enumerate(), vec![(0, 1), (1, 2), (2, 3)]);
  // assert_seq_equal(e.enumerate(), vec![]);

  // fill
  assert_eq!(Vec::fill(1, 2), vec![1, 1]);
  assert_eq!(Vec::fill(1, 0), vec![]);

  // init
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.init(), vec![1, 2]);
  assert_seq_equal(e.init(), vec![]);

  // interleave
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.interleave(vec![4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
  let a = a_source.clone();
  assert_seq_equal(a.interleave(vec![4, 5]), vec![1, 4, 2, 5, 3]);
  let a = a_source.clone();
  assert_seq_equal(a.interleave(vec![]), vec![1, 2, 3]);
  assert_seq_equal(e.interleave(vec![1]), vec![]);

  // interleave_exact
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.interleave_exact(vec![4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
  let a = a_source.clone();
  assert_seq_equal(a.interleave_exact(vec![4, 5]), vec![1, 4, 2, 5]);
  let a = a_source.clone();
  assert_seq_equal(a.interleave_exact(vec![]), vec![]);
  assert_seq_equal(e.interleave_exact(vec![1]), vec![]);

  // intersperse - FIXME - fix the failing test case
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_seq_equal(a.intersperse(1, 0), vec![1, 0, 2, 0, 3]);
  // let a = a_source.clone();
  // assert_seq_equal(a.intersperse(2, 0), vec![1, 2, 0, 3]);
  // let a = a_source.clone();
  // assert_seq_equal(a.intersperse(3, 0), vec![1, 2, 3]);
  // assert_seq_equal(e.intersperse(1, 0), vec![]);

  // intersperse_with - FIXME - fix the failing test case
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_seq_equal(a.intersperse_with(2, || 0), vec![1, 2, 0, 3]);
  // let a = a_source.clone();
  // assert_seq_equal(a.intersperse_with(3, || 0), vec![1, 2, 3]);
  // assert_seq_equal(e.intersperse_with(1, || 0), vec![]);

  // map_while - FIXME - implement test
  // assert_seq_equal(a.map_while(|&x| if x < 2 { Some(x + 1) } else { None }), vec![2, 3]);
  // assert_seq_equal(e.map_while(|&x| if x < 2 { Some(x + 1) } else { None }), vec![]);

  // merge
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.merge(vec![0, 4, 5]), vec![0, 1, 2, 3, 4, 5]);
  assert_seq_equal(e.merge(vec![1]), vec![1]);
  let e = e_source.clone();
  assert_seq_equal(e.merge(vec![]), vec![]);

  // merge_by
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.merge_by(vec![0, 4, 5], |l, r| l.cmp(r)), vec![0, 1, 2, 3, 4, 5]);
  assert_seq_equal(e.merge_by(vec![1], |l, r| l.cmp(r)), vec![1]);
  let e = e_source.clone();
  assert_seq_equal(e.merge_by(vec![], |l, r| l.cmp(r)), vec![]);

  // move_at
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(a.move_at(0, 2), vec![2, 3, 1]);
  let a = a_source.clone();
  assert_seq_equal(a.move_at(2, 1), vec![1, 3, 2]);
  let a = a_source.clone();
  assert_seq_equal(a.move_at(1, 1), vec![1, 2, 3]);
  assert!(panic::catch_unwind(|| { e.move_at(0, 0) }).is_err());

  // pad_left - FIXME - implement test
  // let a = a_source.clone();
  // let e = e_source.clone();
  // assert_seq_equal(a.pad_left(5, 4), vec![4, 4, 1, 2, 3]);
  // assert_seq_equal(e.pad_left(1, 1), vec![1]);

  // // rev
  // assert_equal(repeated.clone().rev(), vec![3, 2, 2, 1]);
  // assert_equal(empty.clone().rev(), vec![]);
  //
  // // tail
  // assert_equal(repeated.clone().tail(), vec![2, 2, 3]);
}
