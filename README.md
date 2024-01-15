Convenient extension methods for Rust standard library collections.

It enables collection manipulation in a functional style without the usual Rust boilerplate.


### Features

- Equivalents of suitable iterator methods are added to standard library collections
- Additional utility methods commonly found in collection libraries are also included
- Transformation methods return a new collection instance instead of returning an iterator
- All methods consider collection instances to be immutable although some may consume them
- Asymptotic complexity is optimal and performance overhead is limited to new collection creation

### Examples

```rust
  use cantrip::*;

  let data = vec![1, 2, 3];
 
  data.fold(0, |r, x| r + x);       // 6
 
  data.filter(|&x| x > 1);          // vec![2, 3]
 
  data.map(|x| x + 1);              // vec![2, 3, 4]
 
  data.add(1).distinct();           // vec![1, 2, 3]
 
  data.delete(0).tail();            // vec![3]
  
  data.interleave(vec![4, 5, 6]);   // vec![(1, 4, 2, 5, 3, 6)]
 
  data.grouped_by(|x| x % 2);       // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
```

### Methods

| Method                   | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice |
|--------------------------|:-------------------------:|:-----------------------------:|:-----------------:|:-----:|
| *add*                    |             *             |               *               |         *         |       |
| *all*                    |             *             |               *               |         *         |   *   |
| *any*                    |             *             |               *               |         *         |   *   |
| *chunked*                |             *             |                               |                   |       |
| *chunked_by*             |             *             |                               |                   |       |
| *count_by*               |             *             |               *               |         *         |   *   |
| *cycle*                  |             *             |                               |                   |       |
| *delete*                 |             *             |                               |                   |       |
| *diff*                   |             *             |               *               |         *         |       |
| *distinct*               |             *             |                               |                   |       |
| *distinct_by*            |             *             |                               |                   |       |
| *duplicates*             |             *             |                               |                   |       |
| *duplicates_by*          |             *             |                               |                   |       |
| *enumerate*              |             *             |                               |                   |       |
| *exclude*                |             *             |               *               |         *         |       |
| *fill*                   |             *             |               *               |         *         |       |
| *fill_with*              |             *             |               *               |         *         |       |
| *filter*                 |             *             |               *               |         *         |       |
| *filter_keys*            |                           |                               |         *         |       |
| *filter_map*             |             *             |               *               |         *         |       |
| *filter_map_to*          |             *             |               *               |         *         |       |
| *filter_values*          |                           |                               |         *         |       |
| *find_map*               |             *             |               *               |         *         |       |
| *find_map_to*            |             *             |               *               |         *         |       |
| *find*                   |             *             |               *               |         *         |   *   |
| *flat_map*               |             *             |               *               |         *         |       |
| *flat_map_to*            |             *             |               *               |         *         |       |
| *flat*                   |             *             |               *               |                   |       |
| *fold*                   |             *             |               *               |         *         |   *   |
| *frequencies*            |             *             |                               |                   |       |
| *frequencies_by*         |             *             |                               |                   |       |
| *grouped_by*             |             *             |               *               |                   |       |
| *interleave*             |             *             |                               |                   |       |
| *intersect*              |             *             |               *               |         *         |       |
| *intersperse*            |             *             |                               |                   |       |
| *intersperse_with*       |             *             |                               |                   |       |
| *init*                   |             *             |                               |                   |   *   |
| *join_items*             |             *             |                               |                   |       |
| *largest*                |             *             |               *               |                   |       |
| *map*                    |             *             |               *               |         *         |       |
| *map_to*                 |             *             |               *               |         *         |       |
| *map_keys*               |                           |                               |         *         |       |
| *map_values*             |                           |                               |         *         |       |
| *map_while*              |             *             |                               |                   |       |
| *max_by*                 |             *             |               *               |         *         |   *   |
| *max_item*               |             *             |               *               |         *         |   *   |
| *merge*                  |             *             |               *               |         *         |       |
| *min_by*                 |             *             |               *               |         *         |   *   |
| *min_item*               |             *             |               *               |         *         |   *   |
| *partition*              |             *             |               *               |         *         |       |
| *position*               |             *             |                               |                   |   *   |
| *positions*              |             *             |                               |                   |   *   |
| *product*                |             *             |               *               |                   |       |
| *product_keys*           |                           |                               |         *         |       |
| *product_values*         |                           |                               |         *         |       |
| *put*                    |             *             |                               |                   |       |
| *reduce*                 |             *             |               *               |         *         |   *   |
| *replace*                |             *             |                               |                   |       |
| *rev*                    |             *             |                               |                   |       |
| *rfind*                  |             *             |                               |                   |   *   |
| *rfold*                  |             *             |                               |                   |   *   |
| *rposition*              |             *             |                               |                   |   *   |
| *pad*                    |             *             |                               |                   |   *   |
| *pad_with*               |             *             |                               |                   |   *   |
| *scan*                   |             *             |                               |                   |       |
| *skip*                   |             *             |                               |                   |       |
| *skip_while*             |             *             |                               |                   |   *   |
| *smallest*               |             *             |               *               |                   |       |
| *sorted*                 |             *             |                               |                   |       |
| *sorted_by*              |             *             |                               |                   |       |
| *sorted_by_cached_key*   |             *             |                               |                   |       |
| *sorted_by_key*          |             *             |                               |                   |       |
| *sorted_unstable*        |             *             |                               |                   |       |
| *sorted_unstable_by*     |             *             |                               |                   |       |
| *sorted_unstable_by_key* |             *             |                               |                   |       |
| *step_by*                |             *             |                               |                   |       |
| *sum*                    |             *             |               *               |                   |       |
| *sum_keys*               |                           |                               |         *         |       |
| *sum_values*             |                           |                               |         *         |       |
| *tail*                   |             *             |                               |                   |   *   |
| *take*                   |             *             |                               |                   |       |
| *take_while*             |             *             |                               |                   |   *   |
| *unit*                   |             *             |               *               |         *         |       |
| *unzip*                  |             *             |                               |                   |       |
| *windowed*               |             *             |                               |                   |       |
| *zip*                    |             *             |                               |                   |       |

## Inspiration

- [Rust Collections](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Scala Collections](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/IndexedSeq.html)
- [Haskell Collections](https://hackage.haskell.org/package/collections-api-1.0.0.0/docs/Data-Collections.html)
- [Python Collections](https://python-reference.readthedocs.io/en/latest/docs/list/index.html)
- [Qt Collections](https://doc.qt.io/qt-6/qlist.html)
- [Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)
- [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)

# Tasks

- Extend Rust collections type with methods available for iterators, slices and arrays
  - String

- Implement tests

- Write documentation
