[![API](https://img.shields.io/badge/Rustdoc-API-mediumpurple)](https://docs.rs/cantrip/latest/cantrip)
![Artifacts](https://img.shields.io/crates/v/cantrip.svg?label=Artifacts)
[![License](https://img.shields.io/github/license/martin-ockajak/cantrip?label=License&color=teal)](https://github.com/martin-ockajak/cantrip/blob/main/LICENSE)
[![Build](https://github.com/martin-ockajak/cantrip/actions/workflows/build.yml/badge.svg)](https://github.com/martin-ockajak/cantrip/actions/workflows/build.yml)

Practical extension methods for Rust standard library collections.

Enables direct functional-style collection manipulation without the usual iterator boilerplate.


## Features

- Equivalents of standard iterator methods are added to standard library collections
- Additional utility methods commonly found in collection libraries are also included
- Transformation methods return a new collection instance instead of returning an iterator
- All methods consider collection instances to be immutable although some may consume them
- Asymptotic complexity is optimal and performance overhead is limited to new collection creation


## Examples

```rust
use cantrip::*;

let a = vec![1, 2, 3];

a.fold(0, |r, &x| r + x);        // 6

a.filter(|&x| x > 1);            // vec![2, 3]

a.map(|x| x + 1);                // vec![2, 3, 4]

a.add(1).unique();               // vec![1, 2, 3]

a.delete_at(0).tail();           // vec![3]

a.interleave(vec![4, 5, 6]);     // vec![(1, 4, 2, 5, 3, 6)]

a.group_by(|x| x % 2);           // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
```


## Methods

| Method / Collection type                                        | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap |
|-----------------------------------------------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|
| [add](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.add)                                       |            *              |       |              *                |         *         |
| [add_at](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.add_at)                                    |            *              |       |                               |                   |
| [add_at_multi](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.add_at_multi)                        |            *              |       |                               |                   |
| [add_multi](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.add_multi)                           |            *              |       |              *                |         *         |
| [all](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.all)                                       |            *              |   *   |              *                |         *         |
| [any](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.any)                                       |            *              |   *   |              *                |         *         |
| [cartesian_product](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.cartesian_product)              |            *              |       |                               |                   |
| [chunked](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.chunked)                                  |            *              |       |                               |                   |
| [chunked_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.chunked_by)                            |            *              |       |                               |                   |
| [chunked_exact](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.chunked_exact)                      |            *              |       |                               |                   |
| [combinations](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.combinations)                     |            *              |       |              *                |                   |
| [combinations_multi](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.combinations_multi)            |            *              |       |                               |                   |
| [coalesce](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.coalesce)                                |            *              |       |                               |                   |
| [common_prefix_length](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.common_prefix_length)         |            *              |   *   |                               |                   |
| [common_suffix_length](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.common_suffix_length)         |            *              |   *   |                               |                   |
| [count_by](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.count_by)                             |            *              |   *   |              *                |         *         |
| [count_unique](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.count_unique)                         |            *              |   *   |                               |         *         |
| [delete](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.delete)                                 |            *              |       |              *                |         *         |
| [delete_at](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.delete_at)                              |            *              |       |                               |                   |
| [delete_at_multi](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.delete_at_multi)                  |            *              |       |                               |                   |
| [delete_multi](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.delete_multi)                     |            *              |       |              *                |         *         |
| [divide](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.divide)                                    |            *              |       |                               |                   |
| [divide_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.divide_by)                              |            *              |       |                               |                   |
| [disjoint](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.disjoint)                             |            *              |   *   |              *                |         *         |
| [duplicates](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.duplicates)                            |            *              |       |                               |                   |
| [duplicates_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.duplicates_by)                      |            *              |       |                               |                   |
| [enumerate](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.enumerate)                              |            *              |       |                               |                   |
| [equivalent](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.equivalent)                             |            *              |   *   |                               |                   |
| [fill](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.fill)                                        |            *              |       |                               |                   |
| [fill_with](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.fill_with)                           |            *              |       |              *                |         *         |
| [filter](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.filter)                                 |            *              |       |              *                |         *         |
| [filter_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.filter_keys)                               |                           |       |                               |         *         |
| [filter_map](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.filter_map)                         |            *              |       |              *                |         *         |
| [filter_map_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.filter_map_to)                   |            *              |       |              *                |         *         |
| [filter_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.filter_values)                           |                           |       |                               |         *         |
| [find](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.find)                                     |            *              |   *   |              *                |         *         |
| [find_map](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.find_map)                             |            *              |   *   |              *                |         *         |
| [find_map_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.find_map_to)                       |            *              |       |              *                |         *         |
| [find_position](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.find_position)                       |            *              |   *   |                               |                   |
| [first](https://docs.rs/cantrip/latest/cantrip/trait.List.html#method.first)                                          |            *              |   *   |                               |                   |
| [flat_map](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.flat_map)                             |            *              |       |              *                |         *         |
| [flat_map_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.flat_map_to)                       |            *              |       |              *                |         *         |
| [flat](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.flat)                                     |            *              |       |              *                |                   |
| [fold](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.fold)                                     |            *              |   *   |              *                |         *         |
| [fold_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.fold_to)                               |            *              |       |              *                |         *         |
| [for_each](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.for_each)                             |            *              |   *   |              *                |         *         |
| [frequencies](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.frequencies)                           |            *              |   *   |                               |                   |
| [frequencies_by](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.frequencies_by)                     |            *              |   *   |                               |                   |
| [group_by](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.group_by)                             |            *              |       |              *                |                   |
| [group_fold](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.group_fold)                         |            *              |   *   |              *                |                   |
| [group_fold_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.group_fold_to)                   |            *              |       |              *                |                   |
| [group_reduce](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.group_reduce)                     |            *              |   *   |              *                |                   |
| [group_reduce_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.group_reduce_to)               |            *              |       |              *                |                   |
| [init](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.init)                                        |            *              |   *   |                               |                   |
| [interleave](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.interleave)                            |            *              |       |                               |                   |
| [interleave_exact](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.interleave_exact)                |            *              |       |                               |                   |
| [intersect](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.intersect)                           |            *              |       |              *                |         *         |
| [intersperse](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.intersperse)                          |            *              |       |                               |                   |
| [intersperse_with](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.intersperse_with)                |            *              |       |                               |                   |
| [joined](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.joined)                                     |            *              |   *   |                               |                   |
| [largest](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.largest)                               |            *              |       |              *                |                   |
| [last](https://docs.rs/cantrip/latest/cantrip/trait.List.html#method.last)                                            |            *              |   *   |                               |                   |
| [map](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.map)                                       |            *              |       |              *                |         *         |
| [map_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.map_to)                                 |            *              |       |              *                |         *         |
| [map_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.map_keys)                                     |                           |       |                               |         *         |
| [map_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.map_values)                                 |                           |       |                               |         *         |
| [map_while](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.map_while)                              |            *              |       |                               |                   |
| [max_by](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.max_by)                                 |            *              |   *   |              *                |         *         |
| [max_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.max_by_key)                         |            *              |   *   |              *                |         *         |
| [max_of](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.max_of)                                 |            *              |   *   |              *                |         *         |
| [merge](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.merge)                                      |            *              |       |                               |                   |
| [merge_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.merge_by)                                |            *              |       |                               |                   |
| [min_by](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.min_by)                                 |            *              |   *   |              *                |         *         |
| [min_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.min_by_key)                         |            *              |   *   |              *                |         *         |
| [min_of](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.min_of)                                 |            *              |   *   |              *                |         *         |
| [minmax_by](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.minmax_by)                           |            *              |   *   |              *                |         *         |
| [minmax_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.minmax_by_key)                   |            *              |   *   |              *                |         *         |
| [minmax_of](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.minmax_of)                           |            *              |   *   |              *                |         *         |
| [move_at](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.move_at)                                  |            *              |       |                               |                   |
| [pad_left](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.pad_left)                                |            *              |       |                               |                   |
| [pad_left_with](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.pad_left_with)                      |            *              |       |                               |                   |
| [pad_right](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.pad_right)                              |            *              |       |                               |                   |
| [pad_right_with](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.pad_right_with)                    |            *              |       |                               |                   |
| [partition](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.partition)                           |            *              |       |              *                |         *         |
| [partition_map](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.partition_map)                   |            *              |       |              *                |         *         |
| [partition_map_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.partition_map_to)             |            *              |       |              *                |         *         |
| [position](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.position)                                 |            *              |   *   |                               |                   |
| [position_multi](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.position_multi)                     |            *              |   *   |                               |                   |
| [position_of](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.position_of)                           |            *              |   *   |                               |                   |
| [position_of_multi](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.position_of_multi)               |            *              |   *   |                               |                   |
| [position_sequence](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.position_sequence)               |            *              |   *   |                               |                   |
| [powerset](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.powerset)                             |            *              |       |              *                |                   |
| [product](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.product)                               |            *              |       |              *                |                   |
| [product_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.product_keys)                             |                           |       |                               |         *         |
| [product_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.product_values)                         |                           |       |                               |         *         |
| [reduce](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.reduce)                                 |            *              |   *   |              *                |         *         |
| [reduce_to](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.reduce_to)                           |            *              |       |              *                |         *         |
| [repeat](https://docs.rs/cantrip/latest/cantrip/trait.List.html#method.repeat)                                        |            *              |       |                               |                   |
| [rev](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.rev)                                          |            *              |       |                               |                   |
| [rfind](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.rfind)                                       |            *              |   *   |                               |                   |
| [rfold](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.rfold)                                       |            *              |   *   |                               |                   |
| [rfold_to](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.rfold_to)                                |            *              |       |                               |                   |
| [rposition](https://docs.rs/cantrip/latest/cantrip/trait.Ordered.html#method.rposition)                               |            *              |   *   |                               |                   |
| [scan](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.scan)                                        |            *              |       |                               |                   |
| [scan_to](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.scan_to)                                  |            *              |       |                               |                   |
| [skip](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.skip)                                        |            *              |   *   |                               |                   |
| [skip_while](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.skip_while)                            |            *              |   *   |                               |                   |
| [slice](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.slice)                                      |            *              |       |                               |                   |
| [smallest](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.smallest)                             |            *              |       |              *                |                   |
| [sorted](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted)                                    |            *              |       |                               |                   |
| [sorted_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted_by)                              |            *              |       |                               |                   |
| [sorted_by_cached_key](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted_by_cached_key)        |            *              |       |                               |                   |
| [sorted_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted_by_key)                      |            *              |       |                               |                   |
| [sorted_unstable](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted_unstable)                  |            *              |       |                               |                   |
| [sorted_unstable_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted_unstable_by)            |            *              |       |                               |                   |
| [sorted_unstable_by_key](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.sorted_unstable_by_key)    |            *              |       |                               |                   |
| [step_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.step_by)                                  |            *              |       |                               |                   |
| [subset](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.subset)                                 |            *              |   *   |              *                |         *         |
| [substitute](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.substitute)                         |            *              |       |              *                |         *         |
| [substitute_at](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.substitute_at)                      |            *              |       |                               |                   |
| [substitute_at_multi](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.substitute_at_multi)          |            *              |       |                               |                   |
| [substitute_multi](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.substitute_multi)             |            *              |       |              *                |         *         |
| [superset](https://docs.rs/cantrip/latest/cantrip/trait.Traversable.html#method.superset)                             |            *              |   *   |              *                |         *         |
| [sum](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.sum)                                       |            *              |       |              *                |                   |
| [sum_keys](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.sum_keys)                                     |                           |       |                               |         *         |
| [sum_values](https://docs.rs/cantrip/latest/cantrip/trait.Map.html#method.sum_values)                                 |                           |       |                               |         *         |
| [swap_at](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.swap_at)                                  |            *              |       |                               |                   |
| [tail](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.tail)                                        |            *              |   *   |                               |                   |
| [take](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.take)                                        |            *              |   *   |                               |                   |
| [take_while](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.take_while)                            |            *              |   *   |                               |                   |
| [unique](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.unique)                                    |            *              |       |                               |                   |
| [unique_by](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.unique_by)                              |            *              |       |                               |                   |
| [unit](https://docs.rs/cantrip/latest/cantrip/trait.Collectible.html#method.unit)                                     |            *              |       |              *                |         *         |
| [unzip](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.unzip)                                      |            *              |       |                               |                   |
| [variations](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.variations)                            |            *              |       |                               |                   |
| [windowed](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.windowed)                                |            *              |       |                               |                   |
| [windowed_circular](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.windowed_circular)              |            *              |       |                               |                   |
| [zip](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.zip)                                          |            *              |       |                               |                   |
| [zip_padded](https://docs.rs/cantrip/latest/cantrip/trait.Sequence.html#method.zip_padded)                            |            *              |       |                               |                   |


## Inspiration

- [Rust Collections](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Scala Collections](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/IndexedSeq.html)
- [Haskell Collections](https://hackage.haskell.org/package/collections-api-1.0.0.0/docs/Data-Collections.html)
- [Python Collections](https://python-reference.readthedocs.io/en/latest/docs/list/index.html)
- [Qt Collections](https://doc.qt.io/qt-6/qlist.html)
- [Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)
- [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)


## Build

### Requirements

- [Rust](https://www.rust-lang.org) 1.79+

### Setup

```shell
cargo install cargo-make
```

### Test

```shell
makers build
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

