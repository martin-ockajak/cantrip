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
| *add*                | Python      | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *all*                | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *any*                | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *count_by*           | Scala       | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *delete*             | Python      | :white_circle:            |                               |                   |                |
| *diff*               | Scala       | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *distinct*           | Scala       | :white_circle:            |                               |                   |                |
| *distinct_by*        | Scala       | :white_circle:            |                               |                   |                |
| *enumerate*          | Rust        | :white_circle:            |                               |                   |                |
| *exclude*            | Python      | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *filter*             | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *filter_keys*        | Toolz       |                           |                               | :white_circle:    |                |
| *filter_map*         | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *filter_values*      | Toolz       |                           |                               | :white_circle:    |                |
| *find_map*           | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *find*               | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *flat_map*           | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *flat*               | Rust        | :white_circle:            | :white_circle:                |                   |                |
| *fold*               | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *grouped_by*         | Scala       | :white_circle:            | :white_circle:                |                   |                |
| *interleave*         | Toolz       | :white_circle:            |                               |                   |                |
| *intersect*          | Scala       | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *init*               | Scala       | :white_circle:            |                               |                   | :white_circle: |
| *map*                | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *map_keys*           | Toolz       |                           |                               | :white_circle:    |                |
| *map_values*         | Toolz       |                           |                               | :white_circle:    |                |
| *map_while*          | Rust        | :white_circle:            |                               |                   |                |
| *max_by*             | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *merge*              | Scala       | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *min_by*             | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *partition*          | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *position*           | Rust        | :white_circle:            |                               |                   | :white_circle: |
| *product*            | Rust        | :white_circle:            | :white_circle:                |                   |                |
| *product_keys*       | Rust        |                           |                               | :white_circle:    |                |
| *product_values*     | Rust        |                           |                               | :white_circle:    |                |
| *put*                | Python      | :white_circle:            |                               |                   |                |
| *reduce*             | Rust        | :white_circle:            | :white_circle:                | :white_circle:    | :white_circle: |
| *replace*            | Python      | :white_circle:            |                               |                   |                |
| *rev*                | Rust        | :white_circle:            |                               |                   |                |
| *rfind*              | Rust        | :white_circle:            |                               |                   | :white_circle: |
| *rfold*              | Rust        | :white_circle:            |                               |                   | :white_circle: |
| *rposition*          | Rust        | :white_circle:            |                               |                   | :white_circle: |
| *scan*               | Rust        | :white_circle:            |                               |                   |                |
| *skip*               | Rust        | :white_circle:            |                               |                   |                |
| *skip_while*         | Rust        | :white_circle:            |                               |                   | :white_circle: |
| *sorted*             | Scala       | :white_circle:            |                               |                   |                |
| *sorted_by*          | Scala       | :white_circle:            |                               |                   |                |
| *sorted_unstable*    | Scala       | :white_circle:            |                               |                   |                |
| *sorted_unstable_by* | Scala       | :white_circle:            |                               |                   |                |
| *step_by*            | Rust        | :white_circle:            |                               |                   |                |
| *sum*                | Rust        | :white_circle:            | :white_circle:                |                   |                |
| *sum_keys*           | Rust        |                           |                               | :white_circle:    |                |
| *sum_values*         | Rust        |                           |                               | :white_circle:    |                |
| *tail*               | Scala       | :white_circle:            |                               |                   | :white_circle: |
| *take*               | Rust        | :white_circle:            |                               |                   |                |
| *take_while*         | Rust        | :white_circle:            |                               |                   | :white_circle: |
| *unit*               | Rust        | :white_circle:            | :white_circle:                | :white_circle:    |                |
| *unzip*              | Rust        | :white_circle:            |                               |                   |                |
| *zip*                | Rust        | :white_circle:            |                               |                   |                |


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
