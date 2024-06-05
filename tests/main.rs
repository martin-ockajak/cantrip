#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::collections::{BinaryHeap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use crate::extensions::collections::*;

mod extensions;

#[test]
fn collectibles() {
  test_collectible_traits::<HashSet<i64>>();
  test_collectible_traits::<BTreeSet<i64>>();
  test_collectible_traits::<BinaryHeap<i64>>();
}

#[test]
fn slices() {
  // test_slice_traits::<&[i64]>();
}

#[test]
fn sequences() {
  test_sequence_traits::<LinkedList<i64>>();
  test_sequence_traits::<Vec<i64>>();
  test_sequence_traits::<VecDeque<i64>>();
}

#[quickcheck]
fn hash_map_string(data: HashMap<String, String>) -> bool {
  data.len();
  true
}
