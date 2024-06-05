use std::fmt::Debug;

use cantrip::Map;

use crate::extensions::util::{assert_map_equal, Equal, from_map_slice};

pub fn test_map<'a, C>()
where
  C: Map<i64, i64> + FromIterator<(i64, i64)> + Iterator<Item = (i64, i64)> + Clone + Equal + Debug + 'a,
{
  let distinct = from_map_slice::<C>(&[(0, 0), (1, 1), (2, 2)]);
  let empty = from_map_slice::<C>(&[]);

  // add
  assert_map_equal(distinct.clone().add(3, 3), &[(0, 0), (1, 1), (2, 2), (3, 3)]);
  assert_map_equal(empty.clone().add(0, 0), &[(0, 0)]);
}
