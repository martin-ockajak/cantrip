use std::panic;
use std::panic::UnwindSafe;

use cantrip::SequenceTo;

use crate::extensions::util::{TestCollectible, TestCollection, TestSequence, assert_seq_equal, assert_vec_seq_equal};

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::too_many_lines)]
pub(crate) fn test_sequence_to<'a, C, G, I>(a_source: &C, b_source: &C, c_source: &C, g_source: &G, e_source: &C)
where
  I: DoubleEndedIterator<Item = i64> + ExactSizeIterator<Item = i64>,
  C: TestSequence<'a, i64, I> + UnwindSafe,
  <C as SequenceTo<i64>>::This<i64>: TestCollection<i64>,
  <C as SequenceTo<i64>>::This<(i64, i64)>: TestCollection<(i64, i64)>,
  <C as SequenceTo<i64>>::This<(usize, i64)>: TestCollection<(usize, i64)>,
  G: SequenceTo<(i64, i64)> + TestCollectible<'a, (i64, i64)>,
  <G as SequenceTo<(i64, i64)>>::This<i64>: TestCollection<i64>,
  for<'c> &'c C: UnwindSafe,
{
  // add_at
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.add_at(0, 4), vec![4, 1, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.add_at(1, 4), vec![1, 4, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.add_at(3, 4), vec![1, 2, 3, 4]);
  assert_seq_equal(&e.add_at(0, 1), vec![1]);
  let a = a_source.clone();
  assert!(panic::catch_unwind(|| { a.add_at(4, 1) }).is_err());

  // add_at_multi
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.add_at_multi(0, vec![4, 5]), vec![4, 5, 1, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.add_at_multi(1, vec![4, 5]), vec![1, 4, 5, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.add_at_multi(3, vec![4, 5]), vec![1, 2, 3, 4, 5]);
  assert_seq_equal(&e.add_at_multi(0, vec![1, 2]), vec![1, 2]);
  let a = a_source.clone();
  assert!(panic::catch_unwind(|| { a.add_at_multi(4, vec![1, 2]) }).is_err());

  // cartesian_product
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.cartesian_product(0), &vec![vec![]]);
  assert_vec_seq_equal(a.cartesian_product(1), &vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(
    a.cartesian_product(2),
    &vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 1], vec![2, 2], vec![2, 3], vec![3, 1], vec![3, 2], vec![3, 3]],
  );
  assert_vec_seq_equal(e.cartesian_product(2), &vec![]);

  // chunked
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.chunked(3), &vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked(2), &vec![vec![1, 2], vec![3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked(1), &vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(e.chunked(1), &vec![]);
  let a = a_source.clone();
  assert!(panic::catch_unwind(|| { a.chunked(0) }).is_err());

  // chunked_by
  let a = a_source.clone();
  let e = e_source.clone();
  let chunked_by = a.chunked_by(|&p, &n| p > 0 && n > 2);
  assert_vec_seq_equal(chunked_by, &vec![vec![1, 2], vec![3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_by(|_, _| false), &vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_by(|_, _| true), &vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(e.chunked_by(|_, _| true), &vec![]);

  // chunked_exact
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.chunked_exact(2), &vec![vec![1, 2]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_exact(3), &vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.chunked_exact(1), &vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(e.chunked_exact(1), &vec![]);
  let a = a_source.clone();
  assert!(panic::catch_unwind(|| { a.chunked_exact(0) }).is_err());

  let c = vec![1, 1, 2, 1, 2, 2, 3];
  let e = e_source.clone();
  assert_seq_equal(&c.coalesce(|p, n| if p % 2 == n % 2 { Ok(p + n) } else { Err((p, n)) }), vec![4, 1, 4, 3]);
  assert_seq_equal(&e.coalesce(|p, n| if p % 2 == n % 2 { Ok(p + n) } else { Err((p, n)) }), vec![]);

  // combinations_multi
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.combinations_multi(0), &vec![vec![]]);
  assert_vec_seq_equal(a.combinations_multi(1), &vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(
    a.combinations_multi(2),
    &vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 2], vec![2, 3], vec![3, 3]],
  );
  assert_vec_seq_equal(
    a.combinations_multi(3),
    &vec![
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
  assert_vec_seq_equal(e.combinations_multi(1), &vec![]);

  // delete_at
  let a = a_source.clone();
  assert_seq_equal(&a.delete_at(0), vec![2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.delete_at(1), vec![1, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.delete_at(2), vec![1, 2]);
  let e = e_source.clone();
  assert!(panic::catch_unwind(|| { e.delete_at(0) }).is_err());

  // delete_at_multi
  let a = a_source.clone();
  assert_seq_equal(&a.delete_at_multi(vec![0, 2]), vec![2]);
  let a = a_source.clone();
  assert_seq_equal(&a.delete_at_multi(vec![0, 1, 2]), vec![]);
  let e = e_source.clone();
  assert!(panic::catch_unwind(|| { e.delete_at_multi(vec![0]) }).is_err());

  // divide
  let a = a_source.clone();
  assert_vec_seq_equal(a.divide(&2), &vec![vec![1], vec![3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.divide(&0), &vec![vec![1, 2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.divide(&1), &vec![vec![], vec![2, 3]]);
  let b = b_source.clone();
  assert_vec_seq_equal(b.divide(&2), &vec![vec![1], vec![], vec![3]]);

  // divide_by
  let b = b_source.clone();
  assert_vec_seq_equal(b.divide_by(|x| x % 2 == 0), &vec![vec![1], vec![], vec![3]]);

  // duplicates
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.duplicates(), vec![2]);
  assert_seq_equal(&e.duplicates(), vec![]);

  // duplicates
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.duplicates(), vec![2]);
  assert_seq_equal(&e.duplicates(), vec![]);

  // duplicates_by
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.duplicates_by(|x| x % 2), vec![1, 3]);
  assert_seq_equal(&e.duplicates_by(|x| x % 2), vec![]);

  // enumerate
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.enumerate(), vec![(0, 1), (1, 2), (2, 3)]);
  assert_seq_equal(&e.enumerate(), vec![]);

  // fill
  assert_eq!(Vec::fill(1, 2), vec![1, 1]);
  assert_eq!(Vec::fill(1, 0), vec![]);

  // init
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.init(), vec![1, 2]);
  assert_seq_equal(&e.init(), vec![]);

  // interleave
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.interleave(vec![4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
  let a = a_source.clone();
  assert_seq_equal(&a.interleave(vec![4, 5]), vec![1, 4, 2, 5, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.interleave(vec![]), vec![1, 2, 3]);
  assert_seq_equal(&e.interleave(vec![1]), vec![]);

  // interleave_exact
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.interleave_exact(vec![4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
  let a = a_source.clone();
  assert_seq_equal(&a.interleave_exact(vec![4, 5]), vec![1, 4, 2, 5]);
  let a = a_source.clone();
  assert_seq_equal(&a.interleave_exact(vec![]), vec![]);
  assert_seq_equal(&e.interleave_exact(vec![1]), vec![]);

  // intersperse
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.intersperse(1, 0), vec![1, 0, 2, 0, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.intersperse(2, 0), vec![1, 2, 0, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.intersperse(3, 0), vec![1, 2, 3]);
  assert_seq_equal(&e.intersperse(1, 0), vec![]);

  // intersperse_with
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.intersperse_with(2, || 0), vec![1, 2, 0, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.intersperse_with(3, || 0), vec![1, 2, 3]);
  assert_seq_equal(&e.intersperse_with(1, || 0), vec![]);

  // map_while
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.map_while(|&x| if x < 2 { Some(x + 1) } else { None }), vec![2, 3]);
  assert_seq_equal(&e.map_while(|&x| if x < 2 { Some(x + 1) } else { None }), vec![]);

  // merge
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.merge(vec![0, 4, 5]), vec![0, 1, 2, 3, 4, 5]);
  assert_seq_equal(&e.merge(vec![1]), vec![1]);
  let e = e_source.clone();
  assert_seq_equal(&e.merge(vec![]), vec![]);

  // merge_by
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.merge_by(vec![0, 4, 5], Ord::cmp), vec![0, 1, 2, 3, 4, 5]);
  assert_seq_equal(&e.merge_by(vec![1], Ord::cmp), vec![1]);
  let e = e_source.clone();
  assert_seq_equal(&e.merge_by(vec![], Ord::cmp), vec![]);

  // move_at
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.move_at(0, 2), vec![2, 3, 1]);
  let a = a_source.clone();
  assert_seq_equal(&a.move_at(2, 1), vec![1, 3, 2]);
  let a = a_source.clone();
  assert_seq_equal(&a.move_at(1, 1), vec![1, 2, 3]);
  assert!(panic::catch_unwind(|| { e.move_at(0, 0) }).is_err());

  // pad_left
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.pad_left(5, 4), vec![4, 4, 1, 2, 3]);
  assert_seq_equal(&e.pad_left(1, 1), vec![1]);

  // pad_left_with
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.pad_left_with(5, |i| 2 * i as i64), vec![0, 2, 1, 2, 3]);
  assert_seq_equal(&e.pad_left_with(1, |i| 2 * i as i64), vec![0]);

  // pad_right
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.pad_right(5, 4), vec![1, 2, 3, 4, 4]);
  assert_seq_equal(&e.pad_right(1, 1), vec![1]);

  // pad_right_with
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.pad_right_with(5, |i| 2 * i as i64), vec![1, 2, 3, 6, 8]);
  assert_seq_equal(&e.pad_right_with(1, |i| 2 * i as i64), vec![0]);

  // rev
  let a = a_source.clone();
  let b = b_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.rev(), vec![3, 2, 1]);
  assert_seq_equal(&b.rev(), vec![3, 2, 2, 1]);
  assert_seq_equal(&e.rev(), vec![]);

  // rfold
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.rfold(0, |acc, x| acc + x), 6);
  assert_eq!(e.rfold(0, |acc, x| acc + x), 0);

  // scan
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(
    &a.scan_ref(1, |state, x| {
      *state *= x;
      if *state > 2 {
        return None;
      }
      Some(-*state)
    }),
    vec![-1_i64, -2],
  );
  assert_seq_equal(&e.scan(1, |_, x| Some(x)), vec![]);

  // scan_ref
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(
    &a.scan_ref(1, |state, &x| {
      *state *= x;
      if *state > 2 {
        return None;
      }
      Some(-*state)
    }),
    vec![-1_i64, -2],
  );
  assert_seq_equal(&e.scan_ref(1, |_, &x| Some(x)), vec![]);

  // skip
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.skip(2), vec![3]);
  let a = a_source.clone();
  assert_seq_equal(&a.skip(5), vec![]);
  assert_seq_equal(&e.skip(1), vec![]);

  // skip_while
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.skip_while(|&x| x < 3), vec![3]);
  assert_seq_equal(&e.skip_while(|&x| x < 3), vec![]);

  // slice
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.slice(0, 2), vec![1, 2]);
  let a = a_source.clone();
  assert_seq_equal(&a.slice(1, 3), vec![2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.slice(1, 1), vec![]);
  let a = a_source.clone();
  assert!(panic::catch_unwind(|| { a.slice(4, 5) }).is_err());
  let a = a_source.clone();
  assert!(panic::catch_unwind(|| { a.slice(1, 5) }).is_err());
  assert_seq_equal(&e.slice(0, 0), vec![]);

  // sorted
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted(), vec![1, 2, 3]);
  assert_seq_equal(&e.sorted(), vec![]);

  // sorted_by
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted_by(Ord::cmp), vec![1, 2, 3]);
  assert_seq_equal(&e.sorted_by(Ord::cmp), vec![]);

  // sorted_by_cached_key
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted_by_cached_key(ToString::to_string), vec![1, 2, 3]);
  assert_seq_equal(&e.sorted_by_cached_key(ToString::to_string), vec![]);

  // sorted_by_key
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted_by_key(|&k| -k), vec![3, 2, 1]);
  assert_seq_equal(&e.sorted_by_key(|&k| -k), vec![]);

  // sorted_unstable
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted(), vec![1, 2, 3]);
  assert_seq_equal(&e.sorted(), vec![]);

  // sorted_unstable_by
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted_unstable_by(Ord::cmp), vec![1, 2, 3]);
  assert_seq_equal(&e.sorted_unstable_by(Ord::cmp), vec![]);

  // sorted_unstable_by_key
  let c = c_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&c.sorted_unstable_by_key(|&k| -k), vec![3, 2, 1]);
  assert_seq_equal(&e.sorted_unstable_by_key(|&k| -k), vec![]);

  // step_by
  let b = b_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&b.step_by(3), vec![1, 3]);
  let b = b_source.clone();
  assert_seq_equal(&b.step_by(2), vec![1, 2]);
  let b = b_source.clone();
  assert_seq_equal(&b.step_by(1), vec![1, 2, 2, 3]);
  assert_seq_equal(&e.step_by(1), vec![]);

  // substitute_at
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.substitute_at(1, 4), vec![1, 4, 3]);
  assert!(panic::catch_unwind(|| { e.substitute_at(3, 1) }).is_err());

  // substitute_at_multi
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.substitute_at_multi(vec![0, 2], vec![4, 5]), vec![4, 2, 5]);
  let a = a_source.clone();
  assert_seq_equal(&a.substitute_at_multi(vec![0, 2], vec![4]), vec![4, 2, 3]);
  let a = a_source.clone();
  assert_seq_equal(&a.substitute_at_multi(vec![0, 2], vec![4, 5, 6]), vec![4, 2, 5]);
  assert!(panic::catch_unwind(|| { e.substitute_at_multi(vec![0, 4], vec![1, 2]) }).is_err());
  let e = e_source.clone();
  assert!(panic::catch_unwind(|| { e.substitute_at_multi(vec![3, 4], vec![1, 2]) }).is_err());

  // swap_at
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.swap_at(0, 2), vec![3, 2, 1]);
  let a = a_source.clone();
  assert_seq_equal(&a.swap_at(1, 1), vec![1, 2, 3]);
  assert!(panic::catch_unwind(|| { e.swap_at(0, 3) }).is_err());
  let e = e_source.clone();
  assert!(panic::catch_unwind(|| { e.swap_at(3, 4) }).is_err());

  // tail
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.tail(), vec![2, 3]);
  assert_seq_equal(&e.tail(), vec![]);

  // take
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.take(2), vec![1, 2]);
  let a = a_source.clone();
  assert_seq_equal(&a.take(5), vec![1, 2, 3]);
  assert_seq_equal(&e.take(1), vec![]);

  // take_while
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.take_while(|&x| x < 3), vec![1, 2]);
  assert_seq_equal(&e.take_while(|&x| x < 3), vec![]);

  // unique
  let b = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&b.unique(), vec![1, 2, 3]);
  assert_seq_equal(&e.unique(), vec![]);

  // unique_by
  let b = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&b.unique_by(|x| x % 2), vec![1, 2]);
  assert_seq_equal(&e.unique_by(|x| x % 2), vec![]);

  // unzip
  let g = g_source.clone();
  let e = g_source.clone().filter(|_| false);
  let (a_left, a_right) = g.unzip();
  assert_seq_equal(&a_left, vec![1, 2, 3]);
  assert_seq_equal(&a_right, vec![1, 2, 3]);
  let (e_left, e_right) = e.unzip();
  assert_seq_equal(&e_left, vec![]);
  assert_seq_equal(&e_right, vec![]);

  // variations
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.variations(0), &vec![vec![]]);
  assert_vec_seq_equal(a.variations(1), &vec![vec![1], vec![2], vec![3]]);
  assert_vec_seq_equal(a.variations(2), &vec![vec![1, 2], vec![1, 3], vec![2, 1], vec![2, 3], vec![3, 1], vec![3, 2]]);
  assert_vec_seq_equal(
    a.variations(3),
    &vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]],
  );
  assert_vec_seq_equal(e.variations(1), &vec![]);

  // windowed
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.windowed(2, 1), &vec![vec![1, 2], vec![2, 3]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.windowed(2, 2), &vec![vec![1, 2]]);
  assert_vec_seq_equal(e.windowed(1, 1), &vec![]);

  // windowed_circular
  let a = a_source.clone();
  let e = e_source.clone();
  assert_vec_seq_equal(a.windowed_circular(2, 1), &vec![vec![1, 2], vec![2, 3], vec![3, 1]]);
  let a = a_source.clone();
  assert_vec_seq_equal(a.windowed_circular(2, 2), &vec![vec![1, 2], vec![3, 1]]);
  assert_vec_seq_equal(e.windowed_circular(1, 1), &vec![]);

  // zip
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.zip(vec![4_i64, 5, 6]), vec![(1, 4), (2, 5), (3, 6)]);
  let a = a_source.clone();
  assert_seq_equal(&a.zip(vec![4_i64, 5]), vec![(1, 4), (2, 5)]);
  let a = a_source.clone();
  assert_seq_equal(&a.zip(vec![4_i64, 5, 6, 7]), vec![(1, 4), (2, 5), (3, 6)]);
  assert_seq_equal(&e.zip(vec![1_i64]), vec![]);

  // zip_padded
  let a = a_source.clone();
  let e = e_source.clone();
  assert_seq_equal(&a.zip_padded(vec![4_i64, 5, 6], || 1, || 2), vec![(1, 4), (2, 5), (3, 6)]);
  let a = a_source.clone();
  assert_seq_equal(&a.zip_padded(vec![4_i64, 5, 6, 7], || 1, || 2), vec![(1, 4), (2, 5), (3, 6), (1, 7)]);
  let a = a_source.clone();
  assert_seq_equal(&a.zip_padded(vec![4_i64, 5], || 1, || 2), vec![(1, 4), (2, 5), (3, 2)]);
  assert_seq_equal(&e.zip_padded(vec![1_i64], || 1, || 2), vec![(1, 1)]);
}
