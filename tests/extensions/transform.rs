use std::collections::{BinaryHeap, BTreeSet, HashSet, LinkedList, VecDeque};
use cantrip::Transform;

use crate::extensions::util::{assert_set_equal, TestCollection};

pub(crate) fn test_transform<'a, C>(sequence: bool, a_source: &C, e_source: &C)
where
  C: Transform<i64> + TestCollection<i64> + IntoIterator<Item = i64> + 'a,
{
  // collect
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.collect::<LinkedList<i64>>(), LinkedList::from([1, 2, 3]));
  } else {
    assert_eq!(a.collect::<HashSet<i64>>(), HashSet::from([1, 2, 3]));
  }
  assert_eq!(e.collect::<LinkedList<i64>>(), LinkedList::new());

  // to_bset
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.to_bset(), BTreeSet::from([1, 2, 3]));
  assert_eq!(e.to_bset(), BTreeSet::new());

  // to_deque
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.to_deque(), VecDeque::from([1, 2, 3]));
  } else {
    assert_set_equal(a.to_deque(), vec![1, 2, 3]);
  };
  assert_eq!(e.to_deque(), VecDeque::new());

  // to_heap
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.to_heap().into_iter().collect::<HashSet<_>>(), BinaryHeap::from([1, 2, 3]).to_set());
  assert_eq!(e.to_heap().into_iter().collect::<HashSet<_>>(), BinaryHeap::from([]).to_set());

  // to_list
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.to_list(), LinkedList::from([1, 2, 3]));
  } else {
    assert_set_equal(a.to_list(), vec![1, 2, 3]);
  };
  assert_eq!(e.to_list(), LinkedList::new());

  // to_set
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.to_set(), HashSet::from([1, 2, 3]));
  assert_eq!(e.to_set(), HashSet::new());
}
