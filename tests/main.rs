use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

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
  // FIXME - fix slice tests
  // test_slice_traits::<&[i64]>();
  let v: Vec<i32> = Vec::new();
  let d: VecDeque<i32> = VecDeque::new();
  let l: LinkedList<i32> = LinkedList::new();
  let s: HashSet<i32> = HashSet::new();
  let m: HashMap<i32, i32> = HashMap::new();
  let _ = (v.len(), l.len(), d.len(), s.len(), m.len());
}

#[test]
fn sequences() {
  test_sequence_traits::<LinkedList<i64>>();
  test_sequence_traits::<Vec<i64>>();
  test_sequence_traits::<VecDeque<i64>>();
}

#[test]
fn maps() {
  test_map_traits::<HashMap<i64, i64>>();
  test_map_traits::<BTreeMap<i64, i64>>();
}
