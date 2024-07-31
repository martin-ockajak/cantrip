#![deny(warnings)]
#![allow(unused_crate_dependencies)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use crate::extensions::traits::*;

mod extensions;

#[test]
fn collectibles() {
  let a_hash_set = HashSet::from([1_i64, 2, 3]);
  let b_hash_set = HashSet::from([1_i64, 2, 2, 3]);
  let d_hash_set = HashSet::from([vec![1, 2], vec![3]]);
  let g_hash_set = HashSet::from([(1_i64, 1_i64), (2, 2), (3, 3)]);
  let e_hash_set = HashSet::<i64>::new();
  let a_btree_set = BTreeSet::from_iter(a_hash_set.clone());
  let b_btree_set = BTreeSet::from_iter(b_hash_set.clone());
  let d_btree_set = BTreeSet::from_iter(d_hash_set.clone());
  let g_btree_set = BTreeSet::from_iter(g_hash_set.clone());
  let e_btree_set = BTreeSet::from_iter(e_hash_set.clone());
  let a_binary_heap = BinaryHeap::from_iter(a_hash_set.clone());
  let b_binary_heap = BinaryHeap::from_iter(b_hash_set.clone());
  let d_binary_heap = BinaryHeap::from_iter(d_hash_set.clone());
  let g_binary_heap = BinaryHeap::from_iter(g_hash_set.clone());
  let e_binary_heap = BinaryHeap::from_iter(e_hash_set.clone());
  test_set_traits(&a_hash_set, &b_hash_set, &d_hash_set, &g_hash_set, &e_hash_set);
  test_set_traits(&a_btree_set, &b_btree_set, &d_btree_set, &g_btree_set, &e_btree_set);
  test_set_traits(&a_binary_heap, &b_binary_heap, &d_binary_heap, &g_binary_heap, &e_binary_heap);
}

#[test]
fn sequences() {
  let a_vec = vec![1_i64, 2, 3];
  let b_vec = vec![1_i64, 2, 2, 3];
  let c_vec = vec![3_i64, 2, 1];
  let d_vec = vec![vec![1_i64, 2], vec![3]];
  let g_vec = vec![(1_i64, 1), (2, 2), (3, 3)];
  let e_vec = Vec::<i64>::new();
  let a_vec_deque = VecDeque::from_iter(a_vec.clone());
  let b_vec_deque = VecDeque::from_iter(b_vec.clone());
  let c_vec_deque = VecDeque::from_iter(c_vec.clone());
  let d_vec_deque = VecDeque::from_iter(d_vec.clone());
  let g_vec_deque = VecDeque::from_iter(g_vec.clone());
  let e_vec_deque = VecDeque::from_iter(e_vec.clone());
  let a_linked_list = LinkedList::from_iter(a_vec.clone());
  let b_linked_list = LinkedList::from_iter(b_vec.clone());
  let c_linked_list = LinkedList::from_iter(c_vec.clone());
  let d_linked_list = LinkedList::from_iter(d_vec.clone());
  let g_linkeg_list = LinkedList::from_iter(g_vec.clone());
  let e_linked_list = LinkedList::from_iter(e_vec.clone());
  test_slice_traits(&a_vec[..], &b_vec[..], &e_vec[..]);
  test_sequence_traits(&a_vec, &b_vec, &c_vec, &d_vec, &g_vec, &e_vec);
  test_list_traits(&a_vec_deque, &b_vec_deque, &c_vec_deque, &d_vec_deque, &g_vec_deque, &e_vec_deque);
  test_list_traits(&a_linked_list, &b_linked_list, &c_linked_list, &d_linked_list, &g_linkeg_list, &e_linked_list);
}

#[test]
fn maps() {
  let a_hash_map = HashMap::from([(1_i64, 1_i64), (2, 2), (3, 3)]);
  let b_hash_map = HashMap::from([(1_i64, 1_i64), (2, 2), (3, 1)]);
  let e_hash_map = HashMap::<i64, i64>::new();
  let a_btree_map = BTreeMap::from_iter(a_hash_map.clone());
  let b_btree_map = BTreeMap::from_iter(b_hash_map.clone());
  let e_btree_map = BTreeMap::from_iter(e_hash_map.clone());
  test_map_traits(&a_hash_map, &b_hash_map, &e_hash_map);
  test_map_traits(&a_btree_map, &b_btree_map, &e_btree_map);
}
