use std::fmt::Debug;

use cantrip::{Iterable, Slice};

use crate::extensions::util::from;

pub fn test_slice<'a, C>()
where
  C: Slice<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  let values = from::<C>(&[0, 1, 2]);
  let empty = from::<C>(&[]);

  // init
  assert_eq!(values.init(), &from::<C>(&[0, 1]));
  assert_eq!(empty.init(), &from::<C>(&[]));
}
