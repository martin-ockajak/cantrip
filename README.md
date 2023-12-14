# Overview

Practical extensions for standard Rust collections


# API

| Method       | Inspiration | List               | Set                | Map                | Slice              |
|--------------|-------------|--------------------|--------------------|--------------------|--------------------|
| *add*        | Python      | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *all*        | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *any*        | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *delete*     | Python      | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *diff* ?     | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *enumerate*  | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |
| *filter*     | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *filter_map* | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *find_map*   | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *find*       | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *flat_map*   | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *fold*       | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *map*        | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *map_while*  | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |
| *merge* ?    | Cantrip     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *partition*  | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |
| *position*   | Rust        | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| *reduce*     | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *repeat*     | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |
| *rfind*      | Rust        | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| *rfold*      | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| *skip*       | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |
| *take*       | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |
| *unit* ?     | Rust        | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| *zip*        | Rust        | :heavy_check_mark: | :x:                | :x:                | :x:                |

## Data type categories

- List - Vec, VecDeque, LinkedList
- Set - HashSet, BTreeSet
- Map - HashMap, BTreeMap

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
