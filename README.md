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
  use cantrip::extensions::*;

  let data = vec![1, 2, 3];
 
  data.fold(0, |r, x| r + x);     // 6
 
  data.any(|&x| x == 1);          // true
 
  data.map(|x| x + 1);            // vec![2, 3, 4]
 
  data.filter(|&x| x > 1);        // vec![2, 3]
 
  data.add(1).distinct();         // vec![1, 2, 3]
 
  data.delete(0).tail();          // vec![3]
 
  data.grouped_by(|x| x % 2);     // HashMap::from(vec![(0, vec![2]), (1, vec![1, 3])])
 
  data.partition(|&x| x > 1);     // (vec![3], vec![1, 2])
 
  data.clone().zip(data);         // vec![(1, 1), (2, 2), (3, 3)]
```

### Methods

| Method               | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice |
|----------------------|:-------------------------:|:-----------------------------:|:-----------------:|:-----:|
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

- Add methods not existing in Rust to initial examples

- Replace argument in the zip example with another collection

- Add consuming versions of certain methods with the suffix '_to'

- Add `chunked_by` method
  - https://apidock.com/ruby/v2_6_3/Enumerable/chunk
  - https://apidock.com/rails/v2.3.8/ActiveSupport/CoreExtensions/Array/Grouping/in_groups
  - https://apidock.com/rails/browse - CoreExtensions

- Extend Rust collections type with methods available for iterators, slices and arrays
  - String

- Add global import

- Additional collection methods

- List libraries serving as an inspiration

- Implement tests

- Write documentation
