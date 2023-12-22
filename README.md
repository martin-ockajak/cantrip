Convenient functional-style methods for existing Rust standard library collections.

# Goals

- Reduce complexity and enhance clarity or Rust code
- Ensure reasonably low and predictable performance cost
- Require minimal learning by mirroring established interfaces

# Features

- Equivalents of suitable iterator methods are added to all standard library collection data types
- Utility methods inspired by other libraries are also added to the same collection data types
- Method names are distinct from current or planned standard library collection method names
- All methods consider collection instances to be immutable although some consume them
- Transformation methods return a new collection instance instead of an iterator

# Examples

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


# Methods

| Method               | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice          |
|----------------------|---------------------------|-------------------------------|-------------------|----------------|
| *add*                | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *all*                | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *any*                | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *chunked*            | :radio_button:            |                               |                   |                |
| *count_by*           | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *delete*             | :radio_button:            |                               |                   |                |
| *diff*               | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *distinct*           | :radio_button:            |                               |                   |                |
| *distinct_by*        | :radio_button:            |                               |                   |                |
| *enumerate*          | :radio_button:            |                               |                   |                |
| *exclude*            | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *filter*             | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *filter_keys*        |                           |                               | :radio_button:    |                |
| *filter_map*         | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *filter_values*      |                           |                               | :radio_button:    |                |
| *find_map*           | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *find*               | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *flat_map*           | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *flat*               | :radio_button:            | :radio_button:                |                   |                |
| *fold*               | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *grouped_by*         | :radio_button:            | :radio_button:                |                   |                |
| *interleave*         | :radio_button:            |                               |                   |                |
| *intersect*          | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *init*               | :radio_button:            |                               |                   | :radio_button: |
| *map*                | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *map_keys*           |                           |                               | :radio_button:    |                |
| *map_values*         |                           |                               | :radio_button:    |                |
| *map_while*          | :radio_button:            |                               |                   |                |
| *max_by*             | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *max_entry*          | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *merge*              | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *min_by*             | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *min_entry*          | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *partition*          | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *position*           | :radio_button:            |                               |                   | :radio_button: |
| *product*            | :radio_button:            | :radio_button:                |                   |                |
| *product_keys*       |                           |                               | :radio_button:    |                |
| *product_values*     |                           |                               | :radio_button:    |                |
| *put*                | :radio_button:            |                               |                   |                |
| *reduce*             | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *replace*            | :radio_button:            |                               |                   |                |
| *rev*                | :radio_button:            |                               |                   |                |
| *rfind*              | :radio_button:            |                               |                   | :radio_button: |
| *rfold*              | :radio_button:            |                               |                   | :radio_button: |
| *rposition*          | :radio_button:            |                               |                   | :radio_button: |
| *scan*               | :radio_button:            |                               |                   |                |
| *skip*               | :radio_button:            |                               |                   |                |
| *skip_while*         | :radio_button:            |                               |                   | :radio_button: |
| *sorted*             | :radio_button:            |                               |                   |                |
| *sorted_by*          | :radio_button:            |                               |                   |                |
| *sorted_unstable*    | :radio_button:            |                               |                   |                |
| *sorted_unstable_by* | :radio_button:            |                               |                   |                |
| *step_by*            | :radio_button:            |                               |                   |                |
| *sum*                | :radio_button:            | :radio_button:                |                   |                |
| *sum_keys*           |                           |                               | :radio_button:    |                |
| *sum_values*         |                           |                               | :radio_button:    |                |
| *tail*               | :radio_button:            |                               |                   | :radio_button: |
| *take*               | :radio_button:            |                               |                   |                |
| *take_while*         | :radio_button:            |                               |                   | :radio_button: |
| *unit*               | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *unzip*              | :radio_button:            |                               |                   |                |
| *zip*                | :radio_button:            |                               |                   |                |


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
