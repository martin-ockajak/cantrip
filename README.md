# Overview

Practical extensions for standard Rust collections


# API

| Method       | Inspiration | Sequence           | Set                | Map | View               |
|--------------|-------------|--------------------|--------------------|-----|--------------------|
| *add*        | Python      | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *all*        | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :heavy_check_mark: |
| *any*        | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :heavy_check_mark: |
| *delete*     | Python      | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *difference* | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *enumerate*  | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *filter*     | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *filter_map* | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *find_map*   | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *find*       | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :heavy_check_mark: |
| *flat_map*   | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *fold*       | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :heavy_check_mark: |
| *merge*      | Cantrip     | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *partition*  | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *reduce*     | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :heavy_check_mark: |
| *repeat*     | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *rfold*      | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :heavy_check_mark: |
| *map*        | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *map_while*  | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *skip*       | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *take*       | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *unit*       | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |
| *zip*        | Rust        | :heavy_check_mark: | :heavy_check_mark: |     | :x:                |

## Data type categories

- Sequence - Vec, VecDeque, LinkedList
- Set - HashSet, BTreeSet
- Map - HashMap, BTreeMap
- View - Slice, Array

## Methdod sources

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
  - [More Itertools](https://more-itertools.readthedocs.io/en/stable/api.html)
  - [Array Tool](https://github.com/danielpclark/array_tool/tree/master)

# Tasks

- Extend Rust collections type with methods available for iterators, slices and arrays
  - Vec
  - HashSet
  - Slice
  - LinkedList
  - HashMap
  - String


- Additional collection methods


- Additional Scala and Python inspired extension
