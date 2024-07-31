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
//! - All methods treat collection instances as immutable although some consume them
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
//! a.map_ref(|&x| (x, x)).to_map();            // HashMap::from([(1, 1), (2, 2), (3, 3)])
//!
//! # let a = source.clone();
//! a.flat_map(|x| [x, -x]).sorted();           // vec![-3, -2, -1, 1, 2, 3]
//!
//! # let a = source.clone();
//! a.filter(|&x| x > 1).to_set();              // HashSet::from([2, 3])
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
//! a.rev().into_iter().to_deque();             // VecDeque::from([3, 2, 1])
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
//! | [`find_map`](CollectionTo::find_map)                              |            *              |       |              *                |         *         |     Y     |
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
//! | [`add`](CollectionTo::add)                                        |            *              |       |              *                |         *         |     Y     |
//! | [`add_at`](SequenceTo::add_at)                                    |            *              |       |                               |                   |     Y     |
//! | [`add_at_multi`](SequenceTo::add_at_multi)                        |            *              |       |                               |                   |     Y     |
//! | [`add_multi`](CollectionTo::add_multi)                            |            *              |       |              *                |         *         |     Y     |
//! | [`delete`](CollectionTo::delete)                                  |            *              |       |              *                |         *         |     Y     |
//! | [`delete_at`](SequenceTo::delete_at)                              |            *              |       |                               |                   |     Y     |
//! | [`delete_at_multi`](SequenceTo::delete_at_multi)                  |            *              |       |                               |                   |     Y     |
//! | [`delete_multi`](CollectionTo::delete_multi)                      |            *              |       |              *                |         *         |     Y     |
//! | [`move_at`](SequenceTo::move_at)                                  |            *              |       |                               |                   |     Y     |
//! | [`pad_left`](SequenceTo::pad_left)                                |            *              |       |                               |                   |     Y     |
//! | [`pad_left_with`](SequenceTo::pad_left_with)                      |            *              |       |                               |                   |     Y     |
//! | [`pad_right`](SequenceTo::pad_right)                              |            *              |       |                               |                   |     Y     |
//! | [`pad_right_with`](SequenceTo::pad_right_with)                    |            *              |       |                               |                   |     Y     |
//! | [`rev`](SequenceTo::rev)                                          |            *              |       |                               |                   |     Y     |
//! | [`substitute`](CollectionTo::substitute)                          |            *              |       |              *                |         *         |     Y     |
//! | [`substitute_at`](SequenceTo::substitute_at)                      |            *              |       |                               |                   |     Y     |
//! | [`substitute_at_multi`](SequenceTo::substitute_at_multi)          |            *              |       |                               |                   |     Y     |
//! | [`substitute_multi`](CollectionTo::substitute_multi)              |            *              |       |              *                |         *         |     Y     |
//! | [`swap_at`](SequenceTo::swap_at)                                  |            *              |       |                               |                   |     Y     |
//!
//! ### Filtering
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`duplicates`](SequenceTo::duplicates)                            |            *              |       |                               |                   |     Y     |
//! | [`duplicates_by`](SequenceTo::duplicates_by)                      |            *              |       |                               |                   |     Y     |
//! | [`filter`](CollectionTo::filter)                                  |            *              |       |              *                |         *         |     Y     |
//! | [`filter_keys`](Map::filter_keys)                                 |                           |       |                               |         *         |     Y     |
//! | [`filter_map`](CollectionTo::filter_map)                          |            *              |       |              *                |         *         |     Y     |
//! | [`filter_map_ref`](CollectionTo::filter_map_ref)                  |            *              |       |              *                |         *         |     N     |
//! | [`filter_ref`](CollectionTo::filter_ref)                          |            *              |       |              *                |         *         |     N     |
//! | [`filter_values`](Map::filter_values)                             |                           |       |                               |         *         |     Y     |
//! | [`init`](SequenceTo::init)                                        |            *              |       |                               |                   |     Y     |
//! | [`init_ref`](Slice::init_ref)                                     |                           |   *   |                               |                   |     Y     |
//! | [`intersect`](CollectionTo::intersect)                            |            *              |       |              *                |         *         |     Y     |
//! | [`largest`](CollectionTo::largest)                                |            *              |       |              *                |                   |     Y     |
//! | [`slice`](SequenceTo::slice)                                      |            *              |       |                               |                   |     Y     |
//! | [`smallest`](CollectionTo::smallest)                              |            *              |       |              *                |                   |     Y     |
//! | [`skip`](SequenceTo::skip)                                        |            *              |       |                               |                   |     Y     |
//! | [`skip_while`](SequenceTo::skip_while)                            |            *              |       |                               |                   |     Y     |
//! | [`skip_ref`](Slice::skip_ref)                                     |                           |   *   |                               |                   |     Y     |
//! | [`skip_while_ref`](Slice::skip_while_ref)                         |                           |   *   |                               |                   |     Y     |
//! | [`step_by`](SequenceTo::step_by)                                  |            *              |       |                               |                   |     Y     |
//! | [`take`](SequenceTo::take)                                        |            *              |       |                               |                   |     Y     |
//! | [`take_while`](SequenceTo::take_while)                            |            *              |       |                               |                   |     Y     |
//! | [`take_ref`](Slice::take_ref)                                     |                           |   *   |                               |                   |     Y     |
//! | [`take_while_ref`](Slice::take_while_ref)                         |                           |   *   |                               |                   |     Y     |
//! | [`unique`](SequenceTo::unique)                                    |            *              |       |                               |                   |     Y     |
//! | [`unique_by`](SequenceTo::unique_by)                              |            *              |       |                               |                   |     Y     |
//! | [`tail`](SequenceTo::tail)                                        |            *              |       |                               |                   |     Y     |
//! | [`tail_ref`](Slice::tail_ref)                                     |                           |   *   |                               |                   |     N     |
//!
//! ### Mapping
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`coalesce`](SequenceTo::coalesce)                                |            *              |       |                               |                   |     Y     |
//! | [`enumerate`](SequenceTo::enumerate)                              |            *              |       |                               |                   |     Y     |
//! | [`flat_map`](CollectionTo::flat_map)                              |            *              |       |              *                |         *         |     Y     |
//! | [`flat_map_ref`](CollectionTo::flat_map_ref)                      |            *              |       |              *                |         *         |     N     |
//! | [`map`](CollectionTo::map)                                        |            *              |       |              *                |         *         |     Y     |
//! | [`map_ref`](CollectionTo::map_ref)                                |            *              |       |              *                |         *         |     N     |
//! | [`map_keys`](Map::map_keys)                                       |                           |       |                               |         *         |     Y     |
//! | [`map_values`](Map::map_values)                                   |                           |       |                               |         *         |     Y     |
//! | [`map_while`](SequenceTo::map_while)                              |            *              |       |                               |                   |     N     |
//! | [`scan`](SequenceTo::scan)                                        |            *              |       |                               |                   |     Y     |
//! | [`scan_ref`](SequenceTo::scan_ref)                                |            *              |       |                               |                   |     N     |
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
//! | [`subset`](Collection::subset)                                    |            *              |   *   |              *                |         *         |     N     |
//! | [`superset`](Collection::superset)                                |            *              |   *   |              *                |         *         |     N     |
//!
//! ### Aggregating
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`fold`](CollectionTo::fold)                                      |            *              |       |              *                |         *         |     Y     |
//! | [`fold_ref`](Collection::fold_ref)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`group_fold`](CollectionTo::group_fold)                          |            *              |       |              *                |                   |     Y     |
//! | [`group_fold_ref`](Collection::group_fold_ref)                    |            *              |   *   |              *                |                   |     N     |
//! | [`group_reduce`](CollectionTo::group_reduce)                      |            *              |       |              *                |                   |     Y     |
//! | [`group_reduce_ref`](Collection::group_reduce_ref)                |            *              |   *   |              *                |                   |     N     |
//! | [`product`](CollectionTo::product)                                |            *              |       |              *                |                   |     Y     |
//! | [`product_keys`](Map::product_keys)                               |                           |       |                               |         *         |     Y     |
//! | [`product_values`](Map::product_values)                           |                           |       |                               |         *         |     Y     |
//! | [`reduce`](CollectionTo::reduce)                                  |            *              |       |              *                |         *         |     Y     |
//! | [`reduce_ref`](Collection::reduce_ref)                            |            *              |   *   |              *                |         *         |     N     |
//! | [`rfold`](SequenceTo::rfold)                                      |            *              |       |                               |                   |     Y     |
//! | [`rfold_ref`](Sequence::rfold_ref)                                |            *              |   *   |                               |                   |     N     |
//! | [`sum`](CollectionTo::sum)                                        |            *              |       |              *                |                   |     Y     |
//! | [`sum_keys`](Map::sum_keys)                                       |                           |       |                               |         *         |     Y     |
//! | [`sum_values`](Map::sum_values)                                   |                           |       |                               |         *         |     Y     |
//!
//! ### Selecting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`chunked`](SequenceTo::chunked)                                  |            *              |       |                               |                   |     Y     |
//! | [`chunked_by`](SequenceTo::chunked_by)                            |            *              |       |                               |                   |     Y     |
//! | [`chunked_exact`](SequenceTo::chunked_exact)                      |            *              |       |                               |                   |     Y     |
//! | [`cartesian_product`](SequenceTo::cartesian_product)              |            *              |       |                               |                   |     N     |
//! | [`combinations`](CollectionTo::combinations)                      |            *              |       |              *                |                   |     N     |
//! | [`combinations_multi`](SequenceTo::combinations_multi)            |            *              |       |                               |                   |     N     |
//! | [`powerset`](CollectionTo::powerset)                              |            *              |       |              *                |                   |     N     |
//! | [`variations`](SequenceTo::variations)                            |            *              |       |                               |                   |     N     |
//! | [`windowed`](SequenceTo::windowed)                                |            *              |       |                               |                   |     N     |
//! | [`windowed_circular`](SequenceTo::windowed_circular)              |            *              |       |                               |                   |     N     |
//!
//! ### Partitioning
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`divide`](SequenceTo::divide)                                    |            *              |       |                               |                   |     Y     |
//! | [`divide_by`](SequenceTo::divide_by)                              |            *              |       |                               |                   |     Y     |
//! | [`group_by`](CollectionTo::group_by)                              |            *              |       |              *                |                   |     Y     |
//! | [`partition`](CollectionTo::partition)                            |            *              |       |              *                |         *         |     Y     |
//! | [`partitions`](CollectionTo::partitions)                          |            *              |       |              *                |                   |     N     |
//! | [`partition_map`](CollectionTo::partition_map)                    |            *              |       |              *                |         *         |     Y     |
//! | [`partition_map_ref`](CollectionTo::partition_map_ref)            |            *              |       |              *                |         *         |     N     |
//! | [`unzip`](SequenceTo::unzip)                                      |            *              |       |                               |                   |     Y     |
//!
//! ### Merging
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`flat`](CollectionTo::flat)                                      |            *              |       |              *                |                   |     Y     |
//! | [`interleave`](SequenceTo::interleave)                            |            *              |       |                               |                   |     Y     |
//! | [`interleave_exact`](SequenceTo::interleave_exact)                |            *              |       |                               |                   |     Y     |
//! | [`intersperse`](SequenceTo::intersperse)                          |            *              |       |                               |                   |     Y     |
//! | [`intersperse_with`](SequenceTo::intersperse_with)                |            *              |       |                               |                   |     Y     |
//! | [`joined`](Sequence::joined)                                      |            *              |       |                               |                   |     N     |
//! | [`merge`](SequenceTo::merge)                                      |            *              |       |                               |                   |     Y     |
//! | [`merge_by`](SequenceTo::merge_by)                                |            *              |       |                               |                   |     Y     |
//! | [`zip`](SequenceTo::zip)                                          |            *              |       |                               |                   |     Y     |
//! | [`zip_padded`](SequenceTo::zip_padded)                            |            *              |       |                               |                   |     Y     |
//!
//! ### Sorting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`sorted`](SequenceTo::sorted)                                    |            *              |       |                               |                   |     Y     |
//! | [`sorted_by`](SequenceTo::sorted_by)                              |            *              |       |                               |                   |     Y     |
//! | [`sorted_by_cached_key`](SequenceTo::sorted_by_cached_key)        |            *              |       |                               |                   |     Y     |
//! | [`sorted_by_key`](SequenceTo::sorted_by_key)                      |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable`](SequenceTo::sorted_unstable)                  |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable_by`](SequenceTo::sorted_unstable_by)            |            *              |       |                               |                   |     Y     |
//! | [`sorted_unstable_by_key`](SequenceTo::sorted_unstable_by_key)    |            *              |       |                               |                   |     Y     |
//!
//! ### Converting
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`collect`](CollectionTo::collect)                                |            *              |       |              *                |         *         |     Y     |
//! | [`to_bmap`](Convert::to_bmap)                                     |            *              |       |              *                |         *         |     Y     |
//! | [`to_bset`](Convert::to_bset)                                     |            *              |       |              *                |         *         |     Y     |
//! | [`to_heap`](Convert::to_heap)                                     |            *              |       |              *                |         *         |     Y     |
//! | [`to_keys`](Map::to_keys)                                         |                           |       |                               |         *         |     Y     |
//! | [`to_list`](Convert::to_list)                                     |            *              |       |              *                |         *         |     Y     |
//! | [`to_map`](Convert::to_map)                                       |            *              |       |              *                |         *         |     Y     |
//! | [`to_set`](Convert::to_set)                                       |            *              |       |              *                |         *         |     Y     |
//! | [`to_values`](Map::to_values)                                     |                           |       |                               |         *         |     Y     |
//! | [`to_vec`](Convert::to_vec)                                       |            *              |       |              *                |         *         |     Y     |
//! | [`to_deque`](Convert::to_deque)                                   |            *              |       |              *                |         *         |     Y     |
//!
//! ### Miscellaneous
//!
//! | Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
//! |-------------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
//! | [`fill`](SequenceTo::fill)                                        |            *              |       |                               |                   |     Y     |
//! | [`fill_with`](CollectionTo::fill_with)                            |            *              |       |              *                |         *         |     Y     |
//! | [`for_each`](Collection::for_each)                                |            *              |   *   |              *                |         *         |     N     |
//! | [`repeat`](List::repeat)                                          |            *              |       |                               |                   |           |
//! | [`unit`](CollectionTo::unit)                                      |            *              |       |              *                |         *         |     Y     |
pub(crate) mod collections;
pub(crate) mod core;
pub(crate) mod extensions;

pub use core::iterable::*;
pub use extensions::*;
