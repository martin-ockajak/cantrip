use cantrip::TransformVec;

use crate::extensions::util::{assert_set_equal, TestCollection};

pub(crate) fn test_transform_vec<'a, C>(sequence: bool, a_source: &C, e_source: &C)
where
  C: TransformVec<i64> + TestCollection<i64> + IntoIterator<Item = i64> + 'a,
{
  // into_vec
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.into_vec(), vec![1, 2, 3]);
  } else {
    assert_set_equal(a.into_vec(), vec![1, 2, 3]);
  }
  assert_eq!(e.into_vec(), vec![]);

  // to_vec
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.to_vec(), vec![1, 2, 3]);
  } else {
    assert_set_equal(a.to_vec(), vec![1, 2, 3]);
  }
  assert_eq!(e.to_vec(), vec![]);
}
