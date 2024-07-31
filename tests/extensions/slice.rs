use cantrip::Slice;

pub(crate) fn test_slice(a: &[i64], b: &[i64], e: &[i64]) {
  // init_ref
  assert_eq!(a.init_ref(), [1, 2]);
  assert_eq!(b.init_ref(), [1, 2, 2]);
  assert_eq!(e.init_ref(), []);

  // skip_ref
  assert_eq!(a.skip_ref(2), [3]);
  assert_eq!(b.skip_ref(2), [2, 3]);
  assert_eq!(e.skip_ref(2), []);

  // skip_while_ref
  assert_eq!(a.skip_while_ref(|&x| x < 3), [3]);
  assert_eq!(e.skip_while_ref(|&x| x < 3), []);

  // tail_ref
  assert_eq!(a.tail_ref(), [2, 3]);
  assert_eq!(b.tail_ref(), [2, 2, 3]);
  assert_eq!(e.tail_ref(), []);

  // take_ref
  assert_eq!(a.take_ref(2), [1, 2]);
  assert_eq!(a.take_ref(5), [1, 2, 3]);
  assert_eq!(e.take_ref(2), []);

  // take_while_ref
  assert_eq!(a.take_while_ref(|&x| x < 3), [1, 2]);
  assert_eq!(e.take_while_ref(|&x| x < 3), []);
}
