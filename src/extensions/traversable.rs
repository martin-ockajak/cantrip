#![deny(missing_docs)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;

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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert!(a.all(|&x| x > 0));
  ///
  /// assert!(!a.all(|&x| x > 2));
  /// ```
  fn all(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  /// Tests if all elements of the collection are equal.
  ///
  /// `all_equal()` returns `true` if all elements of the collection are equal
  /// and `false` if a pair of unequal elements exist.
  ///
  /// An empty collection returns `true`.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 1, 1];
  /// let b = vec![1, 2, 3];
  ///
  /// assert!(a.all_equal());
  ///
  /// assert!(!b.all_equal());
  /// ```
  fn all_equal(&self) -> bool
  where
    Item: PartialEq;

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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert!(a.any(|&x| x > 0));
  ///
  /// assert!(!a.any(|&x| x > 5));
  /// ```
  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  /// Counts elements of a collection that satisfy a predicate.
  ///
  /// `count_by()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and counts those which
  /// return `true`, disregarding those which return `false`.
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
  /// assert_eq!(a.count_by(|&x| x == 2), 1);
  ///
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
  /// # Examples
  ///
  /// Basic usage:
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
  /// `find_map(f)` is equivalent to `find().map()`.
  ///
  /// [`find`]: Traversable::find
  /// [`map`]: Traversable::map
  ///
  /// # Examples
  ///
  /// ```
  /// let a = ["lol", "NaN", "2", "5"];
  ///
  /// let first_number = a.iter().find_map(|s| s.parse().ok());
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
  /// Note: [`reduce()`] can be used to use the first element as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold()` combines elements in a *left-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *right-associative* version of `fold()`, see [`rfold()`].
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = [1, 2, 3];
  ///
  /// // the sum of all of the elements of the array
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
  /// let numbers = [1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// let result = numbers.fold(zero, |acc, &x| {
  ///     format!("({acc} + {x})")
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
  /// let numbers = [1, 2, 3, 4, 5];
  ///
  /// let mut result = 0;
  ///
  /// // for loop:
  /// for i in &numbers {
  ///     result = result + i;
  /// }
  ///
  /// // fold:
  /// let result2 = numbers.fold(0, |acc, &x| acc + x);
  ///
  /// // they're the same
  /// assert_eq!(result, result2);
  /// ```
  fn fold<B>(&self, initial_value: B, function: impl FnMut(B, &Item) -> B) -> B;

  /// Combine all collection elements into one `String`, separated by `sep`.
  ///
  /// Use the `Display` implementation of each element.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.join_items(", "), "1, 2, 3");
  /// ```
  fn join_items(&self, separator: &str) -> String
  where
    Item: Display;

  // FIXME - implement
  // fn includes(&self, iterable: impl IntoIterator<Item = Item>) -> bool;

  /// Returns the element that gives the maximum value with respect to the
  /// specified comparison function.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.max_by(|x, y| x.cmp(y)).unwrap(), &5);
  /// ```
  fn max_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the element that gives the maximum value from the
  /// specified function.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.max_by_key(|x| x.abs()).unwrap(), &-10);
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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let b: Vec<u32> = Vec::new();
  ///
  /// assert_eq!(a.max_item(), Some(&3));
  /// assert_eq!(b.max_item(), None);
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
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.min_by(|x, y| x.cmp(y)).unwrap(), &-10);
  /// ```
  fn min_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the element that gives the minimum value from the
  /// specified function.
  ///
  /// If several elements are equally minimum, the fist element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.min_by_key(|x| x.abs()).unwrap(), &0);
  /// ```
  fn min_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<&Item>;

  /// Returns the minimum element of a collection.
  ///
  /// If several elements are equally minimum, the first element is returned.
  /// If the collection is empty, [`None`] is returned.
  ///
  /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
  /// incomparable. You can work around this by using [`Collectible::reduce`]:
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(
  ///     vec![2.4, f32::NAN, 1.3]
  ///         .reduce(f32::min)
  ///         .unwrap(),
  ///     1.3
  /// );
  /// ```
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let b: Vec<u32> = Vec::new();
  ///
  /// assert_eq!(a.min_item(), Some(&1));
  /// assert_eq!(b.min_item(), None);
  /// ```
  #[inline]
  fn min_item(&self) -> Option<&Item>
  where
    Item: Ord,
  {
    self.min_by(Ord::cmp)
  }

  /// Return the minimum and maximum element of a collection with respect to the
  /// specified comparison function.
  ///
  /// For the minimum, the first minimal element is returned. For the maximum,
  /// the last maximal element is returned. If the collection is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.minmax_by(|x, y| x.cmp(y)).unwrap(), (&-10, &5));
  /// ```
  fn minmax_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<(&Item, &Item)>;

  /// Return the minimum and maximum element of a collection from the
  /// specified function.
  ///
  /// For the minimum, the first minimal element is returned. For the maximum,
  /// the last maximal element is returned. If the collection is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.minmax_by_key(|x| x.abs()).unwrap(), (&0, &-10));
  /// ```
  fn minmax_by_key<K: Ord>(&self, to_key: impl FnMut(&Item) -> K) -> Option<(&Item, &Item)>;

  /// Return the minimum and maximum element of a collection.
  ///
  /// For the minimum, the first minimal element is returned. For the maximum,
  /// the last maximal element is returned. If the collection is empty, [`None`] is returned.
  /// This matches the behavior of the standard [`Iterator::min`] and [`Iterator::max`] methods.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(a.minmax_item().unwrap(), (&-10, &5));
  /// ```
  #[inline]
  fn minmax_item(&self) -> Option<(&Item, &Item)>
  where
    Item: Ord,
  {
    self.minmax_by(Ord::cmp)
  }
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
    None => false,
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
pub(crate) fn fold<'a, Item: 'a, B>(
  iterator: impl Iterator<Item = &'a Item>, init: B, function: impl FnMut(B, &Item) -> B,
) -> B {
  iterator.fold(init, function)
}

pub(crate) fn join_items<'a, Item: Display + 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, separator: &str,
) -> String {
  match iterator.next() {
    Some(item) => {
      let mut result = String::with_capacity(separator.len() * iterator.size_hint().0);
      write!(&mut result, "{}", item).unwrap();
      for item in iterator {
        result.push_str(separator);
        write!(&mut result, "{}", item).unwrap();
      }
      result.shrink_to_fit();
      result
    }
    None => String::new(),
  }
}

#[inline]
pub(crate) fn minmax_by_key<'a, Item: 'a, K: Ord>(
  iterator: impl Iterator<Item = &'a Item>, mut to_key: impl FnMut(&Item) -> K,
) -> Option<(&'a Item, &'a Item)> {
  minmax_by(iterator, |x, y| to_key(x).cmp(&to_key(y)))
}
