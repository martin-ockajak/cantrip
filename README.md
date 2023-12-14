# Overview

Practical extensions for standard Rust collections.

# Example

```rust
  use cantrip::extensions::*;

  let data = vec![0, 1, 2];
  
  data.map(|x| x + 1);                     // [1, 2, 3]: Vec<i32>

  data.fold(0, |r, x| r + x);              // 3: i32

  data.any(|&x| x == 0);                   // true: bool

  data.head();                             // Some(0): Option<i32>

  data.clone().filter(|&x| x > 0);         // [1, 2]: Vec<i32>

  data.clone().add(0).unique();            // [0, 1, 2]: Vec<i32>

  data.clone().delete(&0).tail();          // [2]: Vec<i32>

  data.clone().partition(|&x| x > 1);      // ([2], [0, 1]): (Vec<i32>, Vec<i32>)

  data.clone().zip(data);                  // [(0, 0), (1, 1), (2, 2)]: Vec<(i32, i32)>
```


# API

| Method          | Inspiration | Vec, VecDeque, LinkedList | HashSet, BTreeSet  | HashMap, BTreeMap  | Slice              |
|-----------------|-------------|---------------------------|--------------------|--------------------|--------------------|
| *add*           | Python      | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *all*           | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *any*           | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *concat*        | Scala       | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *delete*        | Python      | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *diff*          | Scala       | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *distinct*      | Scala       | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *enumerate*     | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *filter*        | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *filter_keys*   | Toolz       | :x:                       | :x:                | :heavy_check_mark: | :x:                |
| *filter_map*    | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *filter_values* | Toolz       | :x:                       | :x:                | :heavy_check_mark: | :x:                |
| *find_map*      | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *find*          | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *flat_map*      | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *fold*          | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *head*          | Scala       | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *intersect*     | Scala       | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *init*          | Scala       | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *last*          | Scala       | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *map*           | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *map_keys*      | Toolz       | :x:                       | :x:                | :heavy_check_mark: | :x:                |
| *map_values*    | Toolz       | :x:                       | :x:                | :heavy_check_mark: | :x:                |
| *map_while*     | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *partition*     | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *position*      | Rust        | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *reduce*        | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *rev*           | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *rfind*         | Rust        | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *rfold*         | Rust        | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *rposition*     | Rust        | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *skip*          | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *tail*          | Scala       | :heavy_check_mark:        | :x:                | :x:                | :heavy_check_mark: |
| *take*          | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |
| *unit* ?        | Rust        | :heavy_check_mark:        | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *zip*           | Rust        | :heavy_check_mark:        | :x:                | :x:                | :x:                |


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
  - Vec
  - VecDeque
  - LinkedList
  - HashSet
  - BTreeSet
  - HashMap
  - BTreeMap
  - Slice
  - String


- Additional collection methods


- Additional Scala and Python inspired extension
