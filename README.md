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

| Method               | Inspiration | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap  | Slice              |
|----------------------|-------------|---------------------------|-------------------------------|--------------------|--------------------|
| *add*                | Python      | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *all*                | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *any*                | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *count_by*           | Scala       | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *delete*             | Python      | :white_check_mark:        |                               |                    |                    |
| *diff*               | Scala       | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *distinct*           | Scala       | :white_check_mark:        |                               |                    |                    |
| *distinct_by*        | Scala       | :white_check_mark:        |                               |                    |                    |
| *enumerate*          | Rust        | :white_check_mark:        |                               |                    |                    |
| *exclude*            | Python      | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *filter*             | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *filter_keys*        | Toolz       |                           |                               | :white_check_mark: |                    |
| *filter_map*         | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *filter_values*      | Toolz       |                           |                               | :white_check_mark: |                    |
| *find_map*           | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *find*               | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *flat_map*           | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *flat*               | Rust        | :white_check_mark:        | :white_check_mark:            |                    |                    |
| *fold*               | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *grouped_by*         | Scala       | :white_check_mark:        | :white_check_mark:            |                    |                    |
| *interleave*         | Toolz       | :white_check_mark:        |                               |                    |                    |
| *intersect*          | Scala       | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *init*               | Scala       | :white_check_mark:        |                               |                    | :white_check_mark: |
| *map*                | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *map_keys*           | Toolz       |                           |                               | :white_check_mark: |                    |
| *map_values*         | Toolz       |                           |                               | :white_check_mark: |                    |
| *map_while*          | Rust        | :white_check_mark:        |                               |                    |                    |
| *max_by*             | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *merge*              | Scala       | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *min_by*             | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *partition*          | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *position*           | Rust        | :white_check_mark:        |                               |                    | :white_check_mark: |
| *product*            | Rust        | :white_check_mark:        | :white_check_mark:            |                    |                    |
| *product_keys*       | Rust        |                           |                               | :white_check_mark: |                    |
| *product_values*     | Rust        |                           |                               | :white_check_mark: |                    |
| *put*                | Python      | :white_check_mark:        |                               |                    |                    |
| *reduce*             | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: | :white_check_mark: |
| *replace*            | Python      | :white_check_mark:        |                               |                    |                    |
| *rev*                | Rust        | :white_check_mark:        |                               |                    |                    |
| *rfind*              | Rust        | :white_check_mark:        |                               |                    | :white_check_mark: |
| *rfold*              | Rust        | :white_check_mark:        |                               |                    | :white_check_mark: |
| *rposition*          | Rust        | :white_check_mark:        |                               |                    | :white_check_mark: |
| *scan*               | Rust        | :white_check_mark:        |                               |                    |                    |
| *skip*               | Rust        | :white_check_mark:        |                               |                    |                    |
| *skip_while*         | Rust        | :white_check_mark:        |                               |                    | :white_check_mark: |
| *sorted*             | Scala       | :white_check_mark:        |                               |                    |                    |
| *sorted_by*          | Scala       | :white_check_mark:        |                               |                    |                    |
| *sorted_unstable*    | Scala       | :white_check_mark:        |                               |                    |                    |
| *sorted_unstable_by* | Scala       | :white_check_mark:        |                               |                    |                    |
| *step_by*            | Rust        | :white_check_mark:        |                               |                    |                    |
| *sum*                | Rust        | :white_check_mark:        | :white_check_mark:            |                    |                    |
| *sum_keys*           | Rust        |                           |                               | :white_check_mark: |                    |
| *sum_values*         | Rust        |                           |                               | :white_check_mark: |                    |
| *tail*               | Scala       | :white_check_mark:        |                               |                    | :white_check_mark: |
| *take*               | Rust        | :white_check_mark:        |                               |                    |                    |
| *take_while*         | Rust        | :white_check_mark:        |                               |                    | :white_check_mark: |
| *unit*               | Rust        | :white_check_mark:        | :white_check_mark:            | :white_check_mark: |                    |
| *unzip*              | Rust        | :white_check_mark:        |                               |                    |                    |
| *zip*                | Rust        | :white_check_mark:        |                               |                    |                    |


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
