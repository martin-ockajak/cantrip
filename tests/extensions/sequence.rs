use std::fmt::Debug;

use cantrip::{Iterable, Sequence};

use crate::extensions::util::from;

pub fn test_sequence<'a, C>()
where
  C: Sequence<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  let values = from::<C>(&[1, 2, 2, 3]);
  // let empty = from::<C>(&[]);

  // position
  assert_eq!(values.position(|&x| x == 2), Some(1));
}
