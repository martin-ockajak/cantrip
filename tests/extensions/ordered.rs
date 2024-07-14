use std::fmt::Debug;

use cantrip::{Iterable, Ordered};

use crate::extensions::util::{Equal, from_slice};

pub(crate) fn test_ordered<'a, C>()
where
  C: Ordered<i64> + FromIterator<i64> + Iterable<Item<'a> = &'a i64> + Clone + Equal + Debug + 'a,
{
  // FIXME - implement test for all trait methods
  let a_source = from_slice::<C>(&[1, 2, 3]);
  let b_source = from_slice::<C>(&[1, 2, 2, 3]);
  let e_source = from_slice::<C>(&[]);
  let a = a_source.clone();

  // common_prefix_length
  assert_eq!(a.common_prefix_length(&vec![1, 2, 3, 4]), 3);
  assert_eq!(a.common_prefix_length(&vec![1, 2]), 2);
  assert_eq!(a.common_prefix_length(&vec![]), 0);
  
  // common_suffix_length
  assert_eq!(a.common_suffix_length(&vec![0, 1, 2, 3]), 3);
  assert_eq!(a.common_suffix_length(&vec![2, 3]), 2);
  assert_eq!(a.common_suffix_length(&vec![]), 0);

  // position
  let b = b_source.clone();
  assert_eq!(b.position(|&x| x == 2), Some(1));
  let e = e_source.clone();
  assert_eq!(e.position(|&x| x == 0), None);

  // rposition
  let b = b_source.clone();
  assert_eq!(b.rposition(|&x| x == 2), Some(2));
  let e = e_source.clone();
  assert_eq!(e.rposition(|&x| x == 0), None);
}
