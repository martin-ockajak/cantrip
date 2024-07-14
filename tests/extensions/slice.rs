use std::fmt::Debug;

use cantrip::{Iterable, Slice};

pub(crate) fn test_slice<'a, C>()
where
  C: Slice<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let values = C::from_iter(vec![0, 1, 2]);
  let empty = C::from_iter(vec![]);

  // init
  assert_eq!(values.init(), &C::from_iter(vec![0, 1]));
  assert_eq!(empty.init(), &C::from_iter(vec![]));
}
