use std::fmt::Debug;

use cantrip::{Iterable, Reversible};

use crate::extensions::util::from;

pub fn test_reversible<'a, C>()
where
  C: Reversible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  let values = from::<C>(&[1, 2, 3]);
  // let empty = from::<C>(&[]);

  // rposition
  assert_eq!(values.rposition(|&x| x == 2), Some(1));
}
