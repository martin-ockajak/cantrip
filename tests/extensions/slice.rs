use std::fmt::Debug;

use cantrip::{Iterable, Slice};

use crate::extensions::util::from_slice;

pub fn test_slice<'a, C>()
where
  C: Slice<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let values = from_slice::<C>(&[0, 1, 2]);
  let empty = from_slice::<C>(&[]);

  // init
  assert_eq!(values.init(), &from_slice::<C>(&[0, 1]));
  assert_eq!(empty.init(), &from_slice::<C>(&[]));
}
