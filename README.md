[![Documentation](https://img.shields.io/badge/Website-documentation-blue)](https://github.com/martin-ockajak/cantrip)
[![License](https://img.shields.io/github/license/martin-ockajak/cantrip?label=License&color=teal)](https://github.com/martin-ockajak/cantrip/blob/main/LICENSE)
[![Build](https://github.com/martin-ockajak/cantrip/actions/workflows/build.yml/badge.svg)](https://github.com/martin-ockajak/cantrip/actions/workflows/build.yml)

Convenient extension methods for Rust standard library collections.

Enables direct functional-style collection manipulation without the usual iterator boilerplate.

### Features

- Equivalents of standard iterator methods are added to standard library collections
- Additional utility methods commonly found in collection libraries are also included
- Transformation methods return a new collection instance instead of returning an iterator
- All methods consider collection instances to be immutable although some may consume them
- Asymptotic complexity is optimal and performance overhead is limited to new collection creation

### Examples

```rust
use cantrip::*;

let data = vec![1, 2, 3];

data.fold_to(0, |r, x| r + x);       // 6

data.filter(|&x| x > 1);          // vec![2, 3]

data.map(|x| x + 1);              // vec![2, 3, 4]

data.add(1).unique();             // vec![1, 2, 3]

data.delete_at(0).tail();         // vec![3]

data.interleave(vec![4, 5, 6]);   // vec![(1, 4, 2, 5, 3, 6)]

data.group_by(|x| x % 2);         // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
```

### Methods

| Method                   | Vec, VecDeque, LinkedList | Slice | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap |
|--------------------------|:-------------------------:|:-----:|:-----------------------------:|:-----------------:|
| *add*                    |            *              |       |              *                |         *         |
| *add_at*                 |            *              |       |                               |                   |
| *add_at_multi*           |            *              |       |                               |                   |
| *add_multi*              |            *              |       |              *                |         *         |
| *all*                    |            *              |   *   |              *                |         *         |
| *any*                    |            *              |   *   |              *                |         *         |
| *cartesian_product*      |            *              |       |                               |                   |
| *chunked*                |            *              |       |                               |                   |
| *chunked_by*             |            *              |       |                               |                   |
| *chunked_exact*          |            *              |       |                               |                   |
| *combinations*           |            *              |       |              *                |                   |
| *combinations_multi*     |            *              |       |                               |                   |
| *coalesce*               |            *              |       |                               |                   |
| *common_prefix_length*   |            *              |   *   |                               |                   |
| *common_suffix_length*   |            *              |   *   |                               |                   |
| *count_by*               |            *              |   *   |              *                |         *         |
| *count_unique*           |            *              |   *   |                               |         *         |
| *delete*                 |            *              |       |              *                |         *         |
| *delete_at*              |            *              |       |                               |                   |
| *delete_at_multi*        |            *              |       |                               |                   |
| *delete_multi*           |            *              |       |              *                |         *         |
| *duplicates*             |            *              |       |                               |                   |
| *duplicates_by*          |            *              |       |                               |                   |
| *enumerate*              |            *              |       |                               |                   |
| *equivalent*             |            *              |   *   |                               |                   |
| *fill*                   |            *              |       |                               |                   |
| *fill_with*              |            *              |       |              *                |         *         |
| *filter*                 |            *              |       |              *                |         *         |
| *filter_keys*            |                           |       |                               |         *         |
| *filter_map*             |            *              |       |              *                |         *         |
| *filter_map_to*          |            *              |       |              *                |         *         |
| *filter_values*          |                           |       |                               |         *         |
| *find*                   |            *              |   *   |              *                |         *         |
| *find_map*               |            *              |   *   |              *                |         *         |
| *find_map_to*            |            *              |       |              *                |         *         |
| *find_position*          |            *              |   *   |                               |                   |
| *first*                  |            *              |   *   |                               |                   |
| *flat_map*               |            *              |       |              *                |         *         |
| *flat_map_to*            |            *              |       |              *                |         *         |
| *flat*                   |            *              |       |              *                |                   |
| *fold*                   |            *              |   *   |              *                |         *         |
| *fold_to*                |            *              |       |              *                |         *         |
| *for_each*               |            *              |   *   |              *                |         *         |
| *frequencies*            |            *              |   *   |                               |                   |
| *frequencies_by*         |            *              |   *   |                               |                   |
| *group_by*               |            *              |       |              *                |                   |
| *group_fold*             |            *              |   *   |              *                |                   |
| *group_fold_to*          |            *              |       |              *                |                   |
| *group_reduce*           |            *              |   *   |              *                |                   |
| *group_reduce_to*        |            *              |       |              *                |                   |
| *interleave*             |            *              |       |                               |                   |
| *interleave_exact*       |            *              |       |                               |                   |
| *intersect*              |            *              |       |              *                |         *         |
| *intersperse*            |            *              |       |                               |                   |
| *intersperse_with*       |            *              |       |                               |                   |
| *init*                   |            *              |   *   |                               |                   |
| *joined*                 |            *              |   *   |                               |                   |
| *largest*                |            *              |       |              *                |                   |
| *last*                   |            *              |   *   |                               |                   |
| *map*                    |            *              |       |              *                |         *         |
| *map_to*                 |            *              |       |              *                |         *         |
| *map_keys*               |                           |       |                               |         *         |
| *map_values*             |                           |       |                               |         *         |
| *map_while*              |            *              |       |                               |                   |
| *max_by*                 |            *              |   *   |              *                |         *         |
| *max_by_key*             |            *              |   *   |              *                |         *         |
| *max_of*                 |            *              |   *   |              *                |         *         |
| *merge*                  |            *              |       |                               |                   |
| *merge_by*               |            *              |       |                               |                   |
| *min_by*                 |            *              |   *   |              *                |         *         |
| *min_by_key*             |            *              |   *   |              *                |         *         |
| *min_of*                 |            *              |   *   |              *                |         *         |
| *minmax_by*              |            *              |   *   |              *                |         *         |
| *minmax_by_key*          |            *              |   *   |              *                |         *         |
| *minmax_of*              |            *              |   *   |              *                |         *         |
| *move_at*                |            *              |       |                               |                   |
| *pad_left*               |            *              |       |                               |                   |
| *pad_left_with*          |            *              |       |                               |                   |
| *pad_right*              |            *              |       |                               |                   |
| *pad_right_with*         |            *              |       |                               |                   |
| *partition*              |            *              |       |              *                |         *         |
| *partition_map*          |            *              |       |              *                |         *         |
| *partition_map_to*       |            *              |       |              *                |         *         |
| *permutations*           |            *              |       |                               |                   |
| *position*               |            *              |   *   |                               |                   |
| *position_multi*         |            *              |   *   |                               |                   |
| *position_of*            |            *              |   *   |                               |                   |
| *position_of_multi*      |            *              |   *   |                               |                   |
| *position_sequence*      |            *              |   *   |                               |                   |
| *powerset*               |            *              |       |              *                |                   |
| *product*                |            *              |       |              *                |                   |
| *product_keys*           |                           |       |                               |         *         |
| *product_values*         |                           |       |                               |         *         |
| *reduce*                 |            *              |   *   |              *                |         *         |
| *reduce_to*              |            *              |       |              *                |         *         |
| *repeat*                 |            *              |       |                               |                   |
| *rev*                    |            *              |       |                               |                   |
| *rfind*                  |            *              |   *   |                               |                   |
| *rfold*                  |            *              |   *   |                               |                   |
| *rfold_to*               |            *              |       |                               |                   |
| *rposition*              |            *              |   *   |                               |                   |
| *scan*                   |            *              |       |                               |                   |
| *scan_to*                |            *              |       |                               |                   |
| *skip*                   |            *              |   *   |                               |                   |
| *skip_while*             |            *              |   *   |                               |                   |
| *slice*                  |            *              |       |                               |                   |
| *smallest*               |            *              |       |              *                |                   |
| *sorted*                 |            *              |       |                               |                   |
| *sorted_by*              |            *              |       |                               |                   |
| *sorted_by_cached_key*   |            *              |       |                               |                   |
| *sorted_by_key*          |            *              |       |                               |                   |
| *sorted_unstable*        |            *              |       |                               |                   |
| *sorted_unstable_by*     |            *              |       |                               |                   |
| *sorted_unstable_by_key* |            *              |       |                               |                   |
| *splice*                 |            *              |       |                               |                   |
| *step_by*                |            *              |       |                               |                   |
| *subset*                 |            *              |   *   |              *                |         *         |
| *substitute*             |            *              |       |              *                |         *         |
| *substitute_at*          |            *              |       |                               |                   |
| *substitute_at_multi*    |            *              |       |                               |                   |
| *substitute_multi*       |            *              |       |              *                |         *         |
| *superset*               |            *              |   *   |              *                |         *         |
| *sum*                    |            *              |       |              *                |                   |
| *sum_keys*               |                           |       |                               |         *         |
| *sum_values*             |                           |       |                               |         *         |
| *tail*                   |            *              |   *   |                               |                   |
| *take*                   |            *              |   *   |                               |                   |
| *take_while*             |            *              |   *   |                               |                   |
| *unique*                 |            *              |       |                               |                   |
| *unique_by*              |            *              |       |                               |                   |
| *unit*                   |            *              |       |              *                |         *         |
| *unzip*                  |            *              |       |                               |                   |
| *variations*             |            *              |       |                               |                   |
| *windowed*               |            *              |       |                               |                   |
| *windowed_circular*      |            *              |       |                               |                   |
| *zip*                    |            *              |       |                               |                   |
| *zip_padded*             |            *              |       |                               |                   |

### Inspiration

- [Rust Collections](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Scala Collections](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/IndexedSeq.html)
- [Haskell Collections](https://hackage.haskell.org/package/collections-api-1.0.0.0/docs/Data-Collections.html)
- [Python Collections](https://python-reference.readthedocs.io/en/latest/docs/list/index.html)
- [Qt Collections](https://doc.qt.io/qt-6/qlist.html)
- [Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)
- [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)

### Build

#### Requirements

- [Rust](https://www.rust-lang.org) 1.79+
- [Cargo Make](https://github.com/sagiegurari/cargo-make) 0.37+

#### Command

```shell
makers build
```

