[![API](https://img.shields.io/badge/Rustdoc-API-mediumpurple)](https://docs.rs/cantrip/latest/cantrip)
[![Artifacts](https://img.shields.io/crates/v/cantrip.svg?label=Artifacts)](https://crates.io/crates/cantrip)
[![License](https://img.shields.io/github/license/martin-ockajak/cantrip?label=License&color=teal)](https://github.com/martin-ockajak/cantrip/blob/main/LICENSE)
[![Build](https://github.com/martin-ockajak/cantrip/actions/workflows/build.yml/badge.svg)](https://github.com/martin-ockajak/cantrip/actions/workflows/build.yml)

A Swiss Army Knife for for Rust standard library collections.

Enables direct functional-style collection manipulation without the usual
iterator boilerplate and provides many additional operations.

- Enjoy cleaner code with less `.into_iter()`, `.collect()` and `.clone()`.
- Feel free to treat standard collections as immutable values to be reused at will.
- No learning required, just keep using code completion to quickly find a method you need.


## Overview

- Existing standard library collections are extended with equivalents of iterator methods
- Additional utility methods commonly found in collection libraries are also included

- Methods which modify a collection return a new collection instead of an iterator
- All methods treat collection instances as immutable although some consume them

- Standard library method naming conventions are followed as closely as possible
- Performance is near optimal with overhead limited to new collection creation


## Functionality

- [Searching](#searching) - [Modifying](#modifying) - [Filtering](#filtering) -
  [Mapping](#mapping) - [Inspecting](#inspecting) - [Aggregating](#aggregating)

- [Selecting](#selecting) - [Converting](#converting) - [Partitioning](#partitioning) -
  [Merging](#merging) - [Sorting](#sorting) - [Miscellaneous](#miscellaneous)


## Examples

```rust
use cantrip::*;

let a = vec![1, 2, 3];

a.fold(0, |r, x| r + x);                    // 6

a.map_ref(|&x| (x, x)).to_map();            // HashMap::from([(1, 1), (2, 2), (3, 3)])

a.flat_map(|x| [x, -x]).sorted();           // vec![-3, -2, -1, 1, 2, 3]

a.filter(|&x| x > 1).to_set();              // HashSet::from([2, 3])

a.group_by(|x| x % 2);                      // HashMap::from([(0, vec![2]), (1, vec![1, 3])])

a.delete(&1).add(2).unique();               // vec![2, 3]

a.substitute_at(0, 4).to_list();            // LinkedList::from([4, 2, 3])

a.position_multi(|&x| x % 2 == 1);          // vec![0, 2]

a.rev().into_iter().to_deque();             // VecDeque::from([3, 2, 1])
```


## Methods

### Searching

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [find](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.find)                                        |            *              |   *   |              *                |         *         |     N     |
| [find_map](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.find_map)                              |            *              |       |              *                |         *         |     Y     |
| [find_map_ref](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.find_map_ref)                        |            *              |   *   |              *                |         *         |     N     |
| [find_position](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.find_position)                        |            *              |   *   |                               |                   |     N     |
| [first](https://docs.rs/cantrip/latest/cantrip/trait.List.html#tymethod.first)                                            |            *              |   *   |                               |                   |     N     |
| [last](https://docs.rs/cantrip/latest/cantrip/trait.List.html#tymethod.last)                                              |            *              |       |                               |                   |     N     |
| [max_by](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.max_by)                                    |            *              |   *   |              *                |         *         |     N     |
| [max_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.max_by_key)                            |            *              |   *   |              *                |         *         |     N     |
| [max_of](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.max_of)                                    |            *              |   *   |              *                |         *         |     N     |
| [min_by](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.min_by)                                    |            *              |   *   |              *                |         *         |     N     |
| [min_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.min_by_key)                            |            *              |   *   |              *                |         *         |     N     |
| [min_of](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.min_of)                                    |            *              |   *   |              *                |         *         |     N     |
| [minmax_by](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.minmax_by)                              |            *              |   *   |              *                |         *         |     N     |
| [minmax_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.minmax_by_key)                      |            *              |   *   |              *                |         *         |     N     |
| [minmax_of](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.minmax_of)                              |            *              |   *   |              *                |         *         |     N     |
| [position](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.position)                                  |            *              |   *   |                               |                   |     N     |
| [position_multi](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.position_multi)                      |            *              |   *   |                               |                   |     N     |
| [position_of](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.position_of)                            |            *              |   *   |                               |                   |     N     |
| [position_of_multi](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.position_of_multi)                |            *              |   *   |                               |                   |     N     |
| [position_sequence](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.position_sequence)                |            *              |   *   |                               |                   |     N     |
| [rfind](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.rfind)                                        |            *              |   *   |                               |                   |     N     |
| [rposition](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.rposition)                                |            *              |   *   |                               |                   |     N     |

### Modifying

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [add](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.add)                                        |            *              |       |              *                |         *         |     Y     |
| [add_at](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.add_at)                                    |            *              |       |                               |                   |     Y     |
| [add_at_multi](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.add_at_multi)                        |            *              |       |                               |                   |     Y     |
| [add_multi](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.add_multi)                            |            *              |       |              *                |         *         |     Y     |
| [delete](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.delete)                                  |            *              |       |              *                |         *         |     Y     |
| [delete_at](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.delete_at)                              |            *              |       |                               |                   |     Y     |
| [delete_at_multi](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.delete_at_multi)                  |            *              |       |                               |                   |     Y     |
| [delete_multi](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.delete_multi)                      |            *              |       |              *                |         *         |     Y     |
| [move_at](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.move_at)                                  |            *              |       |                               |                   |     Y     |
| [pad_left](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.pad_left)                                |            *              |       |                               |                   |     Y     |
| [pad_left_with](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.pad_left_with)                      |            *              |       |                               |                   |     Y     |
| [pad_right](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.pad_right)                              |            *              |       |                               |                   |     Y     |
| [pad_right_with](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.pad_right_with)                    |            *              |       |                               |                   |     Y     |
| [rev](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.rev)                                          |            *              |       |                               |                   |     Y     |
| [substitute](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.substitute)                          |            *              |       |              *                |         *         |     Y     |
| [substitute_at](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.substitute_at)                      |            *              |       |                               |                   |     Y     |
| [substitute_at_multi](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.substitute_at_multi)          |            *              |       |                               |                   |     Y     |
| [substitute_multi](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.substitute_multi)              |            *              |       |              *                |         *         |     Y     |
| [swap_at](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.swap_at)                                  |            *              |       |                               |                   |     Y     |

### Filtering

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [duplicates](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.duplicates)                            |            *              |       |                               |                   |     Y     |
| [duplicates_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.duplicates_by)                      |            *              |       |                               |                   |     Y     |
| [filter](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.filter)                                  |            *              |       |              *                |         *         |     Y     |
| [filter_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.filter_keys)                                 |                           |       |                               |         *         |     Y     |
| [filter_map](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.filter_map)                          |            *              |       |              *                |         *         |     Y     |
| [filter_map_ref](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.filter_map_ref)                  |            *              |       |              *                |         *         |     N     |
| [filter_ref](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.filter_ref)                          |            *              |       |              *                |         *         |     N     |
| [filter_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.filter_values)                             |                           |       |                               |         *         |     Y     |
| [init](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.init)                                        |            *              |       |                               |                   |     Y     |
| [init_ref](https://docs.rs/cantrip/latest/cantrip/trait.Slice.html#method.init_ref)                                     |                           |   *   |                               |                   |     Y     |
| [intersect](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.intersect)                            |            *              |       |              *                |         *         |     Y     |
| [largest](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.largest)                                |            *              |       |              *                |                   |     Y     |
| [slice](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.slice)                                      |            *              |       |                               |                   |     Y     |
| [smallest](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.smallest)                              |            *              |       |              *                |                   |     Y     |
| [skip](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.skip)                                        |            *              |       |                               |                   |     Y     |
| [skip_while](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.skip_while)                            |            *              |       |                               |                   |     Y     |
| [skip_ref](https://docs.rs/cantrip/latest/cantrip/trait.Slice.html#method.skip_ref)                                     |                           |   *   |                               |                   |     Y     |
| [skip_while_ref](https://docs.rs/cantrip/latest/cantrip/trait.Slice.html#method.skip_while_ref)                         |                           |   *   |                               |                   |     Y     |
| [step_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.step_by)                                  |            *              |       |                               |                   |     Y     |
| [take](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.take)                                        |            *              |       |                               |                   |     Y     |
| [take_while](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.take_while)                            |            *              |       |                               |                   |     Y     |
| [take_ref](https://docs.rs/cantrip/latest/cantrip/trait.Slice.html#method.take_ref)                                     |                           |   *   |                               |                   |     Y     |
| [take_while_ref](https://docs.rs/cantrip/latest/cantrip/trait.Slice.html#method.take_while_ref)                         |                           |   *   |                               |                   |     Y     |
| [unique](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.unique)                                    |            *              |       |                               |                   |     Y     |
| [unique_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.unique_by)                              |            *              |       |                               |                   |     Y     |
| [tail](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.tail)                                        |            *              |       |                               |                   |     Y     |
| [tail_ref](https://docs.rs/cantrip/latest/cantrip/trait.Slice.html#tymethod.tail_ref)                                     |                           |   *   |                               |                   |     N     |

### Mapping

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [coalesce](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.coalesce)                                |            *              |       |                               |                   |     Y     |
| [enumerate](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.enumerate)                              |            *              |       |                               |                   |     Y     |
| [flat_map](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.flat_map)                              |            *              |       |              *                |         *         |     Y     |
| [flat_map_ref](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.flat_map_ref)                      |            *              |       |              *                |         *         |     N     |
| [map](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.map)                                        |            *              |       |              *                |         *         |     Y     |
| [map_ref](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.map_ref)                                |            *              |       |              *                |         *         |     N     |
| [map_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.map_keys)                                       |                           |       |                               |         *         |     Y     |
| [map_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.map_values)                                   |                           |       |                               |         *         |     Y     |
| [map_while](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.map_while)                              |            *              |       |                               |                   |     N     |
| [scan](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.scan)                                        |            *              |       |                               |                   |     Y     |
| [scan_ref](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.scan_ref)                                |            *              |       |                               |                   |     N     |

### Inspecting

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [all](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.all)                                          |            *              |   *   |              *                |         *         |     N     |
| [any](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.any)                                          |            *              |   *   |              *                |         *         |     N     |
| [common_prefix_length](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.common_prefix_length)          |            *              |   *   |                               |                   |     N     |
| [common_suffix_length](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.common_suffix_length)          |            *              |   *   |                               |                   |     N     |
| [count_by](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.count_by)                                |            *              |   *   |              *                |         *         |     N     |
| [count_unique](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.count_unique)                          |            *              |   *   |                               |         *         |     N     |
| [disjoint](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.disjoint)                                |            *              |   *   |              *                |         *         |     N     |
| [equivalent](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.equivalent)                              |            *              |   *   |                               |                   |     N     |
| [frequencies](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.frequencies)                            |            *              |   *   |                               |                   |     N     |
| [frequencies_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.frequencies_by)                      |            *              |   *   |                               |                   |     N     |
| [subset](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.subset)                                    |            *              |   *   |              *                |         *         |     N     |
| [superset](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.superset)                                |            *              |   *   |              *                |         *         |     N     |

### Aggregating

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [fold](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.fold)                                      |            *              |       |              *                |         *         |     Y     |
| [fold_ref](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.fold_ref)                                |            *              |   *   |              *                |         *         |     N     |
| [group_fold](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.group_fold)                          |            *              |       |              *                |                   |     Y     |
| [group_fold_ref](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.group_fold_ref)                    |            *              |   *   |              *                |                   |     N     |
| [group_reduce](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.group_reduce)                      |            *              |       |              *                |                   |     Y     |
| [group_reduce_ref](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.group_reduce_ref)                |            *              |   *   |              *                |                   |     N     |
| [product](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.product)                                |            *              |       |              *                |                   |     Y     |
| [product_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.product_keys)                               |                           |       |                               |         *         |     Y     |
| [product_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.product_values)                           |                           |       |                               |         *         |     Y     |
| [reduce](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.reduce)                                  |            *              |       |              *                |         *         |     Y     |
| [reduce_ref](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.reduce_ref)                            |            *              |   *   |              *                |         *         |     N     |
| [rfold](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.rfold)                                      |            *              |       |                               |                   |     Y     |
| [rfold_ref](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.rfold_ref)                                |            *              |   *   |                               |                   |     N     |
| [sum](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.sum)                                        |            *              |       |              *                |                   |     Y     |
| [sum_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.sum_keys)                                       |                           |       |                               |         *         |     Y     |
| [sum_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.sum_values)                                   |                           |       |                               |         *         |     Y     |

### Selecting

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [chunked](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.chunked)                                  |            *              |       |                               |                   |     Y     |
| [chunked_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.chunked_by)                            |            *              |       |                               |                   |     Y     |
| [chunked_exact](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.chunked_exact)                      |            *              |       |                               |                   |     Y     |
| [cartesian_product](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.cartesian_product)              |            *              |       |                               |                   |     N     |
| [combinations](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.combinations)                      |            *              |       |              *                |                   |     N     |
| [combinations_multi](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.combinations_multi)            |            *              |       |                               |                   |     N     |
| [powerset](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.powerset)                              |            *              |       |              *                |                   |     N     |
| [variations](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.variations)                            |            *              |       |                               |                   |     N     |
| [windowed](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.windowed)                                |            *              |       |                               |                   |     N     |
| [windowed_circular](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#tymethod.windowed_circular)              |            *              |       |                               |                   |     N     |

### Partitioning

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [divide](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.divide)                                    |            *              |       |                               |                   |     Y     |
| [divide_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.divide_by)                              |            *              |       |                               |                   |     Y     |
| [group_by](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.group_by)                              |            *              |       |              *                |                   |     Y     |
| [partition](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.partition)                            |            *              |       |              *                |         *         |     Y     |
| [partitions](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.partitions)                          |            *              |       |              *                |                   |     N     |
| [partition_map](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.partition_map)                    |            *              |       |              *                |         *         |     Y     |
| [partition_map_ref](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#tymethod.partition_map_ref)            |            *              |       |              *                |         *         |     N     |
| [unzip](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.unzip)                                      |            *              |       |                               |                   |     Y     |

### Merging

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [flat](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.flat)                                      |            *              |       |              *                |                   |     Y     |
| [interleave](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.interleave)                            |            *              |       |                               |                   |     Y     |
| [interleave_exact](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.interleave_exact)                |            *              |       |                               |                   |     Y     |
| [intersperse](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.intersperse)                          |            *              |       |                               |                   |     Y     |
| [intersperse_with](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.intersperse_with)                |            *              |       |                               |                   |     Y     |
| [joined](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#tymethod.joined)                                      |            *              |       |                               |                   |     N     |
| [merge](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.merge)                                      |            *              |       |                               |                   |     Y     |
| [merge_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.merge_by)                                |            *              |       |                               |                   |     Y     |
| [zip](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.zip)                                          |            *              |       |                               |                   |     Y     |
| [zip_padded](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.zip_padded)                            |            *              |       |                               |                   |     Y     |

### Sorting

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [sorted](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted)                                    |            *              |       |                               |                   |     Y     |
| [sorted_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted_by)                              |            *              |       |                               |                   |     Y     |
| [sorted_by_cached_key](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted_by_cached_key)        |            *              |       |                               |                   |     Y     |
| [sorted_by_key](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted_by_key)                      |            *              |       |                               |                   |     Y     |
| [sorted_unstable](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted_unstable)                  |            *              |       |                               |                   |     Y     |
| [sorted_unstable_by](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted_unstable_by)            |            *              |       |                               |                   |     Y     |
| [sorted_unstable_by_key](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.sorted_unstable_by_key)    |            *              |       |                               |                   |     Y     |

### Converting

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [collect](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.collect)                                |            *              |       |              *                |         *         |     Y     |
| [to_bmap](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_bmap)                                     |            *              |       |              *                |         *         |     Y     |
| [to_bset](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_bset)                                     |            *              |       |              *                |         *         |     Y     |
| [to_heap](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_heap)                                     |            *              |       |              *                |         *         |     Y     |
| [to_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.to_keys)                                         |                           |       |                               |         *         |     Y     |
| [to_list](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_list)                                     |            *              |       |              *                |         *         |     Y     |
| [to_map](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_map)                                       |            *              |       |              *                |         *         |     Y     |
| [to_set](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_set)                                       |            *              |       |              *                |         *         |     Y     |
| [to_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.to_values)                                     |                           |       |                               |         *         |     Y     |
| [to_vec](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_vec)                                       |            *              |       |              *                |         *         |     Y     |
| [to_deque](https://docs.rs/cantrip/latest/cantrip/trait.Convert.html#method.to_deque)                                   |            *              |       |              *                |         *         |     Y     |

### Miscellaneous

| Method / Collection type                                          | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Consuming |
|:-----------------------------------------------------------------:|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|:---------:|
| [fill](https://docs.rs/cantrip/latest/cantrip/trait.SequenceTo.html#method.fill)                                        |            *              |       |                               |                   |     Y     |
| [fill_with](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.fill_with)                            |            *              |       |              *                |         *         |     Y     |
| [for_each](https://docs.rs/cantrip/latest/cantrip/trait.Collection.html#tymethod.for_each)                                |            *              |   *   |              *                |         *         |     N     |
| [repeat](https://docs.rs/cantrip/latest/cantrip/trait.List::repeat)                                          |            *              |       |                               |                   |           |
| [unit](https://docs.rs/cantrip/latest/cantrip/trait.CollectionTo.html#method.unit)                                      |            *              |       |              *                |         *         |     Y     |


## Inspired by

- [Rust Collections](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Scala Collections](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/IndexedSeq.html)
- [Haskell Collections](https://hackage.haskell.org/package/collections-api-1.0.0.0/docs/Data-Collections.html)
- [Python Collections](https://python-reference.readthedocs.io/en/latest/docs/list/index.html)
- [Qt Collections](https://doc.qt.io/qt-6/qlist.html)
- [Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)
- [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)


## Build

### Requirements

- [Rust](https://www.rust-lang.org) 1.85+

### Test

```shell
cargo test
```

### Benchmark

```shell
cargo bench
```


## Contributing

Please feel free to open an [issue](https://github.com/martin-ockajak/cantrip/issues/new) or a
[pull request](https://github.com/martin-ockajak/cantrip/compare)
with questions, ideas, features, improvements or fixes.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

