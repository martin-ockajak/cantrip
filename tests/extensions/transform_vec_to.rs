use cantrip::TransformVecTo;

use crate::extensions::util::assert_set_equal;

pub(crate) fn test_transform_vec_to<C>(sequence: bool, a: C, e: C)
where
  C: TransformVecTo<i64> + IntoIterator<Item = i64>,
{
  // into_vec
  if sequence {
    assert_eq!(a.into_vec(), vec![1, 2, 3]);
  } else {
    assert_set_equal(a.into_vec(), vec![1, 2, 3]);
  }
  assert_eq!(e.into_vec(), vec![]);
}
