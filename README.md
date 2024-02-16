Convenient extension methods for Rust standard library collections.

Enables collection manipulation in a functional style without the usual Rust boilerplate.

### Features

- Equivalents of suitable iterator methods are added to standard library collections
- Additional utility methods commonly found in collection libraries are also included
- Transformation methods return a new collection instance instead of returning an iterator
- All methods consider collection instances to be immutable although some may consume them
- Asymptotic complexity is optimal and performance overhead is limited to new collection creation

### Examples

```rust
use cantrip::*;

# let source = vec![1, 2, 3];
let data = vec![1, 2, 3];

data.fold(0, |r, x| r + x);       // 6

# let data = source.clone();
data.filter(|&x| x > 1);          // vec![2, 3]

# let data = source.clone();
data.map(|x| x + 1);              // vec![2, 3, 4]

# let data = source.clone();
data.add(1).unique();             // vec![1, 2, 3]

# let data = source.clone();
data.delete_at(0).tail();         // vec![3]

# let data = source.clone();
data.interleave(vec![4, 5, 6]);   // vec![(1, 4, 2, 5, 3, 6)]

# let data = source.clone();
data.grouped_by(|x| x % 2);       // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
```

/ ### Methods

| Method                   | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice |
|--------------------------|:-------------------------:|:-----------------------------:|:-----------------:|:-----:|
| *add*                    |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *add_all*                |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *add_all_at*             |             :heavy_check_mark:             |                               |                   |       |
| *add_at*                 |             :heavy_check_mark:             |                               |                   |       |
| *all*                    |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *all_equal*              |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *all_unique*             |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *any*                    |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *chunked*                |             :heavy_check_mark:             |                               |                   |       |
| *chunked_by*             |             :heavy_check_mark:             |                               |                   |       |
| *count_by*               |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *cycle*                  |             :heavy_check_mark:             |                               |                   |       |
| *delete*                 |             :heavy_check_mark:             |                               |                   |       |
| *delete_all*             |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *delete_at*              |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *delete_range*           |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *duplicates*             |             :heavy_check_mark:             |                               |                   |       |
| *duplicates_by*          |             :heavy_check_mark:             |                               |                   |       |
| *enumerate*              |             :heavy_check_mark:             |                               |                   |       |
| *fill*                   |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *fill_with*              |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *filter*                 |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *filter_keys*            |                           |                               |         :heavy_check_mark:         |       |
| *filter_map*             |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *filter_map_to*          |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *filter_values*          |                           |                               |         :heavy_check_mark:         |       |
| *find_map*               |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *find_map_to*            |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *find*                   |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *flat_map*               |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *flat_map_to*            |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *flat*                   |             :heavy_check_mark:             |               :heavy_check_mark:               |                   |       |
| *fold*                   |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *frequencies*            |             :heavy_check_mark:             |                               |                   |       |
| *frequencies_by*         |             :heavy_check_mark:             |                               |                   |       |
| *grouped_by*             |             :heavy_check_mark:             |               :heavy_check_mark:               |                   |       |
| *interleave*             |             :heavy_check_mark:             |                               |                   |       |
| *interleave_shortest*    |             :heavy_check_mark:             |                               |                   |       |
| *intersect*              |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *intersperse*            |             :heavy_check_mark:             |                               |                   |       |
| *intersperse_with*       |             :heavy_check_mark:             |                               |                   |       |
| *init*                   |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *join_items*             |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *largest*                |             :heavy_check_mark:             |               :heavy_check_mark:               |                   |       |
| *map*                    |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *map_to*                 |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *map_keys*               |                           |                               |         :heavy_check_mark:         |       |
| *map_values*             |                           |                               |         :heavy_check_mark:         |       |
| *map_while*              |             :heavy_check_mark:             |                               |                   |       |
| *max_by*                 |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *max_by_key*             |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *max_item*               |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *min_by*                 |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *min_by_key*             |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *min_item*               |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *minmax_by*              |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *minmax_by_key*          |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *minmax_item*            |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *partition*              |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *position*               |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *positions*              |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *product*                |             :heavy_check_mark:             |               :heavy_check_mark:               |                   |       |
| *product_keys*           |                           |                               |         :heavy_check_mark:         |       |
| *product_values*         |                           |                               |         :heavy_check_mark:         |       |
| *reduce*                 |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |   :heavy_check_mark:   |
| *replace*                |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *replace_all*            |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *replace_at*             |             :heavy_check_mark:             |                               |                   |       |
| *replace_range*          |             :heavy_check_mark:             |                               |                   |       |
| *rev*                    |             :heavy_check_mark:             |                               |                   |       |
| *rfind*                  |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *rfold*                  |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *rposition*              |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *rskip*                  |             :heavy_check_mark:             |                               |                   |       |
| *rskip_while*            |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *rtake*                  |             :heavy_check_mark:             |                               |                   |       |
| *rtake_while*            |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *pad*                    |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *pad_with*               |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *scan*                   |             :heavy_check_mark:             |                               |                   |       |
| *skip*                   |             :heavy_check_mark:             |                               |                   |       |
| *skip_while*             |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *smallest*               |             :heavy_check_mark:             |               :heavy_check_mark:               |                   |       |
| *sorted*                 |             :heavy_check_mark:             |                               |                   |       |
| *sorted_by*              |             :heavy_check_mark:             |                               |                   |       |
| *sorted_by_cached_key*   |             :heavy_check_mark:             |                               |                   |       |
| *sorted_by_key*          |             :heavy_check_mark:             |                               |                   |       |
| *sorted_unstable*        |             :heavy_check_mark:             |                               |                   |       |
| *sorted_unstable_by*     |             :heavy_check_mark:             |                               |                   |       |
| *sorted_unstable_by_key* |             :heavy_check_mark:             |                               |                   |       |
| *splice*                 |             :heavy_check_mark:             |                               |                   |       |
| *step_by*                |             :heavy_check_mark:             |                               |                   |       |
| *sum*                    |             :heavy_check_mark:             |               :heavy_check_mark:               |                   |       |
| *sum_keys*               |                           |                               |         :heavy_check_mark:         |       |
| *sum_values*             |                           |                               |         :heavy_check_mark:         |       |
| *tail*                   |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *take*                   |             :heavy_check_mark:             |                               |                   |       |
| *take_while*             |             :heavy_check_mark:             |                               |                   |   :heavy_check_mark:   |
| *unique*                 |             :heavy_check_mark:             |                               |                   |       |
| *unique_by*              |             :heavy_check_mark:             |                               |                   |       |
| *unit*                   |             :heavy_check_mark:             |               :heavy_check_mark:               |         :heavy_check_mark:         |       |
| *unzip*                  |             :heavy_check_mark:             |                               |                   |       |
| *windowed*               |             :heavy_check_mark:             |                               |                   |       |
| *zip*                    |             :heavy_check_mark:             |                               |                   |       |
## Inspiration

- [Rust Collections](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Scala Collections](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/IndexedSeq.html)
- [Haskell Collections](https://hackage.haskell.org/package/collections-api-1.0.0.0/docs/Data-Collections.html)
- [Python Collections](https://python-reference.readthedocs.io/en/latest/docs/list/index.html)
- [Qt Collections](https://doc.qt.io/qt-6/qlist.html)
- [Itertools](https://docs.rs/itertools/latest/itertools/trait.Itertools.html)
- [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)

# Tasks

- Add String extensions

- Implement tests

- Write documentation
