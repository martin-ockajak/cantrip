use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use crate::extensions::collections::*;

mod extensions;

#[test]
fn collectibles() {
  test_set_traits::<HashSet<i64>>();
  test_set_traits::<BTreeSet<i64>>();
  test_set_traits::<BinaryHeap<i64>>();
}

#[test]
fn sequences() {
  // FIXME - fix slice tests
  // test_slice_traits::<&[i64]>();
  let vec: Vec<i64> = Vec::new();
  let vec_deque: VecDeque<i64> = VecDeque::new();
  let linked_list: LinkedList<i64> = LinkedList::new();
  let slice = &vec[..];
  let _ = (slice.len(), vec.len(), linked_list.len(), vec_deque.len());
  test_sequence_traits(vec);
  test_list_traits(linked_list);
  test_list_traits(vec_deque);
}

#[test]
fn maps() {
  test_map_traits::<HashMap<i64, i64>>();
  test_map_traits::<BTreeMap<i64, i64>>();
}
