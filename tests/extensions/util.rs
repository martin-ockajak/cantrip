use std::fmt::Debug;

pub(crate) fn assert_equal<C: FromIterator<i64> + PartialEq + Debug>(values: C, expected: &[i64]) -> () {
  assert_eq!(values, from(expected))
}

pub(crate) fn from<C: FromIterator<i64>>(values: &[i64]) -> C {
  C::from_iter(values.into_iter().map(|x| x.clone()))
}
