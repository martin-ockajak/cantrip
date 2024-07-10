use std::cmp::Ordering;
use std::collections::HashSet;
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
  /// let e: Vec<i32> = Vec::new();
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
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert!(a.any(|&x| x > 0));
  ///
  /// assert!(!a.any(|&x| x > 5));
  /// assert!(!e.any(|&x| x > 0));
  /// ```
  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  /// Counts elements of a collection that satisfy a predicate.
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

  /// Searches for an element of a collection that satisfies a predicate.
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
  /// assert_eq!(a.find(|&x| x == 2), Some(&2));
  ///
  /// assert_eq!(a.find(|&x| x == 5), None);
  /// ```
  fn find(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

  /// Applies function to the elements of a collection and returns
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
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.max_by(|x, y| x.cmp(y)), Some(&5));
  ///
  /// assert_eq!(e.max_by(|x, y| x.cmp(y)), None);
  /// ```
  fn max_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the element that gives the maximum value from the
  /// specified function.
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
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.max_by_key(|x| x.abs()), Some(&-10));
  ///
  /// assert_eq!(e.max_by_key(|x| x.abs()), None);
  /// ```
  fn max_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<&Item>;

  /// Returns the maximum element of a collection.
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
  ///         .reduce(f32::max)
  ///         .unwrap(),
  ///     2.4
  /// );
  /// ```
  ///
  /// # Example
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<u32> = Vec::new();
  ///
  /// assert_eq!(a.max_item(), Some(&3));
  ///
  /// assert_eq!(e.max_item(), None);
  /// ```
  #[inline]
  fn max_item(&self) -> Option<&Item>
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
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.min_by(|x, y| x.cmp(y)), Some(&-10));
  ///
  /// assert_eq!(e.min_by(|x, y| x.cmp(y)), None);
  /// ```
  fn min_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the element that gives the minimum value from the
  /// specified function.
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
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.min_by_key(|x| x.abs()), Some(&0));
  ///
  /// assert_eq!(e.min_by_key(|x| x.abs()), None);
  /// ```
  fn min_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<&Item>;

  /// Returns the minimum element of a collection.
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
  ///     .reduce(f32::min)
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
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.min_item(), Some(&1));
  ///
  /// assert_eq!(e.min_item(), None);
  /// ```
  #[inline]
  fn min_item(&self) -> Option<&Item>
  where
    Item: Ord,
  {
    self.min_by(Ord::cmp)
  }

  /// Returns the minimum and maximum element of a collection with respect to the
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
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.minmax_by(|x, y| x.cmp(y)), Some((&-10, &5)));
  /// assert_eq!(e.minmax_by(|x, y| x.cmp(y)), None);
  /// ```
  fn minmax_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<(&Item, &Item)>;

  /// Returns the minimum and maximum element of a collection from the
  /// specified function.
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
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.minmax_by_key(|x| x.abs()), Some((&0, &-10)));
  /// assert_eq!(e.minmax_by_key(|x| x.abs()), None);
  /// ```
  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<(&Item, &Item)>;

  /// Return the minimum and maximum element of a collection.
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
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.minmax_item(), Some((&-10, &5)));
  /// assert_eq!(e.minmax_item(), None);
  /// ```
  #[inline]
  fn minmax_item(&self) -> Option<(&Item, &Item)>
  where
    Item: Ord,
  {
    self.minmax_by(Ord::cmp)
  }

  /// Tests if all the elements of a collection can be found in another collection.
  ///
  /// Returns `true` if this collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert!(a.subset(&vec![2, 1, 3, 4]));
  /// assert!(e.subset(&vec![2, 1, 3, 4]));
  ///
  /// assert!(!a.subset(&vec![2, 1]));
  /// assert!(!a.subset(&vec![]));
  /// ```
  fn subset<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> bool
  where
    Item: Eq + Hash + 'a;

  /// Tests if all the elements of another collection can be found in this collection.
  ///
  /// Returns `true` if the other collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert!(a.superset(&vec![2, 1]));
  /// assert!(a.superset(&vec![]));
  ///
  /// assert!(!a.superset(&vec![2, 1, 3, 4]));
  /// assert!(!e.superset(&vec![2, 1]));
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
pub(crate) fn all_equal<'a, Item: PartialEq + 'a>(mut iterator: impl Iterator<Item = &'a Item>) -> bool {
  match iterator.next() {
    Some(head) => iterator.all(|x| x == head),
    None => true,
  }
}

#[inline]
pub(crate) fn all_unique<'a, Item: Eq + Hash + 'a>(mut iterator: impl Iterator<Item = &'a Item>) -> bool {
  let (size, _) = iterator.size_hint();
  let mut items = HashSet::with_capacity(size);
  iterator.all(|x| items.insert(x))
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

pub(crate) fn subset<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool
where
  Item: Eq + Hash + 'a,
{
  let elements_iterator = elements.iterator();
  let occurred: HashSet<&Item> = HashSet::from_iter(elements_iterator);
  for item in iterator {
    if !occurred.contains(item) {
      return false;
    }
  }
  true
}

pub(crate) fn superset<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> bool
where
  Item: Eq + Hash + 'a,
{
  let elements_iterator = elements.iterator();
  let occurred: HashSet<&Item> = HashSet::from_iter(iterator);
  for item in elements_iterator {
    if !occurred.contains(item) {
      return false;
    }
  }
  true
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
