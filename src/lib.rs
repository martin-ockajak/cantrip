#![allow(missing_docs)]
//! Practical extension methods for Rust standard library collections.
//!
//! Enables direct functional-style collection manipulation without the usual iterator boilerplate.
//!
//! ### Features
//!
//! - Equivalents of standard iterator methods are added to standard library collections
//! - Additional utility methods commonly found in collection libraries are also included
//! - Transformation methods return a new collection instance instead of returning an iterator
//! - All methods consider collection instances to be immutable although some may consume them
//! - Asymptotic complexity is optimal and performance overhead is limited to new collection creation
//!
//! ### Examples
//!
//! ```rust
//! use cantrip::*;
//!
//! # let source = vec![1, 2, 3];
//! let a = vec![1, 2, 3];
//!
//! a.fold(0, |r, &x| r + x);        // 6
//!
//! # let a = source.clone();
//! a.filter(|&x| x > 1);            // vec![2, 3]
//!
//! # let a = source.clone();
//! a.map(|x| x + 1);                // vec![2, 3, 4]
//!
//! # let a = source.clone();
//! a.add(1).unique();               // vec![1, 2, 3]
//!
//! # let a = source.clone();
//! a.delete_at(0).tail();           // vec![3]
//!
//! # let a = source.clone();
//! a.interleave(vec![4, 5, 6]);     // vec![(1, 4, 2, 5, 3, 6)]
//!
//! # let a = source.clone();
//! a.group_by(|x| x % 2);           // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
//! ```
//!
//! ### Methods
//!
//! | Method / Collection type                                        | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap |
//! |-----------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|
//! | [`add`](Traversable::add)                                       |            *              |       |              *                |         *         |
//! | [`add_at`](Sequence::add_at)                                    |            *              |       |                               |                   |
//! | [`add_at_multi`](Sequence::add_at_multi)                        |            *              |       |                               |                   |
//! | [`add_multi`](Collectible::add_multi)                           |            *              |       |              *                |         *         |
//! | [`all`](Traversable::all)                                       |            *              |   *   |              *                |         *         |
//! | [`any`](Traversable::any)                                       |            *              |   *   |              *                |         *         |
//! | [`cartesian_product`](Sequence::cartesian_product)              |            *              |       |                               |                   |
//! | [`chunked`](Sequence::chunked)                                  |            *              |       |                               |                   |
//! | [`chunked_by`](Sequence::chunked_by)                            |            *              |       |                               |                   |
//! | [`chunked_exact`](Sequence::chunked_exact)                      |            *              |       |                               |                   |
//! | [`combinations`](Collectible::combinations)                     |            *              |       |              *                |                   |
//! | [`combinations_multi`](Sequence::combinations_multi)            |            *              |       |                               |                   |
//! | [`coalesce`](Sequence::coalesce)                                |            *              |       |                               |                   |
//! | [`common_prefix_length`](Ordered::common_prefix_length)         |            *              |   *   |                               |                   |
//! | [`common_suffix_length`](Ordered::common_suffix_length)         |            *              |   *   |                               |                   |
//! | [`count_by`](Traversable::count_by)                             |            *              |   *   |              *                |         *         |
//! | [`count_unique`](Ordered::count_unique)                         |            *              |   *   |                               |         *         |
//! | [`delete`](Collectible::delete)                                 |            *              |       |              *                |         *         |
//! | [`delete_at`](Sequence::delete_at)                              |            *              |       |                               |                   |
//! | [`delete_at_multi`](Sequence::delete_at_multi)                  |            *              |       |                               |                   |
//! | [`delete_multi`](Collectible::delete_multi)                     |            *              |       |              *                |         *         |
//! | [`duplicates`](Sequence::duplicates)                            |            *              |       |                               |                   |
//! | [`duplicates_by`](Sequence::duplicates_by)                      |            *              |       |                               |                   |
//! | [`enumerate`](Sequence::enumerate)                              |            *              |       |                               |                   |
//! | [`equivalent`](Ordered::equivalent)                             |            *              |   *   |                               |                   |
//! | [`fill`](Sequence::fill)                                        |            *              |       |                               |                   |
//! | [`fill_with`](Collectible::fill_with)                           |            *              |       |              *                |         *         |
//! | [`filter`](Collectible::filter)                                 |            *              |       |              *                |         *         |
//! | [`filter_keys`](Map::filter_keys)                               |                           |       |                               |         *         |
//! | [`filter_map`](Collectible::filter_map)                         |            *              |       |              *                |         *         |
//! | [`filter_map_to`](Collectible::filter_map_to)                   |            *              |       |              *                |         *         |
//! | [`filter_values`](Map::filter_values)                           |                           |       |                               |         *         |
//! | [`find`](Traversable::find)                                     |            *              |   *   |              *                |         *         |
//! | [`find_map`](Traversable::find_map)                             |            *              |   *   |              *                |         *         |
//! | [`find_map_to`](Collectible::find_map_to)                       |            *              |       |              *                |         *         |
//! | [`find_position`](Ordered::find_position)                       |            *              |   *   |                               |                   |
//! | [`first`](List::first)                                          |            *              |   *   |                               |                   |
//! | [`flat_map`](Collectible::flat_map)                             |            *              |       |              *                |         *         |
//! | [`flat_map_to`](Collectible::flat_map_to)                       |            *              |       |              *                |         *         |
//! | [`flat`](Collectible::flat)                                     |            *              |       |              *                |                   |
//! | [`fold`](Traversable::fold)                                     |            *              |   *   |              *                |         *         |
//! | [`fold_to`](Collectible::fold_to)                               |            *              |       |              *                |         *         |
//! | [`for_each`](Traversable::for_each)                             |            *              |   *   |              *                |         *         |
//! | [`frequencies`](Ordered::frequencies)                           |            *              |   *   |                               |                   |
//! | [`frequencies_by`](Ordered::frequencies_by)                     |            *              |   *   |                               |                   |
//! | [`group_by`](Collectible::group_by)                             |            *              |       |              *                |                   |
//! | [`group_fold`](Traversable::group_fold)                         |            *              |   *   |              *                |                   |
//! | [`group_fold_to`](Collectible::group_fold_to)                   |            *              |       |              *                |                   |
//! | [`group_reduce`](Traversable::group_reduce)                     |            *              |   *   |              *                |                   |
//! | [`group_reduce_to`](Collectible::group_reduce_to)               |            *              |       |              *                |                   |
//! | [`init`](Sequence::init)                                        |            *              |   *   |                               |                   |
//! | [`interleave`](Sequence::interleave)                            |            *              |       |                               |                   |
//! | [`interleave_exact`](Sequence::interleave_exact)                |            *              |       |                               |                   |
//! | [`intersect`](Collectible::intersect)                           |            *              |       |              *                |         *         |
//! | [`intersperse`](Sequence::intersperse)                          |            *              |       |                               |                   |
//! | [`intersperse_with`](Sequence::intersperse_with)                |            *              |       |                               |                   |
//! | [`joined`](Ordered::joined)                                     |            *              |   *   |                               |                   |
//! | [`largest`](Collectible::largest)                               |            *              |       |              *                |                   |
//! | [`last`](List::last)                                            |            *              |   *   |                               |                   |
//! | [`map`](Collectible::map)                                       |            *              |       |              *                |         *         |
//! | [`map_to`](Collectible::map_to)                                 |            *              |       |              *                |         *         |
//! | [`map_keys`](Map::map_keys)                                     |                           |       |                               |         *         |
//! | [`map_values`](Map::map_values)                                 |                           |       |                               |         *         |
//! | [`map_while`](Sequence::map_while)                              |            *              |       |                               |                   |
//! | [`max_by`](Traversable::max_by)                                 |            *              |   *   |              *                |         *         |
//! | [`max_by_key`](Traversable::max_by_key)                         |            *              |   *   |              *                |         *         |
//! | [`max_of`](Traversable::max_of)                                 |            *              |   *   |              *                |         *         |
//! | [`merge`](Sequence::merge)                                      |            *              |       |                               |                   |
//! | [`merge_by`](Sequence::merge_by)                                |            *              |       |                               |                   |
//! | [`min_by`](Traversable::min_by)                                 |            *              |   *   |              *                |         *         |
//! | [`min_by_key`](Traversable::min_by_key)                         |            *              |   *   |              *                |         *         |
//! | [`min_of`](Traversable::min_of)                                 |            *              |   *   |              *                |         *         |
//! | [`minmax_by`](Traversable::minmax_by)                           |            *              |   *   |              *                |         *         |
//! | [`minmax_by_key`](Traversable::minmax_by_key)                   |            *              |   *   |              *                |         *         |
//! | [`minmax_of`](Traversable::minmax_of)                           |            *              |   *   |              *                |         *         |
//! | [`move_at`](Sequence::move_at)                                  |            *              |       |                               |                   |
//! | [`pad_left`](Sequence::pad_left)                                |            *              |       |                               |                   |
//! | [`pad_left_with`](Sequence::pad_left_with)                      |            *              |       |                               |                   |
//! | [`pad_right`](Sequence::pad_right)                              |            *              |       |                               |                   |
//! | [`pad_right_with`](Sequence::pad_right_with)                    |            *              |       |                               |                   |
//! | [`partition`](Collectible::partition)                           |            *              |       |              *                |         *         |
//! | [`partition_map`](Collectible::partition_map)                   |            *              |       |              *                |         *         |
//! | [`partition_map_to`](Collectible::partition_map_to)             |            *              |       |              *                |         *         |
//! | [`permutations`](Collectible::permutations)                     |            *              |       |                               |                   |
//! | [`position`](Ordered::position)                                 |            *              |   *   |                               |                   |
//! | [`position_multi`](Ordered::position_multi)                     |            *              |   *   |                               |                   |
//! | [`position_of`](Ordered::position_of)                           |            *              |   *   |                               |                   |
//! | [`position_of_multi`](Ordered::position_of_multi)               |            *              |   *   |                               |                   |
//! | [`position_sequence`](Ordered::position_sequence)               |            *              |   *   |                               |                   |
//! | [`powerset`](Collectible::powerset)                             |            *              |       |              *                |                   |
//! | [`product`](Collectible::product)                               |            *              |       |              *                |                   |
//! | [`product_keys`](Map::product_keys)                             |                           |       |                               |         *         |
//! | [`product_values`](Map::product_values)                         |                           |       |                               |         *         |
//! | [`reduce`](Traversable::reduce)                                 |            *              |   *   |              *                |         *         |
//! | [`reduce_to`](Collectible::reduce_to)                           |            *              |       |              *                |         *         |
//! | [`repeat`](List::repeat)                                        |            *              |       |                               |                   |
//! | [`rev`](Sequence::rev)                                          |            *              |       |                               |                   |
//! | [`rfind`](Ordered::rfind)                                       |            *              |   *   |                               |                   |
//! | [`rfold`](Ordered::rfold)                                       |            *              |   *   |                               |                   |
//! | [`rfold_to`](Sequence::rfold_to)                                |            *              |       |                               |                   |
//! | [`rposition`](Ordered::rposition)                               |            *              |   *   |                               |                   |
//! | [`scan`](Sequence::scan)                                        |            *              |       |                               |                   |
//! | [`scan_to`](Sequence::scan_to)                                  |            *              |       |                               |                   |
//! | [`skip`](Sequence::skip)                                        |            *              |   *   |                               |                   |
//! | [`skip_while`](Sequence::skip_while)                            |            *              |   *   |                               |                   |
//! | [`slice`](Sequence::slice)                                      |            *              |       |                               |                   |
//! | [`smallest`](Collectible::smallest)                             |            *              |       |              *                |                   |
//! | [`sorted`](Sequence::sorted)                                    |            *              |       |                               |                   |
//! | [`sorted_by`](Sequence::sorted_by)                              |            *              |       |                               |                   |
//! | [`sorted_by_cached_key`](Sequence::sorted_by_cached_key)        |            *              |       |                               |                   |
//! | [`sorted_by_key`](Sequence::sorted_by_key)                      |            *              |       |                               |                   |
//! | [`sorted_unstable`](Sequence::sorted_unstable)                  |            *              |       |                               |                   |
//! | [`sorted_unstable_by`](Sequence::sorted_unstable_by)            |            *              |       |                               |                   |
//! | [`sorted_unstable_by_key`](Sequence::sorted_unstable_by_key)    |            *              |       |                               |                   |
//! | [`step_by`](Sequence::step_by)                                  |            *              |       |                               |                   |
//! | [`subset`](Traversable::subset)                                 |            *              |   *   |              *                |         *         |
//! | [`substitute`](Collectible::substitute)                         |            *              |       |              *                |         *         |
//! | [`substitute_at`](Sequence::substitute_at)                      |            *              |       |                               |                   |
//! | [`substitute_at_multi`](Sequence::substitute_at_multi)          |            *              |       |                               |                   |
//! | [`substitute_multi`](Collectible::substitute_multi)             |            *              |       |              *                |         *         |
//! | [`superset`](Traversable::superset)                             |            *              |   *   |              *                |         *         |
//! | [`sum`](Collectible::sum)                                       |            *              |       |              *                |                   |
//! | [`sum_keys`](Map::sum_keys)                                     |                           |       |                               |         *         |
//! | [`sum_values`](Map::sum_values)                                 |                           |       |                               |         *         |
//! | [`tail`](Sequence::tail)                                        |            *              |   *   |                               |                   |
//! | [`take`](Sequence::take)                                        |            *              |   *   |                               |                   |
//! | [`take_while`](Sequence::take_while)                            |            *              |   *   |                               |                   |
//! | [`unique`](Sequence::unique)                                    |            *              |       |                               |                   |
//! | [`unique_by`](Sequence::unique_by)                              |            *              |       |                               |                   |
//! | [`unit`](Collectible::unit)                                     |            *              |       |              *                |         *         |
//! | [`unzip`](Sequence::unzip)                                      |            *              |       |                               |                   |
//! | [`variations`](Sequence::variations)                            |            *              |       |                               |                   |
//! | [`windowed`](Sequence::windowed)                                |            *              |       |                               |                   |
//! | [`windowed_circular`](Sequence::windowed_circular)              |            *              |       |                               |                   |
//! | [`zip`](Sequence::zip)                                          |            *              |       |                               |                   |
//! | [`zip_padded`](Sequence::zip_padded)                            |            *              |       |                               |                   |
pub mod extensions;

pub use extensions::*;
