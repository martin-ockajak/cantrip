use std::collections::HashMap;
use std::fmt::Debug;

use cantrip::{Iterable, Ordered};

use crate::extensions::util::Equal;

pub(crate) fn test_ordered<'a, C>(a: &C, b: &C, e: &C)
where
  C: Ordered<i64> + Iterable<Item<'a> = &'a i64> + Equal + Debug + ?Sized + 'a,
{
  // common_prefix_length
  assert_eq!(a.common_prefix_length(&vec![1, 2, 3, 4]), 3);
  assert_eq!(a.common_prefix_length(&vec![1, 2]), 2);
  assert_eq!(a.common_prefix_length(&vec![]), 0);
  assert_eq!(e.common_prefix_length(&vec![]), 0);

  // common_suffix_length
  assert_eq!(a.common_suffix_length(&vec![0, 1, 2, 3]), 3);
  assert_eq!(a.common_suffix_length(&vec![2, 3]), 2);
  assert_eq!(a.common_suffix_length(&vec![]), 0);
  assert_eq!(e.common_prefix_length(&vec![]), 0);

  // count_unique
  assert_eq!(b.count_unique(), 3);
  assert_eq!(e.count_unique(), 0);

  // equivalent
  assert!(b.equivalent(&vec![3, 2, 1, 2]));
  assert!(!b.equivalent(&vec![1, 3, 3]));
  assert!(!b.equivalent(&vec![1, 1, 2, 2, 3]));
  assert!(!b.equivalent(&vec![]));
  assert!(e.equivalent(&vec![]));

  // find_position
  assert_eq!(a.find_position(|&x| x == 2), Some((1, &2)));
  assert_eq!(a.find_position(|&x| x == 5), None);
  assert_eq!(e.find_position(|&x| x == 5), None);

  // frequencies
  assert_eq!(b.frequencies(), HashMap::from([
    (&1, 1),
    (&2, 2),
    (&3, 1),
  ]));
  assert_eq!(e.frequencies(), HashMap::new());

  // frequencies_by
  assert_eq!(b.frequencies_by(|x| x % 2), HashMap::from([
    (0, 2),
    (1, 2),
  ]));
  assert_eq!(e.frequencies(), HashMap::new());

  // joined
  assert_eq!(a.joined(", "), "1, 2, 3");
  assert_eq!(e.joined(", "), "");

  // position
  assert_eq!(b.position(|&x| x == 2), Some(1));
  assert_eq!(b.position(|&x| x == 5), None);
  assert_eq!(e.position(|&x| x == 5), None);

  // position_multi
  assert_eq!(b.position_multi(|&x| x % 2 == 0), vec![1, 2]);
  assert_eq!(b.position_multi(|&x| x > 3), vec![]);
  assert_eq!(e.position_multi(|&x| x > 3), vec![]);

  // position
  assert_eq!(b.position(|&x| x == 2), Some(1));
  assert_eq!(e.position(|&x| x == 0), None);

  // position_of
  assert_eq!(b.position_of(&2), Some(1));
  assert_eq!(b.position_of(&5), None);
  assert_eq!(e.position_of(&5), None);

  // position_of_multi
  assert_eq!(b.position_of_multi(&2), vec![1, 2]);
  assert_eq!(b.position_of_multi(&5), vec![]);
  assert_eq!(e.position_of_multi(&5), vec![]);

  // position_of_sequence
  assert_eq!(b.position_sequence(&vec![2, 2]), Some(1));
  assert_eq!(b.position_sequence(&vec![]), Some(0));
  assert_eq!(b.position_sequence(&vec![1, 3]), None);
  assert_eq!(e.position_sequence(&vec![1, 3]), None);

  // rfind
  assert_eq!(a.rfind(|&x| x % 2 == 1), Some(&3));
  assert_eq!(a.rfind(|&x| x == 5), None);
  assert_eq!(e.rfind(|&x| x == 5), None);

  // rfold
  assert_eq!(a.rfold(0, |acc, x| acc + x), 6);
  assert_eq!(e.rfold(0, |acc, x| acc + x), 0);

  // rposition
  assert_eq!(a.rposition(|&x| x % 2 == 1), Some(2));
  assert_eq!(a.rposition(|&x| x == 5), None);
  assert_eq!(e.rposition(|&x| x == 5), None);
}
