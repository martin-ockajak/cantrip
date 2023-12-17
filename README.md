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
| *add*                | Python      | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *all*                | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *any*                | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *divide*             | Rust        | :radio_button:            |                               |                   |                |
| *count_by*           | Scala       | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *delete*             | Python      | :radio_button:            |                               |                   |                |
| *diff*               | Scala       | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *distinct*           | Scala       | :radio_button:            |                               |                   |                |
| *distinct_by*        | Scala       | :radio_button:            |                               |                   |                |
| *enumerate*          | Rust        | :radio_button:            |                               |                   |                |
| *exclude*            | Python      | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *filter*             | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *filter_keys*        | Toolz       |                           |                               | :radio_button:    |                |
| *filter_map*         | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *filter_values*      | Toolz       |                           |                               | :radio_button:    |                |
| *find_map*           | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *find*               | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *flat_map*           | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *flat*               | Rust        | :radio_button:            | :radio_button:                |                   |                |
| *fold*               | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *grouped_by*         | Scala       | :radio_button:            | :radio_button:                |                   |                |
| *interleave*         | Toolz       | :radio_button:            |                               |                   |                |
| *intersect*          | Scala       | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *init*               | Scala       | :radio_button:            |                               |                   | :radio_button: |
| *map*                | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *map_keys*           | Toolz       |                           |                               | :radio_button:    |                |
| *map_values*         | Toolz       |                           |                               | :radio_button:    |                |
| *map_while*          | Rust        | :radio_button:            |                               |                   |                |
| *max_by*             | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *merge*              | Toolz       | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *min_by*             | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *partition*          | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *position*           | Rust        | :radio_button:            |                               |                   | :radio_button: |
| *product*            | Rust        | :radio_button:            | :radio_button:                |                   |                |
| *product_keys*       | Rust        |                           |                               | :radio_button:    |                |
| *product_values*     | Rust        |                           |                               | :radio_button:    |                |
| *put*                | Python      | :radio_button:            |                               |                   |                |
| *reduce*             | Rust        | :radio_button:            | :radio_button:                | :radio_button:    | :radio_button: |
| *replace*            | Python      | :radio_button:            |                               |                   |                |
| *rev*                | Rust        | :radio_button:            |                               |                   |                |
| *rfind*              | Rust        | :radio_button:            |                               |                   | :radio_button: |
| *rfold*              | Rust        | :radio_button:            |                               |                   | :radio_button: |
| *rposition*          | Rust        | :radio_button:            |                               |                   | :radio_button: |
| *scan*               | Rust        | :radio_button:            |                               |                   |                |
| *skip*               | Rust        | :radio_button:            |                               |                   |                |
| *skip_while*         | Rust        | :radio_button:            |                               |                   | :radio_button: |
| *sorted*             | Scala       | :radio_button:            |                               |                   |                |
| *sorted_by*          | Scala       | :radio_button:            |                               |                   |                |
| *sorted_unstable*    | Scala       | :radio_button:            |                               |                   |                |
| *sorted_unstable_by* | Scala       | :radio_button:            |                               |                   |                |
| *step_by*            | Rust        | :radio_button:            |                               |                   |                |
| *sum*                | Rust        | :radio_button:            | :radio_button:                |                   |                |
| *sum_keys*           | Rust        |                           |                               | :radio_button:    |                |
| *sum_values*         | Rust        |                           |                               | :radio_button:    |                |
| *tail*               | Scala       | :radio_button:            |                               |                   | :radio_button: |
| *take*               | Rust        | :radio_button:            |                               |                   |                |
| *take_while*         | Rust        | :radio_button:            |                               |                   | :radio_button: |
| *unit*               | Rust        | :radio_button:            | :radio_button:                | :radio_button:    |                |
| *unzip*              | Rust        | :radio_button:            |                               |                   |                |
| *zip*                | Rust        | :radio_button:            |                               |                   |                |


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
