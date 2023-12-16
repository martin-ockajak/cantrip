# Overview

Cantrip adds convenient functional-style methods to existing Rust standard library collections.

## Goals

* Reduce complexity and enhance clarity or Rust code
* Ensure reasonably low and predictable performance cost
* Require minimal learning by mirroring established interfaces

## Features

* Equivalents of suitable iterator methods are added to all standard library collection data types
* Utility methods inspired by other libraries are also added to the same collection data types
* All methods treat collection instances as immutable although some consume them
* Transformation methods return a new instance of the same collection type
* No methods perform cloning nor do they use dynamic dispatch
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
| *add*                | Python      | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *all*                | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *any*                | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *concat*             | Scala       | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *count_by*           | Scala       | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *delete*             | Python      | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *diff*               | Scala       | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *distinct*           | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *distinct_by*        | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *enumerate*          | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *exclude*            | Python      | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *filter*             | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *filter_keys*        | Toolz       | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *filter_map*         | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *filter_values*      | Toolz       | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *find_map*           | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *find*               | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *flat_map*           | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *flatten*            | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :x:                | :x:                |
| *fold*               | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *group_by*           | Scala       | :heavy_check_mark:        | :heavy_check_mark:            | :x:                | :x:                |
| *interleave*         | Toolz       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *intersect*          | Scala       | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *init*               | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *map*                | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *map_keys*           | Toolz       | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *map_values*         | Toolz       | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *map_while*          | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *max_by*             | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *min_by*             | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *partition*          | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *position*           | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *product*            | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :x:                | :x:                |
| *product_keys*       | Rust        | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *product_values*     | Rust        | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *put*                | Python      | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *reduce*             | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :heavy_check_mark: |
| *replace*            | Python      | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *rev*                | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *rfind*              | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *rfold*              | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *rposition*          | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *scan*               | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *skip*               | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *skip_while*         | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *sorted*             | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *sorted_by*          | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *sorted_unstable*    | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *sorted_unstable_by* | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *step_by*            | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *sum*                | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :x:                | :x:                |
| *sum_keys*           | Rust        | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *sum_values*         | Rust        | :x:                       | :x:                           | :heavy_check_mark: | :x:                |
| *tail*               | Scala       | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *take*               | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *take_while*         | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :heavy_check_mark: |
| *unit*               | Rust        | :heavy_check_mark:        | :heavy_check_mark:            | :heavy_check_mark: | :x:                |
| *unzip*              | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |
| *zip*                | Rust        | :heavy_check_mark:        | :x:                           | :x:                | :x:                |


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
  - LinkedList
  - BinaryHeap
  - BTreeSet
  - BTreeMap
  - String

- Additional collection methods

- Additional Scala and Python inspired extension

- Implement tests

- Write documentation
