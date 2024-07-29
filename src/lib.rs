#![deny(warnings)]
#![allow(unused_crate_dependencies)]
//! Practical extension methods for Rust standard library collections.
//!
//! Enables direct functional-style collection manipulation without the usual iterator boilerplate.
//!
//!
//! ## Overview
//!
//! - Equivalents of standard iterator methods are added to standard library collections
//! - Additional utility methods commonly found in collection libraries are also included
//! - All methods treat collection instances as immutable although some may consume them
//! - Methods which modify a collection return a new collection instead of an iterator
//! - Performance is near optimal and overhead is limited to new collection creation
//!
//!
//! ## Functionality
//!
//! - [Searching](#searching) - [Modifying](#modifying) - [Filtering](#filtering) -
//!   [Mapping](#mapping) - [Inspecting](#inspecting) - [Aggregating](#aggregating)
//!
//! - [Selecting](#selecting) - [Converting](#converting) - [Partitioning](#partitioning) -
//!   [Merging](#merging) - [Sorting](#sorting) - [Miscellaneous](#miscellaneous)
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
//! a.fold(0, |r, x| r + x);                    // 6
//!
//! # let a = source.clone();
//! a.map_ref(|&x| (x, x + 1)).to_map();        // HashMap::from([(1, 2), (2, 3), (3, 4)])
//!
//! # let a = source.clone();
//! a.flat_map(|x| [x, -x]).sorted();           // vec![-3, -2, -1, 1, 2, 3]
//!
//! # let a = source.clone();
//! a.filter(|&x| x > 1).into_set();            // HashSet::from([2, 3])
//!
//! # let a = source.clone();
//! a.group_by(|x| x % 2);                      // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
//!
//! # let a = source.clone();
//! a.delete(&1).add(2).unique();               // vec![2, 3]
//!
//! # let a = source.clone();
//! a.substitute_at(0, 4).to_list();            // LinkedList::from([4, 2, 3])
//!
//! # let a = source.clone();
//! a.position_multi(|&x| x % 2 == 1);          // vec![0, 2]
//!
//! # let a = source.clone();
//! a.rev().into_iter().into_deque();           // VecDeque::from([3, 2, 1])
//! ```
//!
//!
//! ## Methods
//!
//! ### Searching
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`find`](Collection::find)                                        |            *              |   *   |              *                |         *         |     N     |
//! | [`find_map`](CollectionInto::find_map)                            |            *              |       |              *                |         *         |     Y     |
//! | [`find_map_ref`](Collection::find_map_ref)                        |            *              |   *   |              *                |         *         |     N     |
//! | [`find_position`](Sequence::find_position)                        |            *              |   *   |                               |                   |     N     |
//! | [`first`](List::first)                                            |            *              |   *   |                               |                   |     N     |
//! | [`last`](List::last)                                              |            *              |       |                               |                   |     N     |
//! | [`max_by`](Collection::max_by)                                    |            *              |   *   |              *                |         *         |     N     |
//! | [`max_by_key`](Collection::max_by_key)                            |            *              |   *   |              *                |         *         |     N     |
//! | [`max_of`](Collection::max_of)                                    |            *              |   *   |              *                |         *         |     N     |
//! | [`min_by`](Collection::min_by)                                    |            *              |   *   |              *                |         *         |     N     |
//! | [`min_by_key`](Collection::min_by_key)                            |            *              |   *   |              *                |         *         |     N     |
//! | [`min_of`](Collection::min_of)                                    |            *              |   *   |              *                |         *         |     N     |
//! | [`minmax_by`](Collection::minmax_by)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`minmax_by_key`](Collection::minmax_by_key)                      |            *              |   *   |              *                |         *         |     N     |
//! | [`minmax_of`](Collection::minmax_of)                              |            *              |   *   |              *                |         *         |     N     |
//! | [`position`](Sequence::position)                                  |            *              |   *   |                               |                   |     N     |
//! | [`position_multi`](Sequence::position_multi)                      |            *              |   *   |                               |                   |     N     |
//! | [`position_of`](Sequence::position_of)                            |            *              |   *   |                               |                   |     N     |
//! | [`position_of_multi`](Sequence::position_of_multi)                |            *              |   *   |                               |                   |     N     |
//! | [`position_sequence`](Sequence::position_sequence)                |            *              |   *   |                               |                   |     N     |
//! | [`rfind`](Sequence::rfind)                                        |            *              |   *   |                               |                   |     N     |
//! | [`rposition`](Sequence::rposition)                                |            *              |   *   |                               |                   |     N     |
//!
//! ### Modifying
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`add`](CollectionInto::add)                                      |            *              |       |              *                |         *         |     Y     |
//! | [`add_at`](SequenceInto::add_at)                                  |            *              |       |                               |                   |     Y     |
//! | [`add_at_multi`](SequenceInto::add_at_multi)                      |            *              |       |                               |                   |     Y     |
//! | [`add_multi`](CollectionInto::add_multi)                          |            *              |       |              *                |         *         |     Y     |
//! | [`delete`](CollectionInto::delete)                                |            *              |       |              *                |         *         |     Y     |
//! | [`delete_at`](SequenceInto::delete_at)                            |            *              |       |                               |                   |     Y     |
//! | [`delete_at_multi`](SequenceInto::delete_at_multi)                |            *              |       |                               |                   |     Y     |
//! | [`delete_multi`](CollectionInto::delete_multi)                    |            *              |       |              *                |         *         |     Y     |
//! | [`move_at`](SequenceInto::move_at)                                |            *              |       |                               |                   |     Y     |
//! | [`pad_left`](SequenceInto::pad_left)                              |            *              |       |                               |                   |     Y     |
//! | [`pad_left_with`](SequenceInto::pad_left_with)                    |            *              |       |                               |                   |     Y     |
//! | [`pad_right`](SequenceInto::pad_right)                            |            *              |       |                               |                   |     Y     |
//! | [`pad_right_with`](SequenceInto::pad_right_with)                  |            *              |       |                               |                   |     Y     |
//! | [`rev`](SequenceInto::rev)                                        |            *              |       |                               |                   |     Y     |
//! | [`substitute`](CollectionInto::substitute)                        |            *              |       |              *                |         *         |     Y     |
//! | [`substitute_at`](SequenceInto::substitute_at)                    |            *              |       |                               |                   |     Y     |
//! | [`substitute_at_multi`](SequenceInto::substitute_at_multi)        |            *              |       |                               |                   |     Y     |
//! | [`substitute_multi`](CollectionInto::substitute_multi)            |            *              |       |              *                |         *         |     Y     |
//! | [`swap_at`](SequenceInto::swap_at)                                |            *              |       |                               |                   |     Y     |
//!
//! ### Filtering
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`duplicates`](SequenceInto::duplicates)                          |            *              |       |                               |                   |     Y     |
//! | [`duplicates_by`](SequenceInto::duplicates_by)                    |            *              |       |                               |                   |     Y     |
//! | [`filter`](CollectionInto::filter)                                |            *              |       |              *                |         *         |     Y     |
//! | [`filter_keys`](Map::filter_keys)                                 |                           |       |                               |         *         |     Y     |
//! | [`filter_map`](CollectionInto::filter_map)                        |            *              |       |              *                |         *         |     Y     |
//! | [`filter_map_ref`](CollectionInto::filter_map_ref)                |            *              |       |              *                |         *         |     N     |
//! | [`filter_ref`](CollectionInto::filter_ref)                        |            *              |       |              *                |         *         |     N     |
//! | [`filter_values`](Map::filter_values)                             |                           |       |                               |         *         |     Y     |
//! | [`init`](SequenceInto::init)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`largest`](CollectionInto::largest)                              |            *              |       |              *                |                   |     Y     |
//! | [`slice`](SequenceInto::slice)                                    |            *              |       |                               |                   |     Y     |
//! | [`smallest`](CollectionInto::smallest)                            |            *              |       |              *                |                   |     Y     |
//! | [`skip`](SequenceInto::skip)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`skip_while`](SequenceInto::skip_while)                          |            *              |   *   |                               |                   |     Y     |
//! | [`step_by`](SequenceInto::step_by)                                |            *              |       |                               |                   |     Y     |
//! | [`take`](SequenceInto::take)                                      |            *              |   *   |                               |                   |     Y     |
//! | [`take_while`](SequenceInto::take_while)                          |            *              |   *   |                               |                   |     Y     |
//! | [`unique`](SequenceInto::unique)                                  |            *              |       |                               |                   |     Y     |
//! | [`unique_by`](SequenceInto::unique_by)                            |            *              |       |                               |                   |     Y     |
//! | [`tail`](SequenceInto::tail)                                      |            *              |   *   |                               |                   |     Y     |
//!
//! ### Mapping
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`coalesce`](SequenceInto::coalesce)                              |            *              |       |                               |                   |     Y     |
//! | [`enumerate`](SequenceInto::enumerate)                            |            *              |       |                               |                   |     Y     |
//! | [`flat_map`](CollectionInto::flat_map)                            |            *              |       |              *                |         *         |     Y     |
//! | [`flat_map_ref`](CollectionInto::flat_map_ref)                    |            *              |       |              *                |         *         |     N     |
//! | [`map`](CollectionInto::map)                                      |            *              |       |              *                |         *         |     Y     |
//! | [`map_ref`](CollectionInto::map_ref)                              |            *              |       |              *                |         *         |     N     |
//! | [`map_keys`](Map::map_keys)                                       |                           |       |                               |         *         |     Y     |
//! | [`map_values`](Map::map_values)                                   |                           |       |                               |         *         |     Y     |
//! | [`map_while`](SequenceInto::map_while)                            |            *              |       |                               |                   |     N     |
//! | [`scan`](SequenceInto::scan)                                      |            *              |       |                               |                   |     Y     |
//! | [`scan_ref`](SequenceInto::scan_ref)                              |            *              |       |                               |                   |     N     |
//!
//! ### Inspecting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`all`](Collection::all)                                          |            *              |   *   |              *                |         *         |     N     |
//! | [`any`](Collection::any)                                          |            *              |   *   |              *                |         *         |     N     |
//! | [`common_prefix_length`](Sequence::common_prefix_length)          |            *              |   *   |                               |                   |     N     |
//! | [`common_suffix_length`](Sequence::common_suffix_length)          |            *              |   *   |                               |                   |     N     |
//! | [`count_by`](Collection::count_by)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`count_unique`](Sequence::count_unique)                          |            *              |   *   |                               |         *         |     N     |
//! | [`disjoint`](Collection::disjoint)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`equivalent`](Sequence::equivalent)                              |            *              |   *   |                               |                   |     N     |
//! | [`frequencies`](Sequence::frequencies)                            |            *              |   *   |                               |                   |     N     |
//! | [`frequencies_by`](Sequence::frequencies_by)                      |            *              |   *   |                               |                   |     N     |
//! | [`intersect`](CollectionInto::intersect)                          |            *              |       |              *                |         *         |     Y     |
//! | [`subset`](Collection::subset)                                    |            *              |   *   |              *                |         *         |     N     |
//! | [`superset`](Collection::superset)                                |            *              |   *   |              *                |         *         |     N     |
//!
//! ### Aggregating
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`fold`](CollectionInto::fold)                                    |            *              |       |              *                |         *         |     Y     |
//! | [`fold_ref`](Collection::fold_ref)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`group_fold`](CollectionInto::group_fold)                        |            *              |       |              *                |                   |     Y     |
//! | [`group_fold_ref`](Collection::group_fold_ref)                    |            *              |   *   |              *                |                   |     N     |
//! | [`group_reduce`](CollectionInto::group_reduce)                    |            *              |       |              *                |                   |     Y     |
//! | [`group_reduce_ref`](Collection::group_reduce_ref)                |            *              |   *   |              *                |                   |     N     |
//! | [`product`](CollectionInto::product)                              |            *              |       |              *                |                   |     Y     |
//! | [`product_keys`](Map::product_keys)                               |                           |       |                               |         *         |     Y     |
//! | [`product_values`](Map::product_values)                           |                           |       |                               |         *         |     Y     |
//! | [`reduce`](CollectionInto::reduce)                                |            *              |       |              *                |         *         |     Y     |
//! | [`reduce_ref`](Collection::reduce_ref)                            |            *              |   *   |              *                |         *         |     N     |
//! | [`rfold`](SequenceInto::rfold)                                    |            *              |       |                               |                   |     Y     |
//! | [`rfold_ref`](Sequence::rfold_ref)                                |            *              |   *   |                               |                   |     N     |
//! | [`sum`](CollectionInto::sum)                                      |            *              |       |              *                |                   |     Y     |
//! | [`sum_keys`](Map::sum_keys)                                       |                           |       |                               |         *         |     Y     |
//! | [`sum_values`](Map::sum_values)                                   |                           |       |                               |         *         |     Y     |
//!
//! ### Selecting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`chunked`](SequenceInto::chunked)                                |            *              |       |                               |                   |     Y     |
//! | [`chunked_by`](SequenceInto::chunked_by)                          |            *              |       |                               |                   |     Y     |
//! | [`chunked_exact`](SequenceInto::chunked_exact)                    |            *              |       |                               |                   |     Y     |
//! | [`cartesian_product`](SequenceInto::cartesian_product)            |            *              |       |                               |                   |     N     |
//! | [`combinations`](CollectionInto::combinations)                    |            *              |       |              *                |                   |     N     |
//! | [`combinations_multi`](SequenceInto::combinations_multi)          |            *              |       |                               |                   |     N     |
//! | [`powerset`](CollectionInto::powerset)                            |            *              |       |              *                |                   |     N     |
//! | [`variations`](SequenceInto::variations)                          |            *              |       |                               |                   |     N     |
//! | [`windowed`](SequenceInto::windowed)                              |            *              |       |                               |                   |     N     |
//! | [`windowed_circular`](SequenceInto::windowed_circular)            |            *              |       |                               |                   |     N     |
//!
//! ### Partitioning
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`divide`](SequenceInto::divide)                                  |            *              |       |                               |                   |     Y     |
//! | [`divide_by`](SequenceInto::divide_by)                            |            *              |       |                               |                   |     Y     |
//! | [`group_by`](CollectionInto::group_by)                            |            *              |       |              *                |                   |     Y     |
//! | [`partition`](CollectionInto::partition)                          |            *              |       |              *                |         *         |     Y     |
//! | [`partition_map`](CollectionInto::partition_map)                  |            *              |       |              *                |         *         |     Y     |
//! | [`partition_map_ref`](CollectionInto::partition_map_ref)          |            *              |       |              *                |         *         |     N     |
//! | [`unzip`](SequenceInto::unzip)                                    |            *              |       |                               |                   |     Y     |
//!
//! ### Merging
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`flat`](CollectionInto::flat)                                    |            *              |       |              *                |                   |     Y     |
//! | [`interleave`](SequenceInto::interleave)                          |            *              |       |                               |                   |     Y     |
//! | [`interleave_exact`](SequenceInto::interleave_exact)              |            *              |       |                               |                   |     Y     |
//! | [`intersperse`](SequenceInto::intersperse)                        |            *              |       |                               |                   |     Y     |
//! | [`intersperse_with`](SequenceInto::intersperse_with)              |            *              |       |                               |                   |     Y     |
//! | [`joined`](Sequence::joined)                                      |            *              |       |                               |                   |     N     |
//! | [`merge`](SequenceInto::merge)                                    |            *              |       |                               |                   |     Y     |
//! | [`merge_by`](SequenceInto::merge_by)                              |            *              |       |                               |                   |     Y     |
//! | [`zip`](SequenceInto::zip)                                        |            *              |       |                               |                   |     Y     |
//! | [`zip_padded`](SequenceInto::zip_padded)                          |            *              |       |                               |                   |     Y     |
//!
//! ### Sorting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`sorted`](SequenceInto::sorted)                                  |            *              |       |                               |                   |     Y     |
//! | [`sorted_by`](SequenceInto::sorted_by)                            |            *              |       |                               |                   |     Y     |
//! | [`sorted_by_cached_key`](SequenceInto::sorted_by_cached_key)      |            *              |       |                               |                   |     Y     |
//! | [`sorted_by_key`](SequenceInto::sorted_by_key)                    |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable`](SequenceInto::sorted_unstable)                |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable_by`](SequenceInto::sorted_unstable_by)          |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable_by_key`](SequenceInto::sorted_unstable_by_key)  |            *              |       |                               |                   |     Y     |
//!
//! ### Converting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`collect`](Transform::collect)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`collect_to`](CollectionInto::collect_to)                        |            *              |   *   |              *                |         *         |     Y     |
//! | [`into_bmap`](TransformInto::into_bmap)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_bset`](TransformInto::into_bset)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_heap`](TransformInto::into_heap)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_list`](TransformInto::into_list)                           |            *              |       |              *                |         *         |     Y     |
//! | [`into_map`](TransformInto::into_map)                             |            *              |       |              *                |         *         |     Y     |
//! | [`into_set`](TransformInto::into_set)                             |            *              |       |              *                |         *         |     Y     |
//! | [`into_vec`](TransformVec::into_vec)                              |            *              |   *   |              *                |         *         |     Y     |
//! | [`into_deque`](TransformInto::into_deque)                         |            *              |       |              *                |         *         |     Y     |
//! | [`to_bmap`](Transform::to_bmap)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`to_bset`](Transform::to_bset)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`to_heap`](Transform::to_heap)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`to_keys`](Map::to_keys)                                         |                           |       |                               |         *         |     N     |
//! | [`to_list`](Transform::to_list)                                   |            *              |   *   |              *                |         *         |     N     |
//! | [`to_map`](Transform::to_map)                                     |            *              |   *   |              *                |         *         |     N     |
//! | [`to_set`](Transform::to_set)                                     |            *              |   *   |              *                |         *         |     N     |
//! | [`to_values`](Map::to_values)                                     |                           |       |                               |         *         |     N     |
//! | [`to_vec`](TransformVec::to_vec)                                  |            *              |   *   |              *                |         *         |     N     |
//! | [`to_deque`](Transform::to_deque)                                 |            *              |   *   |              *                |         *         |     N     |
//!
//! ### Miscellaneous
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`fill`](SequenceInto::fill)                                      |            *              |       |                               |                   |     Y     |
//! | [`fill_with`](CollectionInto::fill_with)                          |            *              |       |              *                |         *         |     Y     |
//! | [`for_each`](Collection::for_each)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`repeat`](List::repeat)                                          |            *              |       |                               |                   |           |
//! | [`unit`](CollectionInto::unit)                                    |            *              |       |              *                |         *         |     Y     |
pub(crate) mod collections;
pub(crate) mod core;
pub(crate) mod extensions;

pub use core::iterable::*;
pub use extensions::*;
