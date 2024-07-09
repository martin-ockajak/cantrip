use std::fmt::Debug;

use cantrip::{Iterable, Sequence};

use crate::extensions::util::{Equal, from_slice};

pub(crate) fn test_sequence<'a, C>()
where
  C: Sequence<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let repeated = from_slice::<C>(&[1, 2, 2, 3]);
  // let empty = from::<C>(&[]);

  // position
  assert_eq!(repeated.position(|&x| x == 2), Some(1));
}
