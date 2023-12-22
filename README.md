Convenient functional-style methods for existing Rust standard library collections.

### Features

- Equivalents of suitable iterator methods are added to standard library collections
- Additional utility methods commonly found in collection libraries are also included
- Transformation methods return a new collection instance instead of returning an iterator
- All methods consider collection instances to be immutable although some may consume them
- Asymptotic complexity is optimal and performance overhead is limited to new collection creation

### Examples

```rust
  use cantrip::extensions::*;

  let data = vec![0, 1, 2];

  data.map(|x| x + 1);                  // [1, 2, 3]: Vec<i32>
 
  data.fold(0, |r, x| r + x);           // 3: i32
 
  data.any(|&x| x == 0);                // true: bool
 
  data.clone().filter(|&x| x > 0);      // [1, 2]: Vec<i32>
 
  data.clone().add(0).distinct();       // [0, 1, 2]: Vec<i32>
 
  data.clone().delete(0).tail();        // [2]: Vec<i32>
 
  data.clone().grouped_by(|x| x % 2);   // {0: [0, 2], 1: [1]}: HashMap<i32, Vec<i32>>
 
  data.clone().partition(|&x| x > 1);   // ([2], [0, 1]): (Vec<i32>, Vec<i32>)
 
  data.clone().zip(data);               // [(0, 0), (1, 1), (2, 2)]: Vec<(i32, i32)>
```

### Methods

| Method               | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice |
|----------------------|---------------------------|-------------------------------|-------------------|-------|
| *add*                | *                         | *                             | *                 |       |
| *all*                | *                         | *                             | *                 | *     |
| *any*                | *                         | *                             | *                 | *     |
| *chunked*            | *                         |                               |                   |       |
| *count_by*           | *                         | *                             | *                 | *     |
| *delete*             | *                         |                               |                   |       |
| *diff*               | *                         | *                             | *                 |       |
| *distinct*           | *                         |                               |                   |       |
| *distinct_by*        | *                         |                               |                   |       |
| *enumerate*          | *                         |                               |                   |       |
| *exclude*            | *                         | *                             | *                 |       |
| *filter*             | *                         | *                             | *                 |       |
| *filter_keys*        |                           |                               | *                 |       |
| *filter_map*         | *                         | *                             | *                 |       |
| *filter_values*      |                           |                               | *                 |       |
| *find_map*           | *                         | *                             | *                 |       |
| *find*               | *                         | *                             | *                 | *     |
| *flat_map*           | *                         | *                             | *                 |       |
| *flat*               | *                         | *                             |                   |       |
| *fold*               | *                         | *                             | *                 | *     |
| *grouped_by*         | *                         | *                             |                   |       |
| *interleave*         | *                         |                               |                   |       |
| *intersect*          | *                         | *                             | *                 |       |
| *init*               | *                         |                               |                   | *     |
| *map*                | *                         | *                             | *                 |       |
| *map_keys*           |                           |                               | *                 |       |
| *map_values*         |                           |                               | *                 |       |
| *map_while*          | *                         |                               |                   |       |
| *max_by*             | *                         | *                             | *                 | *     |
| *max_entry*          | *                         | *                             | *                 | *     |
| *merge*              | *                         | *                             | *                 |       |
| *min_by*             | *                         | *                             | *                 | *     |
| *min_entry*          | *                         | *                             | *                 | *     |
| *partition*          | *                         | *                             | *                 |       |
| *position*           | *                         |                               |                   | *     |
| *product*            | *                         | *                             |                   |       |
| *product_keys*       |                           |                               | *                 |       |
| *product_values*     |                           |                               | *                 |       |
| *put*                | *                         |                               |                   |       |
| *reduce*             | *                         | *                             | *                 | *     |
| *replace*            | *                         |                               |                   |       |
| *rev*                | *                         |                               |                   |       |
| *rfind*              | *                         |                               |                   | *     |
| *rfold*              | *                         |                               |                   | *     |
| *rposition*          | *                         |                               |                   | *     |
| *scan*               | *                         |                               |                   |       |
| *skip*               | *                         |                               |                   |       |
| *skip_while*         | *                         |                               |                   | *     |
| *sorted*             | *                         |                               |                   |       |
| *sorted_by*          | *                         |                               |                   |       |
| *sorted_unstable*    | *                         |                               |                   |       |
| *sorted_unstable_by* | *                         |                               |                   |       |
| *step_by*            | *                         |                               |                   |       |
| *sum*                | *                         | *                             |                   |       |
| *sum_keys*           |                           |                               | *                 |       |
| *sum_values*         |                           |                               | *                 |       |
| *tail*               | *                         |                               |                   | *     |
| *take*               | *                         |                               |                   |       |
| *take_while*         | *                         |                               |                   | *     |
| *unit*               | *                         | *                             | *                 |       |
| *unzip*              | *                         |                               |                   |       |
| *zip*                | *                         |                               |                   |       |


## Inspirations

- Rust
  - [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  - [Slice](https://doc.rust-lang.org/std/primitive.slice.html)
  - [Array](https://doc.rust-lang.org/std/primitive.array.html)
- Scala
  - [IndexedSeq](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/IndexedSeq.html)
  - [Set](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/Set.html)
  - [Map](https://www.scala-lang.org/api/3.3.1/scala/collection/immutable/Map.html)
  - [String](https://www.scala-lang.org/api/3.3.1/scala/collection/StringOps.html)
- Python
  - [set](https://python-reference.readthedocs.io/en/latest/docs/sets/index.html)
  - [list](https://python-reference.readthedocs.io/en/latest/docs/list/index.html)
  - [dict](https://python-reference.readthedocs.io/en/latest/docs/dict/index.html)
  - [str](https://python-reference.readthedocs.io/en/latest/docs/str/index.html)
- Qt
  - [QList](https://doc.qt.io/qt-6/qlist.html)
  - [QSet](https://doc.qt.io/qt-6/qset.html)
  - [QMap](https://doc.qt.io/qt-6/qmap.html)
  - [QString](https://doc.qt.io/qt-6/qstring.htm)
- Misc
  - [Toolz](https://toolz.readthedocs.io/en/latest/api.html)
  - [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)
  - [Array Tool](https://github.com/danielpclark/array_tool/tree/master)

# Tasks

- Extend Rust collections type with methods available for iterators, slices and arrays
  - VecDeque
  - String

- Additional collection methods

- Implement tests

- Write documentation
