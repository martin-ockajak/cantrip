#![deny(warnings)]
#![allow(unused_crate_dependencies)]
//! Practical extension methods for Rust standard library collections.
//!
//! Enables direct functional-style collection manipulation without the usual iterator boilerplate.
//!
//!
//! ## Features
//!
//! - Equivalents of standard iterator methods are added to standard library collections
//! - Additional utility methods commonly found in collection libraries are also included
//! - Transformation methods return a new collection instance instead of returning an iterator
//! - All methods treat collection instances as immutable although some may consume them
//! - Performance is near optimal and overhead is limited to new collection creation
//!
//!
//! ## Examples
//!
//! ```rust
//! use cantrip::*;
//!
//! # let source = vec![1, 2, 3];
//! let a = vec![1, 2, 3];
//!
//! a.fold_to(0, |r, x| r + x);             // 6
//!
//! # let a = source.clone();
//! a.map_to(|x| (x, x + 1)).to_map();      // HashMap::from([(1, 2), (2, 3), (3, 4)])
//!
//! # let a = source.clone();
//! a.flat_map(|&x| [x, -x]).sorted();      // vec![-3, -2, -1, 1, 2, 3]
//!
//! # let a = source.clone();
//! a.filter(|&x| x > 1).into_set();        // HashSet::from([2, 3])
//!
//! # let a = source.clone();
//! a.group_by(|x| x % 2);                  // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
//!
//! # let a = source.clone();
//! a.delete(&1).add(2).unique();           // vec![2, 3]
//!
//! # let a = source.clone();
//! a.substitute_at(0, 4).to_list();        // LinkedList::from([4, 2, 3])
//!
//! # let a = source.clone();
//! a.position_multi(|&x| x % 2 == 1);      // vec![0, 2]
//!
//! # let a = source.clone();
//! a.rev().into_iter().into_deque();       // VecDeque::from([3, 2, 1])
//! ```
//!
//!
//! ## Methods
//!
//! | Method / Collection type                                        | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-----------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`add`](CollectionTo::add)                                      |            *              |       |              *                |         *         |     Y     |
//! | [`add_at`](SequenceTo::add_at)                                  |            *              |       |                               |                   |     Y     |
//! | [`add_at_multi`](SequenceTo::add_at_multi)                      |            *              |       |                               |                   |     Y     |
//! | [`add_multi`](CollectionTo::add_multi)                          |            *              |       |              *                |         *         |     Y     |
//! | [`all`](Collection::all)                                        |            *              |   *   |              *                |         *         |     N     |
//! | [`any`](Collection::any)                                        |            *              |   *   |              *                |         *         |     N     |
//! | [`cartesian_product`](SequenceTo::cartesian_product)            |            *              |       |                               |                   |     N     |
//! | [`chunked`](SequenceTo::chunked)                                |            *              |       |                               |                   |     Y     |
//! | [`chunked_by`](SequenceTo::chunked_by)                          |            *              |       |                               |                   |     Y     |
//! | [`chunked_exact`](SequenceTo::chunked_exact)                    |            *              |       |                               |                   |     Y     |
//! | [`combinations`](CollectionTo::combinations)                    |            *              |       |              *                |                   |     N     |
//! | [`combinations_multi`](SequenceTo::combinations_multi)          |            *              |       |                               |                   |     N     |
//! | [`coalesce`](SequenceTo::coalesce)                              |            *              |       |                               |                   |     Y     |
//! | [`collect`](Transform::collect)                                 |            *              |   *   |              *                |         *         |     N     |
//! | [`collect_to`](CollectionTo::collect_to)                        |            *              |   *   |              *                |         *         |     Y     |
//! | [`common_prefix_length`](Sequence::common_prefix_length)        |            *              |   *   |                               |                   |     N     |
//! | [`common_suffix_length`](Sequence::common_suffix_length)        |            *              |   *   |                               |                   |     N     |
//! | [`count_by`](Collection::count_by)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`count_unique`](Sequence::count_unique)                        |            *              |   *   |                               |         *         |     N     |
//! | [`delete`](CollectionTo::delete)                                |            *              |       |              *                |         *         |     Y     |
//! | [`delete_at`](SequenceTo::delete_at)                            |            *              |       |                               |                   |     Y     |
//! | [`delete_at_multi`](SequenceTo::delete_at_multi)                |            *              |       |                               |                   |     Y     |
//! | [`delete_multi`](CollectionTo::delete_multi)                    |            *              |       |              *                |         *         |     Y     |
//! | [`divide`](SequenceTo::divide)                                  |            *              |       |                               |                   |     Y     |
//! | [`divide_by`](SequenceTo::divide_by)                            |            *              |       |                               |                   |     Y     |
//! | [`disjoint`](Collection::disjoint)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`duplicates`](SequenceTo::duplicates)                          |            *              |       |                               |                   |     Y     |
//! | [`duplicates_by`](SequenceTo::duplicates_by)                    |            *              |       |                               |                   |     Y     |
//! | [`enumerate`](SequenceTo::enumerate)                            |            *              |       |                               |                   |     Y     |
//! | [`equivalent`](Sequence::equivalent)                            |            *              |   *   |                               |                   |     N     |
//! | [`fill`](SequenceTo::fill)                                      |            *              |       |                               |                   |     Y     |
//! | [`fill_with`](CollectionTo::fill_with)                          |            *              |       |              *                |         *         |     Y     |
//! | [`filter`](CollectionTo::filter)                                |            *              |       |              *                |         *         |     Y     |
//! | [`filter_keys`](Map::filter_keys)                               |                           |       |                               |         *         |     Y     |
//! | [`filter_map`](CollectionTo::filter_map)                        |            *              |       |              *                |         *         |     N     |
//! | [`filter_map_to`](CollectionTo::filter_map_to)                  |            *              |       |              *                |         *         |     Y     |
//! | [`filter_values`](Map::filter_values)                           |                           |       |                               |         *         |     Y     |
//! | [`find`](Collection::find)                                      |            *              |   *   |              *                |         *         |     N     |
//! | [`find_map`](Collection::find_map)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`find_map_to`](CollectionTo::find_map_to)                      |            *              |       |              *                |         *         |     Y     |
//! | [`find_position`](Sequence::find_position)                      |            *              |   *   |                               |                   |     N     |
//! | [`first`](List::first)                                          |            *              |   *   |                               |                   |     N     |
//! | [`flat_map`](CollectionTo::flat_map)                            |            *              |       |              *                |         *         |     N     |
//! | [`flat_map_to`](CollectionTo::flat_map_to)                      |            *              |       |              *                |         *         |     Y     |
//! | [`flat`](CollectionTo::flat)                                    |            *              |       |              *                |                   |     Y     |
//! | [`fold`](Collection::fold)                                      |            *              |   *   |              *                |         *         |     N     |
//! | [`fold_to`](CollectionTo::fold_to)                              |            *              |       |              *                |         *         |     Y     |
//! | [`for_each`](Collection::for_each)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`frequencies`](Sequence::frequencies)                          |            *              |   *   |                               |                   |     N     |
//! | [`frequencies_by`](Sequence::frequencies_by)                    |            *              |   *   |                               |                   |     N     |
//! | [`group_by`](CollectionTo::group_by)                            |            *              |       |              *                |                   |     Y     |
//! | [`group_fold`](Collection::group_fold)                          |            *              |   *   |              *                |                   |     N     |
//! | [`group_fold_to`](CollectionTo::group_fold_to)                  |            *              |       |              *                |                   |     Y     |
//! | [`group_reduce`](Collection::group_reduce)                      |            *              |   *   |              *                |                   |     N     |
//! | [`group_reduce_to`](CollectionTo::group_reduce_to)              |            *              |       |              *                |                   |     Y     |
//! | [`init`](SequenceTo::init)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`interleave`](SequenceTo::interleave)                          |            *              |       |                               |                   |     Y     |
//! | [`interleave_exact`](SequenceTo::interleave_exact)              |            *              |       |                               |                   |     Y     |
//! | [`intersect`](CollectionTo::intersect)                          |            *              |       |              *                |         *         |     Y     |
//! | [`intersperse`](SequenceTo::intersperse)                        |            *              |       |                               |                   |     Y     |
//! | [`intersperse_with`](SequenceTo::intersperse_with)              |            *              |       |                               |                   |     Y     |
//! | [`into_bmap`](TransformTo::into_bmap)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_bset`](TransformTo::into_bset)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_heap`](TransformTo::into_heap)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_list`](TransformTo::into_list)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_map`](TransformTo::into_map)                             |            *              |       |              *                |         *         |     Y     |
//! | [`into_set`](TransformTo::into_set)                             |            *              |       |              *                |         *         |     Y     |
//! | [`into_vec`](TransformVec::into_vec)                            |            *              |   *   |              *                |         *         |     Y     |
//! | [`into_deque`](TransformTo::into_deque)                         |            *              |       |              *                |         *         |     Y     |
//! | [`joined`](Sequence::joined)                                    |            *              |       |                               |                   |     N     |
//! | [`largest`](CollectionTo::largest)                              |            *              |       |              *                |                   |     Y     |
//! | [`last`](List::last)                                            |            *              |       |                               |                   |     N     |
//! | [`map`](CollectionTo::map)                                      |            *              |       |              *                |         *         |     N     |
//! | [`map_to`](CollectionTo::map_to)                                |            *              |       |              *                |         *         |     Y     |
//! | [`map_keys`](Map::map_keys)                                     |                           |       |                               |         *         |     Y     |
//! | [`map_values`](Map::map_values)                                 |                           |       |                               |         *         |     Y     |
//! | [`map_while`](SequenceTo::map_while)                            |            *              |       |                               |                   |     N     |
//! | [`max_by`](Collection::max_by)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`max_by_key`](Collection::max_by_key)                          |            *              |   *   |              *                |         *         |     N     |
//! | [`max_of`](Collection::max_of)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`merge`](SequenceTo::merge)                                    |            *              |       |                               |                   |     Y     |
//! | [`merge_by`](SequenceTo::merge_by)                              |            *              |       |                               |                   |     Y     |
//! | [`min_by`](Collection::min_by)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`min_by_key`](Collection::min_by_key)                          |            *              |   *   |              *                |         *         |     N     |
//! | [`min_of`](Collection::min_of)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`minmax_by`](Collection::minmax_by)                            |            *              |   *   |              *                |         *         |     N     |
//! | [`minmax_by_key`](Collection::minmax_by_key)                    |            *              |   *   |              *                |         *         |     N     |
//! | [`minmax_of`](Collection::minmax_of)                            |            *              |   *   |              *                |         *         |     N     |
//! | [`move_at`](SequenceTo::move_at)                                |            *              |       |                               |                   |     Y     |
//! | [`pad_left`](SequenceTo::pad_left)                              |            *              |       |                               |                   |     Y     |
//! | [`pad_left_with`](SequenceTo::pad_left_with)                    |            *              |       |                               |                   |     Y     |
//! | [`pad_right`](SequenceTo::pad_right)                            |            *              |       |                               |                   |     Y     |
//! | [`pad_right_with`](SequenceTo::pad_right_with)                  |            *              |       |                               |                   |     Y     |
//! | [`partition`](CollectionTo::partition)                          |            *              |       |              *                |         *         |     Y     |
//! | [`partition_map`](CollectionTo::partition_map)                  |            *              |       |              *                |         *         |     N     |
//! | [`partition_map_to`](CollectionTo::partition_map_to)            |            *              |       |              *                |         *         |     Y     |
//! | [`position`](Sequence::position)                                |            *              |   *   |                               |                   |     N     |
//! | [`position_multi`](Sequence::position_multi)                    |            *              |   *   |                               |                   |     N     |
//! | [`position_of`](Sequence::position_of)                          |            *              |   *   |                               |                   |     N     |
//! | [`position_of_multi`](Sequence::position_of_multi)              |            *              |   *   |                               |                   |     N     |
//! | [`position_sequence`](Sequence::position_sequence)              |            *              |   *   |                               |                   |     N     |
//! | [`powerset`](CollectionTo::powerset)                            |            *              |       |              *                |                   |     N     |
//! | [`product`](CollectionTo::product)                              |            *              |       |              *                |                   |     Y     |
//! | [`product_keys`](Map::product_keys)                             |                           |       |                               |         *         |     Y     |
//! | [`product_values`](Map::product_values)                         |                           |       |                               |         *         |     Y     |
//! | [`reduce`](Collection::reduce)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`reduce_to`](CollectionTo::reduce_to)                          |            *              |       |              *                |         *         |     Y     |
//! | [`repeat`](List::repeat)                                        |            *              |       |                               |                   |           |
//! | [`rev`](SequenceTo::rev)                                        |            *              |       |                               |                   |     Y     |
//! | [`rfind`](Sequence::rfind)                                      |            *              |   *   |                               |                   |     N     |
//! | [`rfold`](Sequence::rfold)                                      |            *              |   *   |                               |                   |     N     |
//! | [`rfold_to`](SequenceTo::rfold_to)                              |            *              |       |                               |                   |     Y     |
//! | [`rposition`](Sequence::rposition)                              |            *              |   *   |                               |                   |     N     |
//! | [`scan`](SequenceTo::scan)                                      |            *              |       |                               |                   |     N     |
//! | [`scan_to`](SequenceTo::scan_to)                                |            *              |       |                               |                   |     Y     |
//! | [`skip`](SequenceTo::skip)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`skip_while`](SequenceTo::skip_while)                          |            *              |   *   |                               |                   |     Y     |
//! | [`slice`](SequenceTo::slice)                                    |            *              |       |                               |                   |     Y     |
//! | [`smallest`](CollectionTo::smallest)                            |            *              |       |              *                |                   |     Y     |
//! | [`sorted`](SequenceTo::sorted)                                  |            *              |       |                               |                   |     Y     |
//! | [`sorted_by`](SequenceTo::sorted_by)                            |            *              |       |                               |                   |     Y     |
//! | [`sorted_by_cached_key`](SequenceTo::sorted_by_cached_key)      |            *              |       |                               |                   |     Y     |
//! | [`sorted_by_key`](SequenceTo::sorted_by_key)                    |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable`](SequenceTo::sorted_unstable)                |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable_by`](SequenceTo::sorted_unstable_by)          |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable_by_key`](SequenceTo::sorted_unstable_by_key)  |            *              |       |                               |                   |     Y     |
//! | [`step_by`](SequenceTo::step_by)                                |            *              |       |                               |                   |     Y     |
//! | [`subset`](Collection::subset)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`substitute`](CollectionTo::substitute)                        |            *              |       |              *                |         *         |     Y     |
//! | [`substitute_at`](SequenceTo::substitute_at)                    |            *              |       |                               |                   |     Y     |
//! | [`substitute_at_multi`](SequenceTo::substitute_at_multi)        |            *              |       |                               |                   |     Y     |
//! | [`substitute_multi`](CollectionTo::substitute_multi)            |            *              |       |              *                |         *         |     Y     |
//! | [`superset`](Collection::superset)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`sum`](CollectionTo::sum)                                      |            *              |       |              *                |                   |     Y     |
//! | [`sum_keys`](Map::sum_keys)                                     |                           |       |                               |         *         |     Y     |
//! | [`sum_values`](Map::sum_values)                                 |                           |       |                               |         *         |     Y     |
//! | [`swap_at`](SequenceTo::swap_at)                                |            *              |       |                               |                   |     Y     |
//! | [`tail`](SequenceTo::tail)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`take`](SequenceTo::take)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`take_while`](SequenceTo::take_while)                          |            *              |   *   |                               |                   |     Y     |
//! | [`unique`](SequenceTo::unique)                                  |            *              |       |                               |                   |     Y     |
//! | [`unique_by`](SequenceTo::unique_by)                            |            *              |       |                               |                   |     Y     |
//! | [`unit`](CollectionTo::unit)                                    |            *              |       |              *                |         *         |     Y     |
//! | [`unzip`](SequenceTo::unzip)                                    |            *              |       |                               |                   |     Y     |
//! | [`to_bmap`](Transform::to_bmap)                                 |            *              |   *   |              *                |         *         |     N     |
//! | [`to_bset`](Transform::to_bset)                                 |            *              |   *   |              *                |         *         |     N     |
//! | [`to_heap`](Transform::to_heap)                                 |            *              |   *   |              *                |         *         |     N     |
//! | [`to_keys`](Map::to_keys)                                       |                           |       |                               |         *         |     N     |
//! | [`to_list`](Transform::to_list)                                 |            *              |   *   |              *                |         *         |     N     |
//! | [`to_map`](Transform::to_map)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`to_set`](Transform::to_set)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`to_values`](Map::to_values)                                   |                           |       |                               |         *         |     N     |
//! | [`to_vec`](TransformVec::to_vec)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`to_deque`](Transform::to_deque)                               |            *              |   *   |              *                |         *         |     N     |
//! | [`variations`](SequenceTo::variations)                          |            *              |       |                               |                   |     N     |
//! | [`windowed`](SequenceTo::windowed)                              |            *              |       |                               |                   |     N     |
//! | [`windowed_circular`](SequenceTo::windowed_circular)            |            *              |       |                               |                   |     N     |
//! | [`zip`](SequenceTo::zip)                                        |            *              |       |                               |                   |     Y     |
//! | [`zip_padded`](SequenceTo::zip_padded)                          |            *              |       |                               |                   |     Y     |
pub(crate) mod collections;
pub(crate) mod core;
pub(crate) mod extensions;

pub use core::iterable::*;
pub use extensions::*;
