use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use crate::Iterable;

/// Map operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a map
/// - May consume the map and its entries
/// - May create a new map
///
pub trait Map<Key, Value> {
  /// This map type constructor
  type This<K, V>;

  /// Creates a new map by adding an entry to the original map.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let a_source = HashMap::from([
  /// #  (1, 1),
  /// #  (2, 2),
  /// #  (3, 3),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.add(4, 4), HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  ///   (4, 4),
  /// ]));
  /// # let a = a_source.clone();
  /// assert_eq!(a.add(1, 4), HashMap::from([
  ///   (1, 4),
  ///   (2, 2),
  ///   (3, 3),
  /// ]));
  /// ```
  #[inline]
  fn add(self, key: Key, value: Value) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iter::once((key, value))).collect()
  }

  /// Creates a new map by appending all entries from another collection to
  /// the original map.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let a_source = HashMap::from([
  /// #  (1, 1),
  /// #  (2, 2),
  /// #  (3, 3),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.add_multi(vec![(4, 4), (5, 5)]), HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  ///   (4, 4),
  ///   (5, 5),
  /// ]));
  /// # let a = a_source.clone();
  /// assert_eq!(a.add_multi(vec![(1, 4), (5, 5)]), HashMap::from([
  ///   (1, 4),
  ///   (2, 2),
  ///   (3, 3),
  ///   (5, 5),
  /// ]));
  /// ```
  #[inline]
  fn add_multi(self, entries: impl IntoIterator<Item = (Key, Value)>) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(entries).collect()
  }

  /// Tests if every entry of the map matches a predicate.
  ///
  /// `all()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each entry of the map, and if they all return
  /// `true`, then so does `all()`. If any of them return `false`, it
  /// returns `false`.
  ///
  /// `all()` is short-circuiting; in other words, it will stop processing
  /// as soon as it finds a `false`, given that no matter what else happens,
  /// the result will also be `false`.
  ///
  /// An empty map returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert!(a.all(|(&k, &v)| k > 0 && v > 0));
  /// assert!(e.all(|(&k, _)| k > 0));
  ///
  /// assert!(!a.all(|(&k, _)| k > 2));
  /// ```
  fn all(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  /// Tests if any entry of the map matches a predicate.
  ///
  /// `any()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each entry of the map, and if any of them return
  /// `true`, then so does `any()`. If they all return `false`, it
  /// returns `false`.
  ///
  /// `any()` is short-circuiting; in other words, it will stop processing
  /// as soon as it finds a `true`, given that no matter what else happens,
  /// the result will also be `true`.
  ///
  /// An empty map returns `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert!(a.any(|(&k, &v)| k > 0 && v > 1));
  ///
  /// assert!(!a.any(|(&k, _)| k > 5));
  /// assert!(!e.any(|(&k, _)| k > 0));
  /// ```
  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

  /// Transforms this map into specified collection type.
  ///
  /// `collect()` can take any map, and turn it into a relevant
  /// collection. This can be used in a variety of contexts.
  ///
  /// Because `collect()` is so general, it can cause problems with type
  /// inference. As such, `collect()` is one of the few times you'll see
  /// the syntax affectionately known as the 'turbofish': `::<>`. This
  /// helps the inference algorithm understand specifically which collection
  /// you're trying to collect into.
  ///
  /// This is a non-consuming variant of [`collect_to()`].
  ///
  /// [`collect_to()`]: Map::collect_to
  ///
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{HashMap, HashSet};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let collected: HashSet<(i32, i32)> = a.collect();
  ///
  /// assert_eq!(collected, HashSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// Note that we needed the `::HashSet<i32>` on the left-hand side. This is because
  /// we could collect into, for example, a [`BTreeSet<T>`] instead:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let collected: BTreeSet<(i32, i32)> = a.collect();
  ///
  /// assert_eq!(collected, BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// Using the 'turbofish' instead of annotating `collected`:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.collect::<BTreeSet<(i32, i32)>>(), BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// Because `collect()` only cares about what you're collecting into, you can
  /// still use a partial type hint, `_`, with the turbofish:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.collect::<BTreeSet<_>>(), BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// [`BTreeSet<T>`]: ../../std/collections/struct.BTreeSet.html
  fn collect<B>(&self) -> B
  where
    Key: Clone,
    Value: Clone,
    B: FromIterator<(Key, Value)>;

  /// Transforms this map into specified collection type.
  ///
  /// `collect_to()` can take any map, and turn it into a relevant
  /// collection. This can be used in a variety of contexts.
  ///
  /// Because `collect_to()` is so general, it can cause problems with type
  /// inference. As such, `collect_to()` is one of the few times you'll see
  /// the syntax affectionately known as the 'turbofish': `::<>`. This
  /// helps the inference algorithm understand specifically which collection
  /// you're trying to collect_to into.
  ///
  /// This is a consuming variant of [`collect()`].
  ///
  /// [`collect()`]: Map::collect
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{HashMap, HashSet};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let collect_toed: HashSet<(i32, i32)> = a.collect_to();
  ///
  /// assert_eq!(collect_toed, HashSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// Note that we needed the `::HashSet<i32>` on the left-hand side. This is because
  /// we could collect_to into, for example, a [`BTreeSet<T>`] instead:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let collect_toed: BTreeSet<(i32, i32)> = a.collect_to();
  ///
  /// assert_eq!(collect_toed, BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// Using the 'turbofish' instead of annotating `collect_toed`:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.collect_to::<BTreeSet<(i32, i32)>>(), BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// Because `collect_to()` only cares about what you're collecting into, you can
  /// still use a partial type hint, `_`, with the turbofish:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.collect_to::<BTreeSet<_>>(), BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  ///
  /// [`BTreeSet<T>`]: ../../std/collections/struct.BTreeSet.html
  #[inline]
  fn collect_to<B>(self) -> B
  where
    B: FromIterator<(Key, Value)>,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().collect()
  }

  /// Counts entries of this map that satisfy a predicate.
  ///
  /// `count_by()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each entry of the map, and counts those which
  /// return `true`, disregarding those which return `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.count_by(|(&k, &v)| k == 2 && v == 2), 1);
  /// assert_eq!(a.count_by(|(&k, _)| k == 5), 0);
  /// ```
  fn count_by(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> usize;

  /// Counts number of unique elements in this map.
  ///
  /// Returns `0` for an empty map.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let b = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 1),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.count_unique(), 3);
  /// assert_eq!(b.count_unique(), 2);
  ///
  /// assert_eq!(e.count_unique(), 0);
  /// ```
  fn count_unique(&self) -> usize
  where
    Value: Eq + Hash;

  /// Creates a new map from the original map without
  /// the entry specified by a key.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.delete(&2), HashMap::from([
  ///   (1, 1),
  ///   (3, 3),
  /// ]));
  /// assert_eq!(e.delete(&2), HashMap::new());
  /// ```
  #[inline]
  fn delete(self, key: &Key) -> Self
  where
    Key: PartialEq,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter_map(|(k, v)| if &k != key { Some((k, v)) } else { None }).collect()
  }

  /// Creates a new map from the original map without
  /// the entries specified by keys found in another collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// assert_eq!(a.delete_multi(&vec![1, 3]), HashMap::from([
  ///   (2, 2),
  /// ]));
  ///
  /// assert_eq!(e.delete_multi(&vec![1]), HashMap::new());
  /// ```
  #[inline]
  fn delete_multi<'a>(self, keys: &'a impl Iterable<Item<'a> = &'a Key>) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let removed: HashSet<&Key> = HashSet::from_iter(keys.iterator());
    self.into_iter().filter(|(k, _)| !removed.contains(k)).collect()
  }

  /// Tests if keys of this map and another collection have no elements in common.
  ///
  /// Returns `true` if aby of the collections are empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert!(a.disjoint(&vec![4, 5]));
  /// assert!(a.disjoint(&vec![]));
  ///
  /// assert!(!a.disjoint(&vec![3, 4]));
  /// ```
  fn disjoint<'a>(&'a self, keys: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

  /// Creates a new map containing a result of a function
  /// specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// assert_eq!(HashMap::fill_with(|| (1, 1), 1), HashMap::from([
  ///   (1, 1),
  /// ]));
  ///
  /// assert_eq!(HashMap::fill_with(|| (1, 1), 0), HashMap::new());
  /// ```
  #[inline]
  fn fill_with(mut value: impl FnMut() -> (Key, Value), size: usize) -> Self
  where
    Key: Clone,
    Value: Clone,
    Self: FromIterator<(Key, Value)>,
  {
    iter::repeat(value()).take(size).collect()
  }

  /// Creates a new map by filtering the original map using a
  /// closure to determine if an entry should be retained.
  ///
  /// Given an entry the closure must return `true` or `false`. The returned
  /// map will contain only the entries for which the closure returns true.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter(|(&k, &v)| k != 2 && v != 2),
  ///   HashMap::from([
  ///     (1, 1),
  ///     (3, 3),
  /// ]));
  /// ```
  #[inline]
  fn filter(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(k, v)| predicate((k, v))).collect()
  }

  /// Creates a new map by filtering the original map using a
  /// closure to determine if a key should be retained.
  ///
  /// Given an entry the closure must return `true` or `false`. The returned
  /// map will contain only the entries for which the closure returns
  /// true.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter_keys(|&k| k != 2),
  ///   HashMap::from([
  ///     (1, 1),
  ///     (3, 3),
  /// ]));
  /// ```
  #[inline]
  fn filter_keys(self, mut predicate: impl FnMut(&Key) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(k, _)| predicate(k)).collect()
  }

  /// Creates a new map by filtering the original map using a
  /// closure to determine if a value should be retained.
  ///
  /// Given an entry the closure must return `true` or `false`. The returned
  /// map will contain only the entries for which the closure returns
  /// true.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter_values(|&v| v != 2),
  ///   HashMap::from([
  ///     (1, 1),
  ///     (3, 3),
  /// ]));
  /// ```
  #[inline]
  fn filter_values(self, mut predicate: impl FnMut(&Value) -> bool) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().filter(|(_, v)| predicate(v)).collect()
  }

  /// Creates a new map by filtering and mapping the original map.
  ///
  /// The returned map contains only the `entries` for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map()` can be used to make chains of [`filter()`] and [`map()`] more
  /// concise. The example below shows how a `map().filter().map()` can be
  /// shortened to a single call to `filter_map()`.
  ///
  /// This is a consuming variant of [`filter_map_ref()`].
  ///
  /// [`filter()`]: Map::filter
  /// [`map()`]: Map::map_ref
  /// [`filter_map_ref()`]: Map::filter_map_ref
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter_map(|(k, v)| if k < 2 { Some((k, v + 1)) } else { None }),
  ///   HashMap::from([
  ///     (1, 2),
  /// ]));
  /// ```
  ///
  /// Here's the same example, but with [`filter()`] and [`map()`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter(|(&k, _)| k < 2).map(|(k, v)| (k, v + 1)),
  ///   HashMap::from([
  ///     (1, 2),
  /// ]));
  /// ```
  #[inline]
  fn filter_map<L, W>(self, function: impl FnMut((Key, Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().filter_map(function).collect()
  }

  /// Creates a new map by filtering and mapping the original map.
  ///
  /// The returned map contains only the `entries` for which the supplied
  /// closure returns `Some(entry)`.
  ///
  /// `filter_map_ref()` can be used to make chains of [`filter()`] and [`map_ref()`] more
  /// concise. The example below shows how a `filter().map_ref()` can be
  /// shortened to a single call to `filter_map()`.
  ///
  /// This is a non-consuming variant of [`filter_map_to()`].
  ///
  /// [`filter()`]: Map::filter
  /// [`map_ref()`]: Map::map_ref
  /// [`filter_map_to()`]: Map::filter_map
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter_map_ref(|(&k, &v)| if k < 2 { Some((k, v + 1)) } else { None }),
  ///   HashMap::from([
  ///     (1, 2),
  /// ]));
  /// ```
  ///
  /// Here's the same example, but with [`filter()`] and [`map_ref()`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.filter(|(&k, _)| k < 2).map_ref(|(&k, &v)| (k, v + 1)),
  ///   HashMap::from([
  ///     (1, 2),
  /// ]));
  /// ```
  fn filter_map_ref<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  /// Searches for an entry of this map that satisfies a predicate.
  ///
  /// `find()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each entry of the map, and if any of them return
  /// `true`, then `find()` returns [`Some(entry)`]. If they all return
  /// `false`, it returns [`None`].
  ///
  /// `find()` is short-circuiting; in other words, it will stop processing
  /// as soon as the closure returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.find(|(&k, &v)| k == 2 && v == 2), Some((&2, &2)));
  ///
  /// assert_eq!(a.find(|(&k, _)| k == 5), None);
  /// ```
  fn find(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)>;

  /// Applies function to the entries of this map and returns
  /// the first non-none result.
  ///
  /// `find_map()` can be used to make chains of [`find()`] and [`map_ref()`] more
  /// concise.
  ///
  /// `find_map_ref(f)` is equivalent to `find().map_ref()`.
  ///
  /// This is a non-consuming variant of [`find_map()`].
  ///
  /// [`find()`]: Map::find
  /// [`map_ref()`]: Map::map_ref
  /// [`find_map()`]: Map::find_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.find_map_ref(|(&k, &v)| if k == 2 { Some(v) } else { None }),
  ///   Some(2)
  /// );
  /// ```
  fn find_map_ref<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  /// Applies function to the entries of this map and returns
  /// the first non-none result.
  ///
  /// `find_map()` can be used to make chains of [`find()`] and [`map()`] more concise.
  ///
  /// `find_map(f)` is equivalent to `find().map()`.
  ///
  /// This is a consuming variant of [`find_map()`].
  ///
  /// [`find()`]: Map::find
  /// [`map()`]: Map::map_ref
  /// [`find_map()`]: Map::find_map_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.find_map(|(k, v)| if k == 2 { Some(v) } else { None }),
  ///   Some(2)
  /// );
  /// ```
  #[inline]
  fn find_map<B>(self, function: impl FnMut((Key, Value)) -> Option<B>) -> Option<B>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().find_map(function)
  }

  /// Creates a new map by applying the given closure `function` to each entry
  /// of the original map and flattens the nested map.
  ///
  /// The `flat_map()` method is very useful, but only when the closure
  /// argument produces values. If it produces an iterable value instead, there's
  /// an extra layer of indirection. `flat_map()` will remove this extra layer
  /// on its own.
  ///
  /// You can think of `flat_map(f)` as the semantic equivalent
  /// of mapping, and then flattening as in [`map(f)']['.flat()`].
  ///
  /// Another way of thinking about `flat_map()`: [`map()`]'s closure returns
  /// one item for each entry, and `flat_map()`'s closure returns an
  /// iterable value for each entry.
  ///
  /// This is a consuming variant of [`flat_map_ref()`].
  ///
  /// [`map()`]: Map::map_ref
  /// [`map(f)`]: Map::map_ref
  /// [`.flat()`]: crate::CollectionTo::flat
  /// [`flat_map_ref()`]: Map::flat_map_ref
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// // Vec is iterable because it implements IntoIterator
  /// assert_eq!(
  ///   a.flat_map(|(k, v)| vec![(-k, v), (k, v)]),
  ///   HashMap::from([
  ///     (-1, 1),
  ///     (-2, 2),
  ///     (-3, 3),
  ///     (1, 1),
  ///     (2, 2),
  ///     (3, 3),
  /// ]));
  /// ```
  #[inline]
  fn flat_map<L, W, R>(self, function: impl FnMut((Key, Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().flat_map(function).collect()
  }

  /// Creates a new map by applying the given closure `function` to each entry
  /// of the original map and flattens the nested map.
  ///
  /// The `flat_map_ref()` method is very useful, but only when the closure
  /// argument produces values. If it produces an iterable value instead, there's
  /// an extra layer of indirection. `flat_map_ref()` will remove this extra layer
  /// on its own.
  ///
  /// You can think of `flat_map_ref(f)` as the semantic equivalent
  /// of mapping, and then flattening as in [`map_ref(f)']['.flat()`].
  ///
  /// Another way of thinking about `flat_map_ref()`: [`map_ref()`]'s closure returns
  /// one item for each entry, and `flat_map_ref()`'s closure returns an
  /// iterable value for each entry.
  ///
  /// This is a non-consuming variant of [`flat_map()`].
  ///
  /// [`map_ref()`]: Map::map_ref
  /// [`map_ref(f)`]: Map::map_ref
  /// [`.flat()`]: crate::CollectionTo::flat
  /// [`flat_map()`]: Map::flat_map
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// // Vec is iterable because it implements IntoIterator
  /// assert_eq!(
  ///   a.flat_map_ref(|(&k, &v)| vec![(-k, v), (k, v)]),
  ///   HashMap::from([
  ///     (-1, 1),
  ///     (-2, 2),
  ///     (-3, 3),
  ///     (1, 1),
  ///     (2, 2),
  ///     (3, 3),
  /// ]));
  /// ```
  fn flat_map_ref<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>;

  /// Folds every entry into an accumulator by applying an operation,
  /// returning the final result.
  ///
  /// `fold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an entry. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every entry of this map, `fold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a map of something, and want
  /// to produce a single value from it.
  ///
  /// This is a consuming variant of [`fold()`].
  ///
  /// Note: [`reduce()`] can be used to use the first entry as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold()` combines entries in a *left-associative* fashion. For associative
  /// operators like `+`, the order the entries are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  ///
  /// [`fold()`]: Map::fold_ref
  /// [`reduce()`]: Map::reduce_ref
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// // the sum of all the elements of the array
  /// assert_eq!(
  ///   a.fold(0, |acc, (k, v)| acc + k + v),
  ///   12
  /// );
  /// ```
  ///
  /// Let's walk through each step of the iteration here:
  ///
  /// | element | acc | k | k | result |
  /// |---------|-----|---|---|--------|
  /// |         | 0   |   |   |        |
  /// | 1       | 0   | 1 | a | 2      |
  /// | 2       | 2   | 2 | b | 5      |
  /// | 3       | 5   | 3 | c | 9      |
  ///
  /// And so, our final result, `9`.
  fn fold<B>(self, initial_value: B, function: impl FnMut(B, (Key, Value)) -> B) -> B
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().fold(initial_value, function)
  }

  /// Folds every entry into an accumulator by applying an operation,
  /// returning the final result.
  ///
  /// `fold_ref()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an entry. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every entry of the map, `fold_ref()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a map of something, and want
  /// to produce a single value from it.
  ///
  /// This is a non-consuming variant of [`fold()`].
  ///
  /// Note: [`reduce_ref()`] can be used to use the first entry as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold_ref()` combines entries in a *left-associative* fashion. For associative
  /// operators like `+`, the order the entries are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  ///
  /// [`fold()`]: Map::fold
  /// [`reduce_ref()`]: Map::reduce_ref
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// // the sum of all the elements of the array
  /// assert_eq!(
  ///   a.fold_ref(0, |acc, (&k, &v)| acc + k + v),
  ///   12
  /// );
  /// ```
  ///
  /// Let's walk through each step of the iteration here:
  ///
  /// | element | acc | k | k | result |
  /// |---------|-----|---|---|--------|
  /// |         | 0   |   |   |        |
  /// | 1       | 0   | 1 | a | 2      |
  /// | 2       | 2   | 2 | b | 5      |
  /// | 3       | 5   | 3 | c | 9      |
  ///
  /// And so, our final result, `9`.
  fn fold_ref<B>(&self, initial_value: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B;

  /// Calls a closure on each entry of this map.
  ///
  /// This is equivalent to using a [`for`] loop on the map, although
  /// `break` and `continue` are not possible from a closure. It's generally
  /// more idiomatic to use a `for` loop, but `for_each` may be more legible
  /// when processing items at the end of longer map chains.
  ///
  /// [`for`]: ../../book/ch03-05-control-flow.html#looping-through-a-map-with-for
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let mut acc = 0;
  ///
  /// a.for_each(|(&k, &v)| acc += k + v);
  ///
  /// assert_eq!(acc, 12);
  /// ```
  ///
  /// For such a small example, a `for` loop may be cleaner, but `for_each`
  /// might be preferable to keep a functional style with longer maps:
  ///
  /// ```
  /// (0..5).flat_map(|x| x * 100 .. x * 110)
  ///       .enumerate()
  ///       .filter(|&(i, x)| (i + x) % 3 == 0)
  ///       .for_each(|(i, x)| println!("{i}:{x}"));
  /// ```
  fn for_each(&self, function: impl FnMut((&Key, &Value)));

  /// Creates a new map by retaining the values representing the intersection
  /// of the original map with another map i.e., the values that are
  /// both in `self` and `other`.
  ///
  /// The order or retained values is preserved for ordered maps.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashSet;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// let intersection = a.intersect(&vec![(4, 4), (2, 2), (3, 4), (4, 5)]);
  ///
  /// assert_eq!(intersection, HashMap::from([
  ///   (2, 2),
  /// ]));
  /// assert_eq!(e.intersect(&vec![(1, 1)]), HashMap::new());
  ///
  /// // Print 2 2.
  /// for (k, v) in intersection {
  ///   println!("{k} {v}");
  /// }
  /// ```
  #[inline]
  fn intersect<'a>(self, entries: &'a impl Iterable<Item<'a> = &'a (Key, Value)>) -> Self
  where
    Key: Eq + Hash + 'a,
    Value: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let retained: HashSet<(&Key, &Value)> = HashSet::from_iter(entries.iterator().map(|(k, v)| (k, v)));
    self.into_iter().filter(|(k, v)| retained.contains(&(k, v))).collect()
  }

  /// Creates a new vector from the elements of this map.
  ///
  /// This is an equivalent of [`Iterator::collect`].
  ///
  /// This is a consuming variant of [`to_vec()`].
  ///
  /// [`to_vec()`]: Map::to_vec
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::BTreeMap;
  ///
  /// let a = BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.into_vec(), vec![(1, 1), (2, 2), (3, 3)]);
  /// ```
  #[inline]
  fn into_vec(self) -> Vec<(Key, Value)>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new map by applying the given closure `function` to each entry in
  /// the original map.
  ///
  /// The closure `function` takes a reference to an entry of type
  /// `(Key, Value)` and returns a value of type `(L, W)`.
  /// The resulting other are collected into a new map of the same type.
  ///
  /// This is a consuming variant of [`map_ref()`].
  ///
  /// [`map_ref()`]: Map::map_ref
  ///
  /// # Arguments
  ///
  /// * `self` - the map to apply the mapping to.
  /// * `function` - the closure to apply to each entry.
  ///
  /// # Returns
  ///
  /// A new map of the same type, containing the mapped entries.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.map(|(k, v)| (k, k + v)),
  ///   HashMap::from([
  ///     (1, 2),
  ///     (2, 4),
  ///     (3, 6),
  /// ]));
  /// ```
  #[inline]
  fn map<L, W>(self, function: impl FnMut((Key, Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().map(function).collect()
  }

  /// Creates a new map by applying the given closure `function` to each entry in
  /// the original map.
  ///
  /// The closure `function` takes a reference to an entry of type
  /// `(Key, Value)` and returns a value of type `(L, W)`.
  /// The resulting other are collected into a new map of the same type.
  ///
  /// This is a consuming variant of [`map()`].
  ///
  /// [`map()`]: Map::map
  ///
  /// # Arguments
  ///
  /// * `self` - the map to apply the mapping to.
  /// * `function` - the closure to apply to each entry.
  ///
  /// # Returns
  ///
  /// A new map of the same type, containing the mapped entries.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.map_ref(|(&k, &v)| (k, k + v)),
  ///   HashMap::from([
  ///     (1, 2),
  ///     (2, 4),
  ///     (3, 6),
  /// ]));
  /// ```
  fn map_ref<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  /// Creates a new map by applying the given closure `function` to each key in
  /// the original map.
  ///
  /// The closure `function` takes a reference to an entry of type
  /// `Key` and returns a value of type `L`.
  /// The resulting other are collected into a new map of the same type.
  ///
  /// # Arguments
  ///
  /// * `self` - the map to apply the mapping to.
  /// * `function` - the closure to apply to each key.
  ///
  /// # Returns
  ///
  /// A new map of the same type, containing the mapped keys.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.map_keys(|&k| k + 1),
  ///   HashMap::from([
  ///     (2, 1),
  ///     (3, 2),
  ///     (4, 3),
  /// ]));
  /// ```
  #[inline]
  fn map_keys<L>(self, mut function: impl FnMut(&Key) -> L) -> Self::This<L, Value>
  where
    L: Eq + Hash,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, Value>: FromIterator<(L, Value)>,
  {
    self.into_iter().map(|(k, v)| (function(&k), v)).collect()
  }

  /// Creates a new map by applying the given closure `function` to each value in
  /// the original map.
  ///
  /// The closure `function` takes a reference to an entry of type
  /// `Value` and returns a value of type `W`.
  /// The resulting other are collected into a new map of the same type.
  ///
  /// # Arguments
  ///
  /// * `self` - the map to apply the mapping to.
  /// * `function` - the closure to apply to each value.
  ///
  /// # Returns
  ///
  /// A new map of the same type, containing the mapped keys.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.map_values(|&v| v + 1),
  ///   HashMap::from([
  ///     (1, 2),
  ///     (2, 3),
  ///     (3, 4),
  /// ]));
  /// ```
  #[inline]
  fn map_values<W>(self, mut function: impl FnMut(&Value) -> W) -> Self::This<Key, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<Key, W>: FromIterator<(Key, W)>,
  {
    self.into_iter().map(|(k, v)| (k, function(&v))).collect()
  }

  /// Returns the entry that gives the maximum value with respect to the
  /// specified comparison function.
  ///
  /// If several entries are equally maximum, the last entry is
  /// returned. If the map is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.max_by(|x, y| x.0.cmp(y.0)), Some((&3, &3)));
  ///
  /// assert_eq!(e.max_by(|x, y| x.0.cmp(y.0)), None);
  /// ```
  fn max_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  /// Returns the entry that gives the maximum value from the
  /// specified function.
  ///
  /// If several entries are equally maximum, the last entry is
  /// returned. If the map is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.max_by_key(|(k, _)| -k), Some((&1, &1)));
  ///
  /// assert_eq!(e.max_by_key(|(k, _)| -k), None);
  /// ```
  fn max_by_key<K>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>
  where
    K: Ord;

  /// Returns the maximum entry of this map.
  ///
  /// If several entries are equally maximum, the last entry is
  /// returned. If the map is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.max_of(), Some((&3, &3)));
  ///
  /// assert_eq!(e.max_of(), None);
  /// ```
  #[inline]
  fn max_of(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.max_by(|x, y| x.cmp(&y))
  }

  /// Returns the entry that gives the minimum value with respect to the
  /// specified comparison function.
  ///
  /// If several entries are equally minimum, the last entry is
  /// returned. If the map is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.min_by(|x, y| x.0.cmp(y.0)), Some((&1, &1)));
  ///
  /// assert_eq!(e.min_by(|x, y| x.0.cmp(y.0)), None);
  /// ```
  fn min_by(&self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering) -> Option<(&Key, &Value)>;

  /// Returns the entry that gives the minimum value from the
  /// specified function.
  ///
  /// If several entries are equally minimum, the last entry is
  /// returned. If the map is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.min_by_key(|(k, _)| -k), Some((&3, &3)));
  ///
  /// assert_eq!(e.min_by_key(|(k, _)| -k), None);
  /// ```
  fn min_by_key<K>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>
  where
    K: Ord;

  /// Returns the minimum entry of this map.
  ///
  /// If several entries are equally minimum, the last entry is
  /// returned. If the map is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.min_of(), Some((&1, &1)));
  ///
  /// assert_eq!(e.min_of(), None);
  /// ```
  #[inline]
  fn min_of(&self) -> Option<(&Key, &Value)>
  where
    Key: Ord,
    Value: Ord,
  {
    self.min_by(|(k1, v1), (k2, v2)| (k1, v1).cmp(&(k2, v2)))
  }

  /// Returns the minimum and maximum entry of this map with respect to the
  /// specified comparison function.
  ///
  /// For the minimum, the first minimal entry is returned. For the maximum,
  /// the last maximal entry is returned. If the map is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.minmax_by(|x, y| x.0.cmp(y.0)), Some(((&1, &1), (&3, &3))));
  ///
  /// assert_eq!(e.minmax_by(|x, y| x.0.cmp(y.0)), None);
  /// ```
  fn minmax_by(
    &self, compare: impl FnMut((&Key, &Value), (&Key, &Value)) -> Ordering,
  ) -> Option<((&Key, &Value), (&Key, &Value))>;

  /// Returns the minimum and maximum entry of this map from the
  /// specified function.
  ///
  /// For the minimum, the first minimal entry is returned. For the maximum,
  /// the last maximal entry is returned. If the map is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.minmax_by_key(|(k, _)| -k), Some(((&3, &3), (&1, &1))));
  ///
  /// assert_eq!(e.minmax_by_key(|(k, _)| -k), None);
  /// ```
  fn minmax_by_key<K>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<((&Key, &Value), (&Key, &Value))>
  where
    K: Ord;

  /// Return the minimum and maximum entry of this map.
  ///
  /// For the minimum, the first minimal entry is returned. For the maximum,
  /// the last maximal entry is returned. If the map is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.minmax_of(), Some(((&1, &1), (&3, &3))));
  ///
  /// assert_eq!(e.minmax_of(), None);
  /// ```
  #[inline]
  fn minmax_of(&self) -> Option<((&Key, &Value), (&Key, &Value))>
  where
    Key: Ord,
    Value: Ord,
  {
    self.minmax_by(|(x1, x2), (y1, y2)| (x1, x2).cmp(&(y1, y2)))
  }

  /// Creates two new maps from the original map using by applying
  /// specified predicate.
  ///
  /// The predicate passed to `partition()` can return `true`, or `false`.
  /// `partition()` returns a pair, all the entries for which it returned
  /// `true`, and all the entries for which it returned `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let (even, odd) = a.partition(|(&k, _)| k % 2 == 0);
  ///
  /// assert_eq!(even, HashMap::from([
  ///   (2, 2),
  /// ]));
  /// assert_eq!(odd, HashMap::from([
  ///   (1, 1),
  ///   (3, 3),
  /// ]));
  /// ```
  #[inline]
  fn partition(self, mut predicate: impl FnMut((&Key, &Value)) -> bool) -> (Self, Self)
  where
    Self: IntoIterator<Item = (Key, Value)> + Default + Extend<(Key, Value)>,
  {
    self.into_iter().partition(|(k, v)| predicate((k, v)))
  }

  /// Creates two new maps with arbitrary entry types from the original map
  /// by applying specified function.
  ///
  /// The function passed to `partition_map()` can return `Ok`, or `Err`.
  /// `partition_map()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a consuming variant of [`partition_map_ref()`].
  ///
  /// [`partition_map_ref()`]: Map::partition_map_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let (even, odd) = a.partition_map(|(k, v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  ///
  /// assert_eq!(even, HashMap::from([
  ///   (5, 2),
  /// ]));
  /// assert_eq!(odd, HashMap::from([
  ///   (1, 1),
  ///   (3, 3),
  /// ]));
  /// ```
  fn partition_map<L1, W1, L2, W2>(
    self, mut function: impl FnMut((Key, Value)) -> Result<(L1, W1), (L2, W2)>,
  ) -> (Self::This<L1, W1>, Self::This<L2, W2>)
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L1, W1>: Default + Extend<(L1, W1)>,
    Self::This<L2, W2>: Default + Extend<(L2, W2)>,
  {
    let mut result_left: Self::This<L1, W1> = Self::This::default();
    let mut result_right: Self::This<L2, W2> = Self::This::default();
    for item in self.into_iter() {
      match function(item) {
        Ok(value) => result_left.extend(iter::once(value)),
        Err(value) => result_right.extend(iter::once(value)),
      }
    }
    (result_left, result_right)
  }

  /// Creates two new maps with arbitrary entry types from the original map
  /// by applying specified function.
  ///
  /// The function passed to `partition_map_ref()` can return `Ok`, or `Err`.
  /// `partition_map_ref()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a non-consuming variant of [`partition_map()`].
  ///
  /// [`partition_map()`]: Map::partition_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// let (even, odd) = a.partition_map_ref(|(&k, &v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  ///
  /// assert_eq!(even, HashMap::from([
  ///   (5, 2),
  /// ]));
  /// assert_eq!(odd, HashMap::from([
  ///   (1, 1),
  ///   (3, 3),
  /// ]));
  /// ```
  fn partition_map_ref<L1, W1, L2, W2>(
    &self, function: impl FnMut((&Key, &Value)) -> Result<(L1, W1), (L2, W2)>,
  ) -> (Self::This<L1, W1>, Self::This<L2, W2>)
  where
    Self::This<L1, W1>: Default + Extend<(L1, W1)>,
    Self::This<L2, W2>: Default + Extend<(L2, W2)>;

  /// Iterates over the entire map, multiplying all the keys
  ///
  /// An empty map returns the one value of the type.
  ///
  /// `product_keys()` can be used to multiply any type implementing [`Product`],
  ///
  /// [`Product`]: Product
  ///
  /// # Panics
  ///
  /// When calling `product_keys()` and a primitive integer type is being returned,
  /// method will panic if the computation overflows and debug assertions are
  /// enabled.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.product_keys(), 6);
  /// assert_eq!(e.product_keys(), 1);
  /// ```
  #[inline]
  fn product_keys(self) -> Key
  where
    Key: Product,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).product()
  }

  /// Iterates over the entire map, multiplying all the values
  ///
  /// An empty map returns the one value of the type.
  ///
  /// `product_values()` can be used to multiply any type implementing [`Product`],
  ///
  /// [`Product`]: Product
  ///
  /// # Panics
  ///
  /// When calling `product_values()` and a primitive integer type is being returned,
  /// method will panic if the computation overflows and debug assertions are
  /// enabled.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.product_values(), 6);
  /// assert_eq!(e.product_values(), 1);
  /// ```
  #[inline]
  fn product_values(self) -> Value
  where
    Value: Product,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).product()
  }

  /// Reduces the elements to a single one, by repeatedly applying a reducing
  /// operation.
  ///
  /// If the collection is empty, returns [`None`]; otherwise, returns the
  /// result of the reduction.
  ///
  /// The reducing function is a closure with two arguments: an 'accumulator', and an element.
  /// For collections with at least one element, this is the same as [`fold()`]
  /// with the first element of the collection as the initial accumulator value, folding
  /// every subsequent element into it.
  ///
  /// This is a consuming variant of [`reduce_ref()`].
  ///
  /// [`fold()`]: Map::fold
  /// [`reduce_ref()`]: Map::reduce_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let a_source = HashMap::from([
  /// #   (1, 1),
  /// #   (2, 2),
  /// #   (3, 3),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.reduce(|(a, b), (k, v)| (a + k, b + v)),
  ///   Some((6, 6))
  /// );
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// # let a = a_source.clone();
  /// let folded = a.fold((0, 0), |(a, b), (k, v)| (a + k, b + v));
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(
  ///   a.reduce(|(a, b), (k, v)| (a + k, b + v)).unwrap(),
  ///   folded
  /// );
  /// ```
  fn reduce(self, mut function: impl FnMut((Key, Value), (Key, Value)) -> (Key, Value)) -> Option<(Key, Value)>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    let mut iterator = self.into_iter();
    iterator.next().and_then(|value1| {
      iterator.next().map(|value2| iterator.fold(function(value1, value2), |(k, v), x| function((k, v), x)))
    })
  }

  /// Reduces the elements to a single one, by repeatedly applying a reducing
  /// operation.
  ///
  /// If the collection is empty, returns [`None`]; otherwise, returns the
  /// result of the reduction.
  ///
  /// The reducing function is a closure with two arguments: an 'accumulator', and an element.
  /// For collections with at least one element, this is the same as [`fold_ref()`]
  /// with the first element of the collection as the initial accumulator value, folding
  /// every subsequent element into it.
  ///
  /// This is a non-consuming variant of [`reduce()`].
  ///
  /// [`fold_ref()`]: Map::fold_ref
  /// [`reduce()`]: Map::reduce
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let a_source = HashMap::from([
  /// #   (1, 1),
  /// #   (2, 2),
  /// #   (3, 3),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.reduce_ref(|(&a, &b), (&k, &v)| (a + k, b + v)),
  ///   Some((6, 6))
  /// );
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// let folded = a.fold_ref((0, 0), |(a, b), (&k, &v)| (a + k, b + v));
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(
  ///   a.reduce_ref(|(&a, &b), (&k, &v)| (a + k, b + v)).unwrap(),
  ///   folded
  /// );
  /// ```
  fn reduce_ref(&self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)>;

  /// Tests if all keys of this map can be found in another collection.
  ///
  /// Returns `true` if this map is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert!(a.subset(&vec![4, 3, 2, 2, 1]));
  /// assert!(e.subset(&vec![1]));
  ///
  /// assert!(!a.subset(&vec![1, 2]));
  /// assert!(!a.subset(&vec![]));
  /// ```
  fn subset<'a>(&'a self, keys: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

  /// Creates a new map from the original map by replacing the specified key
  /// and its value with a different entry.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.substitute(&3, 4, 4),
  ///   HashMap::from([
  ///     (1, 1),
  ///     (2, 2),
  ///     (4, 4),
  /// ]));
  /// ```
  #[inline]
  fn substitute(self, value: &Key, replacement_key: Key, replacement_value: Value) -> Self
  where
    Key: PartialEq,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let mut replaced = Some((replacement_key, replacement_value));
    self
      .into_iter()
      .map(|(key, val)| if &key == value { replaced.take().unwrap_or((key, val)) } else { (key, val) })
      .collect()
  }

  /// Creates a new map from the original map by replacing the given occurrences of elements
  /// found in another collection with elements of a replacement collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.substitute_multi(&vec![2, 3], vec![(4, 4), (5, 5)]),
  ///   HashMap::from([
  ///     (1, 1),
  ///     (4, 4),
  ///     (5, 5),
  /// ]));
  /// ```
  #[inline]
  fn substitute_multi<'a>(
    self, keys: &'a impl Iterable<Item<'a> = &'a Key>, replacements: impl IntoIterator<Item = (Key, Value)>,
  ) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let keys_iterator = keys.iterator();
    let mut replaced = HashMap::<&Key, LinkedList<(Key, Value)>>::with_capacity(keys_iterator.size_hint().0);
    for (item, replacement) in keys_iterator.zip(replacements.into_iter()) {
      replaced.entry(item).or_default().push_back(replacement);
    }
    self
      .into_iter()
      .map(
        |(k, v)| if let Some(entries) = replaced.get_mut(&k) { entries.pop_front().unwrap_or((k, v)) } else { (k, v) },
      )
      .collect()
  }

  /// Tests if all the elements of another collection can be found among the keys of this map.
  ///
  /// Returns `true` if the other collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert!(a.superset(&vec![3, 1]));
  /// assert!(a.superset(&vec![]));
  ///
  /// assert!(!a.superset(&vec![1, 2, 3, 4]));
  /// assert!(!a.superset(&vec![1, 2, 2]));
  /// assert!(!a.superset(&vec![3, 4]));
  /// assert!(!e.superset(&vec![1]));
  /// ```
  fn superset<'a>(&'a self, keys: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

  /// Sums keys of this map.
  ///
  /// Takes each key, adds them together, and returns the result.
  ///
  /// An empty map returns the zero value of the type.
  ///
  /// `sum_keys()` can be used to multiply any type implementing [`Sum`],
  ///
  /// [`Sum`]: Sum
  ///
  /// # Panics
  ///
  /// When calling `sum_keys()` and a primitive integer type is being returned, this
  /// method will panic if the computation overflows and debug assertions are
  /// enabled.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.sum_keys(), 6);
  /// assert_eq!(e.sum_keys(), 0);
  /// ```
  #[inline]
  fn sum_keys(self) -> Key
  where
    Key: Sum,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(k, _)| k).sum()
  }

  /// Sums values of this map.
  ///
  /// Takes each value, adds them together, and returns the result.
  ///
  /// An empty map returns the zero value of the type.
  ///
  /// `sum_values()` can be used to multiply any type implementing [`Sum`],
  ///
  /// [`Sum`]: Sum
  ///
  /// # Panics
  ///
  /// When calling `sum_values()` and a primitive integer type is being returned, this
  /// method will panic if the computation overflows and debug assertions are
  /// enabled.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  /// let e = HashMap::<i32, i32>::new();
  ///
  /// assert_eq!(a.sum_values(), 6);
  /// assert_eq!(e.sum_values(), 0);
  /// ```
  #[inline]
  fn sum_values(self) -> Value
  where
    Value: Sum,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().map(|(_, v)| v).sum()
  }

  /// Creates a new ordered map from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_bmap()`].
  ///
  /// [`into_bmap()`]: crate::TransformTo::into_bmap
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeMap, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_bmap(), BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  fn to_bmap(self) -> BTreeMap<Key, Value>
  where
    Key: Ord + Clone,
    Value: Clone;

  /// Creates a new ordered set from the elements of this map.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_bset()`].
  ///
  /// [`into_bset()`]: crate::TransformTo::into_bset
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_bset(), BTreeSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  fn to_bset(self) -> BTreeSet<(Key, Value)>
  where
    Key: Ord + Clone,
    Value: Ord + Clone;

  /// Creates a new double-ended queue from the elements of this map.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_deque()`].
  ///
  /// [`into_deque()`]: crate::TransformTo::into_deque
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeMap, VecDeque};
  ///
  /// let a = BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_deque(), VecDeque::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  fn to_deque(self) -> VecDeque<(Key, Value)>
  where
    Key: Clone,
    Value: Clone;

  /// Creates a new priority queue from the elements of this map.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_heap()`].
  ///
  /// [`into_heap()`]: crate::TransformTo::into_heap
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{HashMap, HashSet};
  /// use std::collections::BinaryHeap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(
  ///   a.to_heap().to_set(),
  ///   BinaryHeap::from([(1, 1), (2, 2), (3, 3)]).to_set()
  /// );
  /// ```
  fn to_heap(self) -> BinaryHeap<(Key, Value)>
  where
    Key: Ord + Clone,
    Value: Ord + Clone;

  /// Creates a new vector from the keys of this map in arbitrary order.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`] for map keys.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_keys().to_set(), vec![1, 2, 3].to_set());
  /// ```
  fn to_keys(&self) -> Vec<Key>
  where
    Key: Clone;

  /// Creates a new doubly-linked list from the elements of this map.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_list()`].
  ///
  /// [`into_list()`]: crate::TransformTo::into_list
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{BTreeMap, LinkedList};
  ///
  /// let a = BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_list(), LinkedList::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  fn to_list(self) -> LinkedList<(Key, Value)>
  where
    Key: Clone,
    Value: Clone;

  /// Creates a new hash map from the elements of this collection.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_map()`].
  ///
  /// [`into_map()`]: crate::TransformTo::into_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_map(), HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3)
  /// ]));
  /// ```
  fn to_map(self) -> HashMap<Key, Value>
  where
    Key: Eq + Hash + Clone,
    Value: Clone;

  /// Creates a new ordered set from the elements of this map.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_set()`].
  ///
  /// [`into_set()`]: crate::TransformTo::into_set
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::{HashSet, HashMap};
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_set(), HashSet::from([(1, 1), (2, 2), (3, 3)]));
  /// ```
  fn to_set(self) -> HashSet<(Key, Value)>
  where
    Key: Eq + Hash + Clone,
    Value: Eq + Hash + Clone;

  /// Creates a new vector from the values of this map in arbitrary order.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`] for map values.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_values().to_set(), vec![1, 2, 3].to_set());
  /// ```
  fn to_values(&self) -> Vec<Value>
  where
    Value: Clone;

  /// Creates a new vector from the elements of this map.
  ///
  /// This is a non-consuming equivalent of [`Iterator::collect`].
  ///
  /// This is a non-consuming variant of [`into_bmap()`].
  ///
  /// [`into_bmap()`]: crate::TransformVec::into_vec
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::BTreeMap;
  ///
  /// let a = BTreeMap::from([
  ///   (1, 1),
  ///   (2, 2),
  ///   (3, 3),
  /// ]);
  ///
  /// assert_eq!(a.to_vec(), vec![(1, 1), (2, 2), (3, 3)]);
  /// ```
  fn to_vec(self) -> Vec<(Key, Value)>
  where
    Key: Clone,
    Value: Clone;

  /// Creates a new map containing a single element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// assert_eq!(
  ///   HashMap::unit(1, 1),
  ///   HashMap::from([
  ///     (1, 1),
  /// ]));
  #[inline]
  fn unit(key: Key, value: Value) -> Self
  where
    Self: FromIterator<(Key, Value)>,
  {
    iter::once((key, value)).collect()
  }
}

pub(crate) fn minmax_by_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut compare: impl FnMut((&K, &V), (&K, &V)) -> Ordering,
) -> Option<((&'a K, &'a V), (&'a K, &'a V))> {
  iterator.next().map(|item| {
    let mut min = item;
    let mut max = min;
    for item in iterator {
      if compare(item, min) == Ordering::Less {
        min = item;
      }
      if compare(item, max) == Ordering::Greater {
        max = item;
      }
    }
    (min, max)
  })
}

#[inline]
pub(crate) fn minmax_by_key_pairs<'a, K: 'a, V: 'a, E: Ord>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut to_key: impl FnMut((&K, &V)) -> E,
) -> Option<((&'a K, &'a V), (&'a K, &'a V))> {
  minmax_by_pairs(iterator, |x, y| to_key(x).cmp(&to_key(y)))
}

pub(crate) fn partition_map_pairs<'a, K: 'a, V: 'a, L1, W1, L2, W2, Left, Right>(
  iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V)) -> Result<(L1, W1), (L2, W2)>,
) -> (Left, Right)
where
  Left: Default + Extend<(L1, W1)>,
  Right: Default + Extend<(L2, W2)>,
{
  let mut result_left = Left::default();
  let mut result_right = Right::default();
  for item in iterator {
    match function(item) {
      Ok(value) => result_left.extend(iter::once(value)),
      Err(value) => result_right.extend(iter::once(value)),
    }
  }
  (result_left, result_right)
}

#[inline]
pub(crate) fn reduce_pairs<'a, K: 'a, V: 'a>(
  mut iterator: impl Iterator<Item = (&'a K, &'a V)>, mut function: impl FnMut((&K, &V), (&K, &V)) -> (K, V),
) -> Option<(K, V)> {
  iterator.next().and_then(|value1| {
    iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function((&r.0, &r.1), x)))
  })
}
