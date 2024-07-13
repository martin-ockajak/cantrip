use crate::extensions::iterable::Iterable;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

/// Map operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a map
/// - May consume the map and its entries
/// - May create a new map
///
pub trait Map<Key, Value> {
  type This<K, V>;

  /// Creates a new map by adding an entry to the original map.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let source = HashMap::from([
  /// #  (1, "a"),
  /// #  (2, "b"),
  /// #  (3, "c"),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.add(4, "d"), HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  ///   (4, "d"),
  /// ]));
  /// # let a = source.clone();
  /// assert_eq!(a.add(1, "d"), HashMap::from([
  ///   (1, "d"),
  ///   (2, "b"),
  ///   (3, "c"),
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
  /// # let source = HashMap::from([
  /// #  (1, "a"),
  /// #  (2, "b"),
  /// #  (3, "c"),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.add_all(vec![(4, "d"), (5, "e")]), HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  ///   (4, "d"),
  ///   (5, "e"),
  /// ]));
  /// # let a = source.clone();
  /// assert_eq!(a.add_all(vec![(1, "d"), (5, "e")]), HashMap::from([
  ///   (1, "d"),
  ///   (2, "b"),
  ///   (3, "c"),
  ///   (5, "e"),
  /// ]));
  /// ```
  #[inline]
  fn add_all(self, iterable: impl IntoIterator<Item = (Key, Value)>) -> Self
  where
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    self.into_iter().chain(iterable).collect()
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.all(|(&k, &v)| k > 0 && v.len() > 0));
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.any(|(&k, &v)| k > 0 && v.len() > 0));
  ///
  /// assert!(!a.any(|(&k, _)| k > 5));
  /// assert!(!e.any(|(&k, _)| k > 0));
  /// ```
  fn any(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> bool;

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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.count_by(|(&k, &v)| k == 2 && v == "b"), 1);
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let b = HashMap::from([
  ///   (1, "a"),
  ///   (2, "a"),
  ///   (3, "a"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.count_unique(), 3);
  /// assert_eq!(b.count_unique(), 1);
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.delete(&2), HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
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
  ///
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.delete_all(&vec![1, 3]), HashMap::from([
  ///   (2, "b"),
  /// ]));
  ///
  /// assert_eq!(e.delete_all(&vec![1]), HashMap::new());
  /// ```
  #[inline]
  fn delete_all<'a>(self, iterable: &'a impl Iterable<Item<'a> = &'a Key>) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let removed: HashSet<&Key> = HashSet::from_iter(iterable.iterator());
    self.into_iter().filter(|(k, _)| !removed.contains(k)).collect()
  }

  /// Creates a new map containing a result of a function
  /// specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// assert_eq!(HashMap::fill_with(|| (1, "a"), 1), HashMap::from([
  ///   (1, "a"),
  /// ]));
  ///
  /// assert_eq!(HashMap::fill_with(|| (1, "a"), 0), HashMap::new());
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let filtered = a.filter(|(&k, &v)| k != 2 && v != "b");
  ///
  /// assert_eq!(filtered, HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let filtered = a.filter_keys(|&k| k != 2);
  ///
  /// assert_eq!(filtered, HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let filtered = a.filter_values(|&v| v != "b");
  ///
  /// assert_eq!(filtered, HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
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
  /// closure returns `Some(entry)`.
  ///
  /// `filter_map` can be used to make chains of [`filter`] and [`map`] more
  /// concise. The example below shows how a `map().filter().map_to()` can be
  /// shortened to a single call to `filter_map`.
  ///
  /// This is a non-consuming variant of [`filter_map_to`].
  ///
  /// [`filter`]: Map::filter
  /// [`map`]: Map::map
  /// [`filter_map_to`]: Map::filter_map_to
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
  ///   (1, "1"),
  ///   (2, "two"),
  ///   (3, "NaN"),
  /// ]);
  ///
  /// let filter_mapped = a.filter_map(|(&k, &v)| v.parse::<i32>().ok().map(|v| (k, v)));
  ///
  /// assert_eq!(filter_mapped, HashMap::from([
  ///   (1, 1),
  /// ]));
  /// ```
  ///
  /// Here's the same example, but with [`filter`] and [`map`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "1"),
  ///   (2, "two"),
  ///   (3, "NaN"),
  /// ]);
  ///
  /// let filter_mapped = a
  ///   .map(|(&k, &v)| (k, v.parse::<i32>()))
  ///   .filter(|(_, s)| s.is_ok())
  ///   .map(|(&k, v)| (k, v.clone().unwrap()));
  ///
  /// assert_eq!(filter_mapped, HashMap::from([
  ///   (1, 1),
  /// ]));
  /// ```
  fn filter_map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  /// Creates a new map by filtering and mapping the original map.
  ///
  /// The returned map contains only the `entries` for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map` can be used to make chains of [`filter`] and [`map`] more
  /// concise. The example below shows how a `map_to().filter_to().map()` can be
  /// shortened to a single call to `filter_map_to`.
  ///
  /// This is a consuming variant of [`filter_map`].
  ///
  /// [`filter`]: Map::filter
  /// [`map`]: Map::map
  /// [`filter_map`]: Map::filter_map
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
  ///   (1, "1"),
  ///   (2, "two"),
  ///   (3, "NaN"),
  /// ]);
  ///
  /// let filter_mapped = a.filter_map_to(|(k, v)| v.parse::<i32>().ok().map(|v| (k, v)));
  ///
  /// assert_eq!(filter_mapped, HashMap::from([
  ///   (1, 1),
  /// ]));
  /// ```
  ///
  /// Here's the same example, but with [`filter`] and [`map_to`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "1"),
  ///   (2, "two"),
  ///   (3, "NaN"),
  /// ]);
  ///
  /// let filter_mapped = a
  ///   .map_to(|(k, v)| (k, v.parse::<i32>()))
  ///   .filter(|(_, s)| s.is_ok())
  ///   .map(|(&k, v)| (k, v.clone().unwrap()));
  ///
  /// assert_eq!(filter_mapped, HashMap::from([
  ///   (1, 1),
  /// ]));
  /// ```
  #[inline]
  fn filter_map_to<L, W>(self, function: impl FnMut((Key, Value)) -> Option<(L, W)>) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().filter_map(function).collect()
  }

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
  /// If you need the index of the entry, see [`position()`].
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.find(|(&k, &v)| k == 2 && v == "b"), Some((&2, &"b")));
  ///
  /// assert_eq!(a.find(|(&k, _)| k == 5), None);
  /// ```
  fn find(&self, predicate: impl FnMut((&Key, &Value)) -> bool) -> Option<(&Key, &Value)>;

  /// Applies function to the entries of this map and returns
  /// the first non-none result.
  ///
  /// `find_map` can be used to make chains of [`find`] and [`map`] more
  /// concise.
  ///
  /// `find_map_to(f)` is equivalent to `find().map()`.
  ///
  /// This is a non-consuming variant of [`find_map_to`].
  ///
  /// [`find`]: Map::find
  /// [`map`]: Map::map
  /// [`find_map_to`]: Map::find_map_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "one"),
  ///   (2, "2"),
  ///   (3, "NaN"),
  /// ]);
  ///
  /// let first_number = a.find_map(|(_, &v)| v.parse().ok());
  ///
  /// assert_eq!(first_number, Some(2));
  /// ```
  fn find_map<B>(&self, function: impl FnMut((&Key, &Value)) -> Option<B>) -> Option<B>;

  /// Applies function to the entries of this map and returns
  /// the first non-none result.
  ///
  /// `find_map_to` can be used to make chains of [`find`] and [`map`] more concise.
  ///
  /// `find_map_to(f)` is equivalent to `find().map()`.
  ///
  /// This is a consuming variant of [`find_map`].
  ///
  /// [`find`]: Map::find
  /// [`map`]: Map::map
  /// [`find_map`]: Map::find_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "one"),
  ///   (2, "2"),
  ///   (3, "NaN"),
  /// ]);
  ///
  /// let first_number = a.find_map_to(|(_, v)| v.parse().ok());
  ///
  /// assert_eq!(first_number, Some(2));
  /// ```
  #[inline]
  fn find_map_to<B>(self, function: impl FnMut((Key, Value)) -> Option<B>) -> Option<B>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().find_map(function)
  }

  /// Creates a new map by applying the given closure `function` to each entry
  /// of the original map and flattens the nested map.
  ///
  /// The [`flat_map`] adapter is very useful, but only when the closure
  /// argument produces values. If it produces an iterable value instead, there's
  /// an extra layer of indirection. `flat_map()` will remove this extra layer
  /// on its own.
  ///
  /// You can think of `flat_map(f)` as the semantic equivalent
  /// of [`map`]ping, and then [`flatten`]ing as in `map(f).flatten()`.
  ///
  /// Another way of thinking about `flat_map()`: [`map`]'s closure returns
  /// one item for each entry, and `flat_map()`'s closure returns an
  /// iterable value for each entry.
  ///
  /// This is a non-consuming variant of [`flat_map_to`].
  ///
  /// [`map`]: Map::map
  /// [`flat`]: crate::Collectible::flat
  /// [`flat_map_to`]: Map::flat_map_to
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// // Vec is iterable because it implements IntoIterator
  /// let flat_mapped = a.flat_map(|(&k, &v)| vec![(-k, v), (k, v)]);
  ///
  /// assert_eq!(flat_mapped, HashMap::from([
  ///   (-1, "a"),
  ///   (-2, "b"),
  ///   (-3, "c"),
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]));
  /// ```
  fn flat_map<L, W, R>(&self, function: impl FnMut((&Key, &Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self::This<L, W>: FromIterator<(L, W)>;

  /// Creates a new map by applying the given closure `function` to each entry
  /// of the original map and flattens the nested map.
  ///
  /// The [`flat_map`] adapter is very useful, but only when the closure
  /// argument produces values. If it produces an iterable value instead, there's
  /// an extra layer of indirection. `flat_map()` will remove this extra layer
  /// on its own.
  ///
  /// You can think of `flat_map(f)` as the semantic equivalent
  /// of [`map`]ping, and then [`flatten`]ing as in `map(f).flatten()`.
  ///
  /// Another way of thinking about `flat_map()`: [`map`]'s closure returns
  /// one item for each entry, and `flat_map()`'s closure returns an
  /// iterable value for each entry.
  ///
  /// This is a consuming variant of [`flat_map`].
  ///
  /// [`map`]: Map::map
  /// [`flat`]: crate::Collectible::flat
  /// [`flat_map`]: Map::flat_map
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// // Vec is iterable because it implements IntoIterator
  /// let flat_mapped = a.flat_map(|(&k, &v)| vec![(-k, v), (k, v)]);
  ///
  /// assert_eq!(flat_mapped, HashMap::from([
  ///   (-1, "a"),
  ///   (-2, "b"),
  ///   (-3, "c"),
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]));
  /// ```
  #[inline]
  fn flat_map_to<L, W, R>(self, function: impl FnMut((Key, Value)) -> R) -> Self::This<L, W>
  where
    R: IntoIterator<Item = (L, W)>,
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().flat_map(function).collect()
  }

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
  /// After applying this closure to every entry of the map, `fold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a map of something, and want
  /// to produce a single value from it.
  ///
  /// This is a non-consuming variant of [`fold_to`].
  ///
  /// Note: [`reduce()`] can be used to use the first entry as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold()` combines entries in a *left-associative* fashion. For associative
  /// operators like `+`, the order the entries are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  ///
  /// [`fold_to`]: Map::fold_to
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// // the sum of all the elements of the array
  /// let sum = a.fold(0, |acc, (&k, &v)| acc + k + v.len());
  ///
  /// assert_eq!(sum, 9);
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
  fn fold<B>(self, initial_value: B, function: impl FnMut(B, (&Key, &Value)) -> B) -> B;

  /// Folds every entry into an accumulator by applying an operation,
  /// returning the final result.
  ///
  /// `fold_to()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an entry. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every entry of this map, `fold_to()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a map of something, and want
  /// to produce a single value from it.
  ///
  /// This is a consuming variant of [`fold`].
  ///
  /// Note: [`reduce_to()`] can be used to use the first entry as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold_to()` combines entries in a *left-associative* fashion. For associative
  /// operators like `+`, the order the entries are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  ///
  /// [`fold`]: Map::fold
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// // the sum of all the elements of the array
  /// let sum = a.fold_to(0, |acc, (k, v)| acc + k + v.len());
  ///
  /// assert_eq!(sum, 9);
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
  fn fold_to<B>(self, initial_value: B, function: impl FnMut(B, (Key, Value)) -> B) -> B
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    self.into_iter().fold(initial_value, function)
  }

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
  /// use std::sync::mpsc::channel;
  ///
  /// let (tx, rx) = channel();
  /// (0..3).for_each(move |x| tx.send(x).unwrap());
  ///
  /// let v: Vec<_> = rx.iter().collect();
  /// assert_eq!(v, vec![0, 1, 2]);
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// let intersection = a.intersect(&vec![(4, "x"), (2, "b"), (3, "y"), (4, "x")]);
  ///
  /// assert_eq!(intersection, HashMap::from([
  ///   (2, "b"),
  /// ]));
  /// assert_eq!(e.intersect(&vec![(1, "a")]), HashMap::new());
  ///
  /// // Print 2 "b".
  /// for (k, v) in intersection {
  ///   println!("{k} {v}");
  /// }
  /// ```
  #[inline]
  fn intersect<'a>(self, iterable: &'a impl Iterable<Item<'a> = &'a (Key, Value)>) -> Self
  where
    Key: Eq + Hash + 'a,
    Value: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let retained: HashSet<(&Key, &Value)> = HashSet::from_iter(iterable.iterator().map(|(k, v)| (k, v)));
    self.into_iter().filter(|(k, v)| retained.contains(&(k, v))).collect()
  }

  /// Creates a new map by applying the given closure `function` to each entry in
  /// the original map.
  ///
  /// The closure `function` takes a reference to an entry of type
  /// `(Key, Value)` and returns a value of type `(L, W)`.
  /// The resulting other are collected into a new map of the same type.
  ///
  /// This is a consuming variant of [`map_to`].
  ///
  /// [`map_to`]: Map::map_to
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let mapped = a.map(|(&k, &v)| (k, k + v.len()));
  ///
  /// assert_eq!(mapped, HashMap::from([
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]));
  /// ```
  fn map<L, W>(&self, function: impl FnMut((&Key, &Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self::This<L, W>: FromIterator<(L, W)>;

  /// Creates a new map by applying the given closure `function` to each entry in
  /// the original map.
  ///
  /// The closure `function` takes a reference to an entry of type
  /// `(Key, Value)` and returns a value of type `(L, W)`.
  /// The resulting other are collected into a new map of the same type.
  ///
  /// This is a consuming variant of [`map`].
  ///
  /// [`map`]: Map::map
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let mapped = a.map_to(|(k, v)| (k, k + v.len()));
  ///
  /// assert_eq!(mapped, HashMap::from([
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]));
  /// ```
  #[inline]
  fn map_to<L, W>(self, function: impl FnMut((Key, Value)) -> (L, W)) -> Self::This<L, W>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
    Self::This<L, W>: FromIterator<(L, W)>,
  {
    self.into_iter().map(function).collect()
  }

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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let mapped = a.map_keys(|&k| k + 1);
  ///
  /// assert_eq!(mapped, HashMap::from([
  ///   (2, "a"),
  ///   (3, "b"),
  ///   (4, "c"),
  /// ]));
  /// ```
  #[inline]
  fn map_keys<L: Eq + Hash>(self, mut function: impl FnMut(&Key) -> L) -> Self::This<L, Value>
  where
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let mapped = a.map_values(|v| v.len());
  ///
  /// assert_eq!(mapped, HashMap::from([
  ///   (1, 1),
  ///   (2, 1),
  ///   (3, 1),
  /// ]));
  /// ```
  #[inline]
  fn map_values<W: Eq + Hash>(self, mut function: impl FnMut(&Value) -> W) -> Self::This<Key, W>
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
  ///   (0, "a"),
  ///   (3, "b"),
  ///   (-3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.max_by(|x, y| x.0.cmp(y.0)), Some((&3, &"b")));
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
  ///   (0_i32, "a"),
  ///   (3, "b"),
  ///   (-5, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.max_by_key(|(k, _)| k.abs()), Some((&-5, &"c")));
  ///
  /// assert_eq!(e.max_by_key(|(k, _)| k.abs()), None);
  /// ```
  fn max_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>;

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
  ///   (0, 1),
  ///   (1, 2),
  ///   (2, 3),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// assert_eq!(a.max_item(), Some((&2, &3)));
  ///
  /// assert_eq!(e.max_item(), None);
  /// ```
  #[inline]
  fn max_item(&self) -> Option<(&Key, &Value)>
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
  ///   (0, "a"),
  ///   (3, "b"),
  ///   (-5, "c"),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// assert_eq!(a.min_by(|x, y| x.0.cmp(y.0)), Some((&-5, &"c")));
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
  ///   (0_i32, "a"),
  ///   (3, "b"),
  ///   (-5, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.min_by_key(|(k, _)| k.abs()), Some((&0, &"a")));
  ///
  /// assert_eq!(e.min_by_key(|(k, _)| k.abs()), None);
  /// ```
  fn min_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<(&Key, &Value)>;

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
  ///   (0, 1),
  ///   (1, 2),
  ///   (2, 3),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// assert_eq!(a.min_item(), Some((&0, &1)));
  ///
  /// assert_eq!(e.min_item(), None);
  /// ```
  #[inline]
  fn min_item(&self) -> Option<(&Key, &Value)>
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
  ///   (0, "a"),
  ///   (3, "b"),
  ///   (-5, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.minmax_by(|x, y| x.0.cmp(y.0)), Some(((&-5, &"c"), (&3, &"b"))));
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
  ///   (0_i32, "a"),
  ///   (3, "b"),
  ///   (-5, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert_eq!(a.minmax_by_key(|(k, _)| k.abs()), Some(((&0, &"a"), (&-5, &"c"))));
  /// assert_eq!(e.minmax_by_key(|(k, _)| k.abs()), None);
  /// ```
  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut((&Key, &Value)) -> K) -> Option<((&Key, &Value), (&Key, &Value))>;

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
  ///   (0, 1),
  ///   (1, 2),
  ///   (2, 3),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// assert_eq!(a.minmax_item(), Some(((&0, &1), (&2, &3))));
  ///
  /// assert_eq!(e.minmax_item(), None);
  /// ```
  #[inline]
  fn minmax_item(&self) -> Option<((&Key, &Value), (&Key, &Value))>
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let (even, odd) = a.partition(|(&k, _)| k % 2 == 0);
  ///
  /// assert_eq!(even, HashMap::from([
  ///   (2, "b"),
  /// ]));
  /// assert_eq!(odd, HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
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
  /// This is a non-consuming variant of [`partition_map_to`].
  ///
  /// [`partition_map_to`]: Map::partition_map_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let (even, odd) = a.partition_map(|(&k, &v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  ///
  /// assert_eq!(even, HashMap::from([
  ///   (5, "b"),
  /// ]));
  /// assert_eq!(odd, HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
  /// ]));
  /// ```
  fn partition_map<L1, W1, L2, W2>(
    &self, function: impl FnMut((&Key, &Value)) -> Result<(L1, W1), (L2, W2)>,
  ) -> (Self::This<L1, W1>, Self::This<L2, W2>)
  where
    Self::This<L1, W1>: Default + Extend<(L1, W1)>,
    Self::This<L2, W2>: Default + Extend<(L2, W2)>;

  /// Creates two new maps with arbitrary entry types from the original map
  /// by applying specified function.
  ///
  /// The function passed to `partition_map_to()` can return `Ok`, or `Err`.
  /// `partition_map_to()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a consuming variant of [`partition_map`].
  ///
  /// [`partition_map`]: Map::partition_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// let (even, odd) = a.partition_map_to(|(k, v)| if k % 2 == 0 { Ok((k + 3, v)) } else { Err((k, v)) });
  ///
  /// assert_eq!(even, HashMap::from([
  ///   (5, "b"),
  /// ]));
  /// assert_eq!(odd, HashMap::from([
  ///   (1, "a"),
  ///   (3, "c"),
  /// ]));
  /// ```
  fn partition_map_to<L1, W1, L2, W2>(
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

  /// Iterates over the entire map, multiplying all the keys
  ///
  /// An empty map returns the one value of the type.
  ///
  /// `product()` can be used to multiply any type implementing [`Product`],
  ///
  /// [`Product`]: Product
  ///
  /// # Panics
  ///
  /// When calling `product()` and a primitive integer type is being returned,
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
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// let product = a.product_keys();
  ///
  /// assert_eq!(product, 6);
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
  /// `product()` can be used to multiply any type implementing [`Product`],
  ///
  /// [`Product`]: Product
  ///
  /// # Panics
  ///
  /// When calling `product()` and a primitive integer type is being returned,
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
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// let product = a.product_values();
  ///
  /// assert_eq!(product, 24);
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
  /// This is a non-consuming variant of [`reduce_to`].
  ///
  /// [`fold()`]: Map::fold
  /// [`reduce_to()`]: Map::reduce_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let source = HashMap::from([
  /// #   (1, 2),
  /// #   (2, 3),
  /// #   (3, 4),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]);
  ///
  /// let reduced = a.reduce(|(&a, &b), (&k, &v)| (a + k, b + v)).unwrap();
  ///
  /// assert_eq!(reduced, (6, 9));
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// # let a = source.clone();
  /// let folded = a.fold((0, 0), |(a, b), (&k, &v)| (a + k, b + v));
  ///
  /// assert_eq!(reduced, folded);
  /// ```
  fn reduce(self, function: impl FnMut((&Key, &Value), (&Key, &Value)) -> (Key, Value)) -> Option<(Key, Value)>;

  /// Reduces the elements to a single one, by repeatedly applying a reducing
  /// operation.
  ///
  /// If the collection is empty, returns [`None`]; otherwise, returns the
  /// result of the reduction.
  ///
  /// The reducing function is a closure with two arguments: an 'accumulator', and an element.
  /// For collections with at least one element, this is the same as [`fold_to()`]
  /// with the first element of the collection as the initial accumulator value, folding
  /// every subsequent element into it.
  ///
  /// This is a consuming variant of [`reduce`].
  ///
  /// [`fold_to()`]: Map::fold_to
  /// [`reduce()`]: Map::reduce
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// # let source = HashMap::from([
  /// #   (1, 2),
  /// #   (2, 3),
  /// #   (3, 4),
  /// # ]);
  /// let a = HashMap::from([
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]);
  ///
  /// let reduced = a.reduce_to(|(a, b), (k, v)| (a + k, b + v)).unwrap();
  ///
  /// assert_eq!(reduced, (6, 9));
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// # let a = source.clone();
  /// let folded = a.fold_to((0, 0), |(a, b), (k, v)| (a + k, b + v));
  ///
  /// assert_eq!(reduced, folded);
  /// ```
  fn reduce_to(self, mut function: impl FnMut((Key, Value), (Key, Value)) -> (Key, Value)) -> Option<(Key, Value)>
  where
    Self: IntoIterator<Item = (Key, Value)> + Sized,
  {
    let mut iterator = self.into_iter();
    iterator.next().and_then(|value1| {
      iterator.next().map(|value2| iterator.fold(function(value1, value2), |(k, v), x| function((k, v), x)))
    })
  }

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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.replace(&3, 4, "d"), HashMap::from([
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (4, "d"),
  /// ]));
  /// ```
  #[inline]
  fn replace(self, value: &Key, replacement_key: Key, replacement_value: Value) -> Self
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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  ///
  /// assert_eq!(a.replace_all(&vec![2, 3], vec![(4, "d"), (5, "e")]), HashMap::from([
  ///   (1, "a"),
  ///   (4, "d"),
  ///   (5, "e"),
  /// ]));
  /// ```
  #[inline]
  fn replace_all<'a>(
    self, elements: &'a impl Iterable<Item<'a> = &'a Key>, replacement: impl IntoIterator<Item = (Key, Value)>,
  ) -> Self
  where
    Key: Eq + Hash + 'a,
    Self: IntoIterator<Item = (Key, Value)> + FromIterator<(Key, Value)>,
  {
    let iterator = elements.iterator();
    let removed: HashSet<&Key> = HashSet::from_iter(iterator);
    self.into_iter().filter(|x| !removed.contains(&x.0)).chain(replacement).collect()
  }

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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.subset(&vec![4, 3, 2, 2, 1]));
  /// assert!(e.subset(&vec![1]));
  ///
  /// assert!(!a.subset(&vec![1, 2]));
  /// assert!(!a.subset(&vec![]));
  /// ```
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

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
  ///   (1, "a"),
  ///   (2, "b"),
  ///   (3, "c"),
  /// ]);
  /// let e: HashMap<i32, &str> = HashMap::new();
  ///
  /// assert!(a.superset(&vec![3, 1]));
  /// assert!(a.superset(&vec![]));
  ///
  /// assert!(!a.superset(&vec![1, 2, 3, 4]));
  /// assert!(!a.superset(&vec![1, 2, 2]));
  /// assert!(!a.superset(&vec![3, 4]));
  /// assert!(!e.superset(&vec![1]));
  /// ```
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Key>) -> bool
  where
    Key: Eq + Hash + 'a;

  /// Sums keys of this map.
  ///
  /// Takes each key, adds them together, and returns the result.
  ///
  /// An empty map returns the zero value of the type.
  ///
  /// `sum()` can be used to multiply any type implementing [`Sum`],
  ///
  /// [`Sum`]: Sum
  ///
  /// # Panics
  ///
  /// When calling `sum()` and a primitive integer type is being returned, this
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
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// let sum = a.sum_keys();
  ///
  /// assert_eq!(sum, 6);
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
  /// `sum()` can be used to multiply any type implementing [`Sum`],
  ///
  /// [`Sum`]: Sum
  ///
  /// # Panics
  ///
  /// When calling `sum()` and a primitive integer type is being returned, this
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
  ///   (1, 2),
  ///   (2, 3),
  ///   (3, 4),
  /// ]);
  /// let e: HashMap<i32, i32> = HashMap::new();
  ///
  /// let sum = a.sum_values();
  ///
  /// assert_eq!(sum, 9);
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

  /// Creates a new map containing a single element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let unit = HashMap::unit(1, "a");
  ///
  /// assert_eq!(unit, HashMap::from([
  ///   (1, "a"),
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
  match iterator.next() {
    Some(item) => {
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
      Some((min, max))
    }
    None => None,
  }
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
