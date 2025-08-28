use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use cantrip::Convert;

use crate::extensions::util::{TestCollection, assert_set_equal};

pub(crate) fn test_convert<'a, C, G>(sequence: bool, a_source: &C, g_source: &G, e_source: &C)
where
  C: Convert<i64> + TestCollection<i64> + IntoIterator<Item = i64> + 'a,
  G: Convert<(i64, i64)> + TestCollection<(i64, i64)> + IntoIterator<Item = (i64, i64)> + 'a, {
  // to_bmap
  let g = g_source.clone();
  let e = g_source.clone().into_iter().filter(|_| false).collect::<G>();
  assert_eq!(g.to_bmap(), BTreeMap::from([(1, 1), (2, 2), (3, 3)]));
  assert_eq!(e.to_bmap(), BTreeMap::new());

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
  assert_eq!(a.to_heap().into_iter().collect::<HashSet<_>>(), BinaryHeap::from([1, 2, 3]).into_iter().collect());
  assert_eq!(e.to_heap().into_iter().collect::<HashSet<_>>(), BinaryHeap::from([]).into_iter().collect());

  // to_list
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.to_list(), LinkedList::from([1, 2, 3]));
  } else {
    assert_set_equal(a.to_list(), vec![1, 2, 3]);
  };
  assert_eq!(e.to_list(), LinkedList::new());

  // to_map
  let g = g_source.clone();
  let e = g_source.clone().into_iter().filter(|_| false).collect::<G>();
  assert_eq!(g.to_map(), HashMap::from([(1, 1), (2, 2), (3, 3)]));
  assert_eq!(e.to_map(), HashMap::new());

  // to_set
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.to_set(), HashSet::from([1, 2, 3]));
  assert_eq!(e.to_set(), HashSet::new());

  // to_vec
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.to_vec(), vec![1, 2, 3]);
  } else {
    assert_set_equal(a.to_vec(), vec![1, 2, 3]);
  }
  assert_eq!(e.to_vec(), vec![]);
}
