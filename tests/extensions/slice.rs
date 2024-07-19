use cantrip::Slice;

pub(crate) fn test_slice(a: &[i64], b: &[i64], e: &[i64])
{
  // init
  assert_eq!(a.init(), [1, 2]);
  assert_eq!(b.init(), [1, 2, 2]);
  assert_eq!(e.init(), []);

  // skip
  assert_eq!(a.skip(2), [3]);
  assert_eq!(b.skip(2), [2, 3]);
  assert_eq!(e.skip(2), []);

  // skip_while
  assert_eq!(a.skip_while(|&x| x < 3), [3]);
  assert_eq!(e.skip_while(|&x| x < 3), []);

  // tail
  assert_eq!(a.tail(), [2, 3]);
  assert_eq!(b.tail(), [2, 2, 3]);
  assert_eq!(e.tail(), []);

  // take
  assert_eq!(a.take(2), [1, 2]);
  assert_eq!(a.take(5), [1, 2, 3]);
  assert_eq!(e.take(2), []);

  // take_while
  assert_eq!(a.take_while(|&x| x < 3), [1, 2]);
  assert_eq!(e.take_while(|&x| x < 3), []);
}
