use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use cantrip::TransformTo;

use crate::extensions::util::{assert_set_equal, TestCollectible, TestCollection};

pub(crate) fn test_transform_to<'a, C, G>(sequence: bool, a_source: &C, g_source: &G, e_source: &C)
where
  C: TransformTo<i64> + TestCollection<i64> + IntoIterator<Item = i64> + 'a,
  G: TestCollectible<'a, (i64, i64)>,
{
  // into_bmap
  let g = g_source.clone();
  let e = g_source.clone().filter(|_| false);
  assert_eq!(g.into_bmap(), BTreeMap::from([(1, 1), (2, 2), (3, 3)]));
  assert_eq!(e.into_bmap(), BTreeMap::new());

  // into_bset
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.into_bset(), BTreeSet::from([1, 2, 3]));
  assert_eq!(e.into_bset(), BTreeSet::new());

  // into_deque
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.into_deque(), VecDeque::from([1, 2, 3]));
  } else {
    assert_set_equal(a.into_deque(), vec![1, 2, 3]);
  };
  assert_eq!(e.into_deque(), VecDeque::new());

  // into_heap
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.into_heap().into_iter().collect::<HashSet<_>>(), BinaryHeap::from([1, 2, 3]).into_iter().collect());
  assert_eq!(e.into_heap().into_iter().collect::<HashSet<_>>(), BinaryHeap::from([]).into_iter().collect());

  // into_list
  let a = a_source.clone();
  let e = e_source.clone();
  if sequence {
    assert_eq!(a.into_list(), LinkedList::from([1, 2, 3]));
  } else {
    assert_set_equal(a.into_list(), vec![1, 2, 3]);
  };
  assert_eq!(e.into_list(), LinkedList::new());

  // into_map
  let g = g_source.clone();
  let e = g_source.clone().filter(|_| false);
  assert_eq!(g.into_map(), HashMap::from([(1, 1), (2, 2), (3, 3)]));
  assert_eq!(e.into_map(), HashMap::new());

  // into_set
  let a = a_source.clone();
  let e = e_source.clone();
  assert_eq!(a.into_set(), HashSet::from([1, 2, 3]));
  assert_eq!(e.into_set(), HashSet::new());
}
