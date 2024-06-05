use std::fmt::Debug;

use cantrip::{Iterable, Reversible};

use crate::extensions::util::from;

pub fn test_reversible<'a, C>()
where
  C: Reversible<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + PartialEq + Debug + 'a,
{
  let values = from::<C>(&[0, 1, 1, 2]);
  let empty = from::<C>(&[]);

  // rposition
  assert_eq!(values.rposition(|&x| x == 1), Some(2));
  assert_eq!(empty.rposition(|&x| x == 0), None);
}
