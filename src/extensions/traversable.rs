use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::Iterable;

/// Non-consuming collection operations.
///
/// Methods have the following properties:
///
/// - Does not consume the collection or its elements
/// - Does not create a new collection
///
pub trait Traversable<Item> {
  /// Tests if every element of the collection matches a predicate.
  ///
  /// `all()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and if they all return
  /// `true`, then so does `all()`. If any of them return `false`, it
  /// returns `false`.
  ///
  /// `all()` is short-circuiting; in other words, it will stop processing
  /// as soon as it finds a `false`, given that no matter what else happens,
  /// the result will also be `false`.
  ///
  /// An empty collection returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert!(a.all(|&x| x > 0));
  /// assert!(e.all(|&x| x > 0));
  ///
  /// assert!(!a.all(|&x| x > 2));
  /// ```
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  /// Tests if any element of the collection matches a predicate.
  ///
  /// `any()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and if any of them return
  /// `true`, then so does `any()`. If they all return `false`, it
  /// returns `false`.
  ///
  /// `any()` is short-circuiting; in other words, it will stop processing
  /// as soon as it finds a `true`, given that no matter what else happens,
  /// the result will also be `true`.
  ///
  /// An empty collection returns `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert!(a.any(|&x| x > 0));
  ///
  /// assert!(!a.any(|&x| x > 5));
  /// assert!(!e.any(|&x| x > 0));
  /// ```
  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  /// Counts elements of this collection that satisfy a predicate.
  ///
  /// `count_by()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and counts those which
  /// return `true`, disregarding those which return `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.count_by(|&x| x == 2), 1);
  /// assert_eq!(a.count_by(|&x| x == 5), 0);
  /// ```
  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize;

  /// Tests this collection and  another collection have no elements in common.
  ///
  /// Returns `true` if aby of the collections are empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert!(a.disjoint(&vec![4, 5]));
  /// assert!(a.disjoint(&vec![]));
  ///
  /// assert!(!a.disjoint(&vec![3, 4]));
  /// ```
  fn disjoint<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a;

  /// Searches for an element of this collection that satisfies a predicate.
  ///
  /// `find()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and if any of them return
  /// `true`, then `find()` returns [`Some(element)`]. If they all return
  /// `false`, it returns [`None`].
  ///
  /// `find()` is short-circuiting; in other words, it will stop processing
  /// as soon as the closure returns `true`.
  ///
  /// If you need the index of the element, see [`position()`].
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.find(|&x| x % 2 == 1), Some(&1));
  ///
  /// assert_eq!(a.find(|&x| x == 5), None);
  /// ```
  fn find(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  /// Applies function to the elements of this collection and returns
  /// the first non-none result.
  ///
  /// `find_map` can be used to make chains of [`find`] and [`map`] more
  /// concise.
  ///
  /// `find_map_to(f)` is equivalent to `find().map()`.
  ///
  /// This is a non-consuming variant of [`find_map_to`].
  ///
  /// [`find`]: Traversable::find
  /// [`map`]: Traversable::map
  /// [`find_map_to`]: crate::Collectible::find_map_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = ["lol", "NaN", "2", "5"];
  ///
  /// let first_number = a.find_map(|s| s.parse().ok());
  ///
  /// assert_eq!(first_number, Some(2));
  /// ```
  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>;

  /// Folds every element into an accumulator by applying an operation,
  /// returning the final result.
  ///
  /// `fold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of the collection, `fold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// This is a non-consuming variant of [`fold_to`].
  ///
  /// Note: [`reduce()`] can be used to use the first element as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold()` combines elements in a *left-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *right-associative* version of `fold()`, see [`rfold()`].
  ///
  /// [`fold_to`]: crate::Collectible::fold_to
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // the sum of all the elements of the array
  /// let sum = a.fold(0, |acc, x| acc + x);
  ///
  /// assert_eq!(sum, 6);
  /// ```
  ///
  /// Let's walk through each step of the iteration here:
  ///
  /// | element | acc | x | result |
  /// |---------|-----|---|--------|
  /// |         | 0   |   |        |
  /// | 1       | 0   | 1 | 1      |
  /// | 2       | 1   | 2 | 3      |
  /// | 3       | 3   | 3 | 6      |
  ///
  /// And so, our final result, `6`.
  ///
  /// This example demonstrates the left-associative nature of `fold()`:
  /// it builds a string, starting with an initial value
  /// and continuing with each element from the front until the back:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let numbers = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// let result = numbers.fold(zero, |acc, &x| {
  ///   format!("({acc} + {x})")
  /// });
  ///
  /// assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");
  /// ```
  /// It's common for people who haven't used collections a lot to
  /// use a `for` loop with a list of things to build up a result. Those
  /// can be turned into `fold()`s:
  ///
  /// [`for`]: ../../book/ch03-05-control-flow.html#looping-through-a-collection-with-for
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let numbers = vec![1, 2, 3, 4, 5];
  ///
  /// let mut result = 0;
  ///
  /// // for loop:
  /// for i in &numbers {
  ///   result = result + i;
  /// }
  ///
  /// // fold:
  /// let result2 = numbers.fold(0, |acc, &x| acc + x);
  ///
  /// // they're the same
  /// assert_eq!(result, result2);
  /// ```
  fn fold<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B;

  /// Calls a closure on each element of this collection.
  ///
  /// This is equivalent to using a [`for`] loop on the collection, although
  /// `break` and `continue` are not possible from a closure. It's generally
  /// more idiomatic to use a `for` loop, but `for_each` may be more legible
  /// when processing items at the end of longer collection chains.
  ///
  /// [`for`]: ../../book/ch03-05-control-flow.html#looping-through-a-collection-with-for
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
  /// might be preferable to keep a functional style with longer collections:
  ///
  /// ```
  /// (0..5).flat_map(|x| x * 100 .. x * 110)
  ///       .enumerate()
  ///       .filter(|&(i, x)| (i + x) % 3 == 0)
  ///       .for_each(|(i, x)| println!("{i}:{x}"));
  /// ```
  fn for_each(&self, function: impl FnMut(&Item));

  /// Creates `HashMap` of keys mapped and folded to values according to
  /// specified discriminator and folding operation functions.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  /// The folding operation takes an accumulator and a closure and returns a new element.
  /// The closure returns the value that the accumulator should have for the next iteration.
  ///
  /// This is a consuming variant of [`group_fold_to`].
  ///
  /// [`group_fold_to`]: Collectible::group_fold_to
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let group_folded = a.group_fold(|x| x % 2, 0, |acc, &x| acc + x);
  ///
  /// assert_eq!(group_folded, HashMap::from([
  ///   (0, 2),
  ///   (1, 4),
  /// ]));
  /// ```
  fn group_fold<K, B>(
    &self, to_key: impl FnMut(&Item) -> K, initial_value: B, function: impl FnMut(B, &Item) -> B,
  ) -> HashMap<K, B>
  where
    K: Eq + Hash,
    B: Clone;

  /// Creates `HashMap` of keys mapped and reduced to values according to
  /// specified discriminator and reducing operation functions.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  /// The reducing operation takes an accumulator and a closure and returns a new element.
  /// The closure returns the value that the accumulator should have for the next iteration.
  ///
  /// This is a non-consuming variant of [`group_reduce`].
  ///
  /// [`group_reduce_to()`]: crate::Collectible::group_reduce_to
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let group_reduced = a.group_reduce(|x| x % 2, |acc, x| acc + x);
  ///
  /// assert_eq!(group_reduced, HashMap::from([
  ///   (0, 2),
  ///   (1, 4),
  /// ]));
  /// ```
  fn group_reduce<K>(
    &self, to_key: impl FnMut(&Item) -> K, function: impl FnMut(&Item, &Item) -> Item,
  ) -> HashMap<K, Item>
  where
    K: Eq + Hash,
    Item: Clone;

  /// Returns the element that gives the maximum value with respect to the
  /// specified comparison function.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.max_by(|x, y| x.cmp(y)), Some(&5));
  ///
  /// assert_eq!(e.max_by(|x, y| x.cmp(y)), None);
  /// ```
  fn max_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the element that gives the maximum value from the
  /// specified key function.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.max_by_key(|x| x.abs()), Some(&-10));
  ///
  /// assert_eq!(e.max_by_key(|x| x.abs()), None);
  /// ```
  fn max_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<&Item>;

  /// Returns the maximum element of this collection.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
  /// incomparable. You can work around this by using [`Collectible::reduce`]:
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(
  ///     vec![2.4, f32::NAN, 1.3]
  ///         .reduce_to(f32::max)
  ///         .unwrap(),
  ///     2.4
  /// );
  /// ```
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<u32> = vec![];
  ///
  /// assert_eq!(a.max_of(), Some(&3));
  ///
  /// assert_eq!(e.max_of(), None);
  /// ```
  #[inline]
  fn max_of(&self) -> Option<&Item>
  where
    Item: Ord,
  {
    self.max_by(Ord::cmp)
  }

  /// Returns the element that gives the minimum value with respect to the
  /// specified comparison function.
  ///
  /// If several elements are equally minimum, the first element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.min_by(|x, y| x.cmp(y)), Some(&-10));
  ///
  /// assert_eq!(e.min_by(|x, y| x.cmp(y)), None);
  /// ```
  fn min_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the element that gives the minimum value from the
  /// specified key function.
  ///
  /// If several elements are equally minimum, the fist element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.min_by_key(|x| x.abs()), Some(&0));
  ///
  /// assert_eq!(e.min_by_key(|x| x.abs()), None);
  /// ```
  fn min_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<&Item>;

  /// Returns the minimum element of this collection.
  ///
  /// If several elements are equally minimum, the first element is returned.
  /// If the collection is empty, [`None`] is returned.
  ///
  /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
  /// incomparable. You can work around this by using [`Collectible::reduce`]:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(
  ///   vec![2.4, f32::NAN, 1.3]
  ///     .reduce_to(f32::min)
  ///     .unwrap(),
  ///   1.3
  /// );
  /// ```
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.min_of(), Some(&1));
  ///
  /// assert_eq!(e.min_of(), None);
  /// ```
  #[inline]
  fn min_of(&self) -> Option<&Item>
  where
    Item: Ord,
  {
    self.min_by(Ord::cmp)
  }

  /// Returns the minimum and maximum element of this collection with respect to the
  /// specified comparison function.
  ///
  /// For the minimum, the first minimal element is returned. For the maximum,
  /// the last maximal element is returned. If the collection is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.minmax_by(|x, y| x.cmp(y)), Some((&-10, &5)));
  /// assert_eq!(e.minmax_by(|x, y| x.cmp(y)), None);
  /// ```
  fn minmax_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<(&Item, &Item)>;

  /// Returns the minimum and maximum element of this collection from the
  /// specified key function.
  ///
  /// For the minimum, the first minimal element is returned. For the maximum,
  /// the last maximal element is returned. If the collection is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.minmax_by_key(|x| x.abs()), Some((&0, &-10)));
  /// assert_eq!(e.minmax_by_key(|x| x.abs()), None);
  /// ```
  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<(&Item, &Item)>;

  /// Return the minimum and maximum element of this collection.
  ///
  /// For the minimum, the first minimal element is returned. For the maximum,
  /// the last maximal element is returned. If the collection is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.minmax_of(), Some((&-10, &5)));
  /// assert_eq!(e.minmax_of(), None);
  /// ```
  #[inline]
  fn minmax_of(&self) -> Option<(&Item, &Item)>
  where
    Item: Ord,
  {
    self.minmax_by(Ord::cmp)
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
  ///
  /// This is a non-consuming variant of [`reduce_to`].
  ///
  /// [`fold()`]: Traversable::fold
  /// [`reduce_to()`]: crate::Collectible::reduce
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// let reduced = a.reduce(|&acc, &e| acc + e).unwrap();
  ///
  /// assert_eq!(reduced, 6);
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// # let a = a_source.clone();
  /// let folded = a.fold(0, |acc, &e| acc + e);
  ///
  /// assert_eq!(reduced, folded);
  /// ```
  fn reduce(&self, function: impl FnMut(&Item, &Item) -> Item) -> Option<Item>;

  /// Tests if another collection contains all elements of this collection
  /// at least as many times as their appear in this collection.
  ///
  /// To obtain set-like semantics for sequences which only considers unique elements,
  /// use `.unique().subset()`.
  ///
  /// Returns `true` if this collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert!(a.subset(&vec![4, 3, 2, 2, 1]));
  /// assert!(e.subset(&vec![1]));
  /// assert!(e.subset(&vec![]));
  ///
  /// assert!(!a.subset(&vec![1, 2, 3]));
  /// assert!(!a.subset(&vec![3, 4]));
  /// ```
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a;

  /// Tests if this collection contains all elements of another collection
  /// at least as many times as their appear in the other collection.
  ///
  /// To obtain set-like semantics for sequences which only considers unique elements,
  /// use `.unique().superset()`.
  ///
  /// Returns `true` if the other collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert!(a.superset(&vec![3, 1]));
  /// assert!(a.superset(&vec![2, 2, 1]));
  /// assert!(a.superset(&vec![]));
  ///
  /// assert!(!a.superset(&vec![1, 1, 2]));
  /// assert!(!a.superset(&vec![3, 4]));
  /// assert!(!e.superset(&vec![1]));
  /// ```
  fn superset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a;
}

#[inline]
pub(crate) fn all<'a, Item: 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, predicate: impl FnMut(&Item) -> bool,
) -> bool {
  iterator.all(predicate)
}

#[inline]
pub(crate) fn any<'a, Item: 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, predicate: impl FnMut(&Item) -> bool,
) -> bool {
  iterator.any(predicate)
}

#[inline]
pub(crate) fn count_by<'a, Item: 'a>(
  iterator: impl Iterator<Item = &'a Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> usize {
  iterator.filter(|&x| predicate(x)).count()
}

pub(crate) fn disjoint<'a, Item: Eq + Hash + 'a>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool {
  let mut occurred = HashSet::with_capacity(iterator.size_hint().0);
  for item in iterator {
    let _ = occurred.insert(item);
  }
  if occurred.is_empty() {
    return true;
  }
  for item in elements.iterator() {
    if occurred.contains(&item) {
      return false;
    }
  }
  true
}

pub(crate) fn frequencies<'a, Item: Eq + Hash + 'a>(
  iterator: impl Iterator<Item = &'a Item>,
) -> HashMap<&'a Item, usize> {
  let mut result = HashMap::with_capacity(iterator.size_hint().0);
  for item in iterator {
    *result.entry(item).or_default() += 1;
  }
  result
}

pub(crate) fn group_fold<'a, Item: 'a, K: Eq + Hash, B: Clone>(
  iterator: impl Iterator<Item = &'a Item>, mut to_key: impl FnMut(&Item) -> K, initial_value: B,
  mut function: impl FnMut(B, &Item) -> B,
) -> HashMap<K, B> {
  let mut result = HashMap::with_capacity(iterator.size_hint().0);
  for item in iterator {
    let key = to_key(item);
    let new_value = match result.remove(&key) {
      Some(value) => function(value, item),
      None => function(initial_value.clone(), item),
    };
    let _unused = result.insert(key, new_value);
  }
  result
}

pub(crate) fn group_reduce<'a, Item: Clone + 'a, K: Eq + Hash>(
  iterator: impl Iterator<Item = &'a Item>, mut to_key: impl FnMut(&Item) -> K,
  mut function: impl FnMut(&Item, &Item) -> Item,
) -> HashMap<K, Item> {
  let mut result = HashMap::with_capacity(iterator.size_hint().0);
  for item in iterator {
    let key = to_key(item);
    let new_value = match result.remove(&key) {
      Some(value) => function(&value, item),
      None => item.clone(),
    };
    let _unused = result.insert(key, new_value);
  }
  result
}

pub(crate) fn minmax_by<'a, Item: 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, mut compare: impl FnMut(&Item, &Item) -> Ordering,
) -> Option<(&'a Item, &'a Item)> {
  match iterator.next() {
    Some(item) => {
      let mut min = item;
      let mut max = min;
      for item in iterator {
        if compare(item, min) == Ordering::Less {
          min = item;
        }
        if compare(item, max) != Ordering::Less {
          max = item;
        }
      }
      Some((min, max))
    }
    None => None,
  }
}

#[inline]
pub(crate) fn minmax_by_key<'a, Item: 'a, K: Ord>(
  iterator: impl Iterator<Item = &'a Item>, mut to_key: impl FnMut(&Item) -> K,
) -> Option<(&'a Item, &'a Item)> {
  minmax_by(iterator, |x, y| to_key(x).cmp(&to_key(y)))
}

#[inline]
pub(crate) fn reduce<'a, Item: 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, mut function: impl FnMut(&Item, &Item) -> Item,
) -> Option<Item> {
  iterator
    .next()
    .and_then(|value1| iterator.next().map(|value2| iterator.fold(function(value1, value2), |r, x| function(&r, x))))
}

pub(crate) fn subset<'a, Item: Eq + Hash + 'a>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool {
  let mut counts = frequencies(iterator);
  if counts.is_empty() {
    return true;
  }
  for item in elements.iterator() {
    if let Some(count) = counts.get_mut(item) {
      *count -= 1;
      if *count == 0 {
        let _ = counts.remove(item);
        if counts.is_empty() {
          return true;
        }
      }
    }
  }
  false
}

pub(crate) fn superset<'a, Item: Eq + Hash + 'a>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool {
  let mut counts = frequencies(elements.iterator());
  if counts.is_empty() {
    return true;
  }
  for item in iterator {
    if let Some(count) = counts.get_mut(item) {
      *count -= 1;
      if *count == 0 {
        let _ = counts.remove(item);
        if counts.is_empty() {
          return true;
        }
      }
    }
  }
  false
}
