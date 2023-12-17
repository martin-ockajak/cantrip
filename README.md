# Overview

Cantrip adds convenient functional-style methods to existing Rust standard library collections.

## Goals

* Reduce complexity and enhance clarity or Rust code
* Ensure reasonably low and predictable performance cost
* Require minimal learning by mirroring established interfaces

## Features

* Equivalents of suitable iterator methods are added to all standard library collection data types
* Utility methods inspired by other libraries are also added to the same collection data types
* Method names are distinct from current or planned standard library collection method names
* All methods consider collection instances to be immutable although some consume them
* Transformation methods return a new collection instance instead of an iterator
* Everything is available via a single import

## Examples

```rust
  use cantrip::extensions::*;

  let data = vec![0, 1, 2];

  data.map(|x| x + 1);                  // [1, 2, 3]: Vec<i32>

  data.fold(0, |r, x| r + x);           // 3: i32

  data.any(|&x| x == 0);                // true: bool

  data.filter(|&x| x > 0);              // [1, 2]: Vec<i32>

  data.add(0).distinct();               // [0, 1, 2]: Vec<i32>

  data.delete(0).tail();                // [2]: Vec<i32>

  data.group_by(|x| x % 2);             // {0: [0, 2], 1: [1]}: HashMap<i32, Vec<i32>>

  data.partition(|&x| x > 1);           // ([2], [0, 1]): (Vec<i32>, Vec<i32>)

  data.clone().zip(data);               // [(0, 0), (1, 1), (2, 2)]: Vec<(i32, i32)>
```


# Collection Methods

| Method               | Inspiration | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice          |
|----------------------|-------------|---------------------------|-------------------------------|-------------------|----------------|
| *add*                | Python      | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *all*                | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *any*                | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *count_by*           | Scala       | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *delete*             | Python      | :black_circle:            |                               |                   |                |
| *diff*               | Scala       | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *distinct*           | Scala       | :black_circle:            |                               |                   |                |
| *distinct_by*        | Scala       | :black_circle:            |                               |                   |                |
| *enumerate*          | Rust        | :black_circle:            |                               |                   |                |
| *exclude*            | Python      | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *filter*             | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *filter_keys*        | Toolz       |                           |                               | :black_circle:    |                |
| *filter_map*         | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *filter_values*      | Toolz       |                           |                               | :black_circle:    |                |
| *find_map*           | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *find*               | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *flat_map*           | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *flat*               | Rust        | :black_circle:            | :black_circle:                |                   |                |
| *fold*               | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *grouped_by*         | Scala       | :black_circle:            | :black_circle:                |                   |                |
| *interleave*         | Toolz       | :black_circle:            |                               |                   |                |
| *intersect*          | Scala       | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *init*               | Scala       | :black_circle:            |                               |                   | :black_circle: |
| *map*                | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *map_keys*           | Toolz       |                           |                               | :black_circle:    |                |
| *map_values*         | Toolz       |                           |                               | :black_circle:    |                |
| *map_while*          | Rust        | :black_circle:            |                               |                   |                |
| *max_by*             | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *merge*              | Scala       | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *min_by*             | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *partition*          | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *position*           | Rust        | :black_circle:            |                               |                   | :black_circle: |
| *product*            | Rust        | :black_circle:            | :black_circle:                |                   |                |
| *product_keys*       | Rust        |                           |                               | :black_circle:    |                |
| *product_values*     | Rust        |                           |                               | :black_circle:    |                |
| *put*                | Python      | :black_circle:            |                               |                   |                |
| *reduce*             | Rust        | :black_circle:            | :black_circle:                | :black_circle:    | :black_circle: |
| *replace*            | Python      | :black_circle:            |                               |                   |                |
| *rev*                | Rust        | :black_circle:            |                               |                   |                |
| *rfind*              | Rust        | :black_circle:            |                               |                   | :black_circle: |
| *rfold*              | Rust        | :black_circle:            |                               |                   | :black_circle: |
| *rposition*          | Rust        | :black_circle:            |                               |                   | :black_circle: |
| *scan*               | Rust        | :black_circle:            |                               |                   |                |
| *skip*               | Rust        | :black_circle:            |                               |                   |                |
| *skip_while*         | Rust        | :black_circle:            |                               |                   | :black_circle: |
| *sorted*             | Scala       | :black_circle:            |                               |                   |                |
| *sorted_by*          | Scala       | :black_circle:            |                               |                   |                |
| *sorted_unstable*    | Scala       | :black_circle:            |                               |                   |                |
| *sorted_unstable_by* | Scala       | :black_circle:            |                               |                   |                |
| *step_by*            | Rust        | :black_circle:            |                               |                   |                |
| *sum*                | Rust        | :black_circle:            | :black_circle:                |                   |                |
| *sum_keys*           | Rust        |                           |                               | :black_circle:    |                |
| *sum_values*         | Rust        |                           |                               | :black_circle:    |                |
| *tail*               | Scala       | :black_circle:            |                               |                   | :black_circle: |
| *take*               | Rust        | :black_circle:            |                               |                   |                |
| *take_while*         | Rust        | :black_circle:            |                               |                   | :black_circle: |
| *unit*               | Rust        | :black_circle:            | :black_circle:                | :black_circle:    |                |
| *unzip*              | Rust        | :black_circle:            |                               |                   |                |
| *zip*                | Rust        | :black_circle:            |                               |                   |                |


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
  - [Itertools](https://docs.rs/itertools/latest/itertools/index.html)
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
