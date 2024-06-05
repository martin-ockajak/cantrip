use std::fmt::Debug;

use cantrip::{Iterable, Sequence};

use crate::extensions::util::{Equal, from_slice};

pub fn test_sequence<'a, C>()
where
  C: Sequence<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  let values = from_slice::<C>(&[1, 2, 2, 3]);
  // let empty = from::<C>(&[]);

  // position
  assert_eq!(values.position(|&x| x == 2), Some(1));
}
