use std::cmp::Ordering;

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
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert!(a.all(|&x| x > 0));
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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert!(a.any(|&x| x > 0));
  ///
  /// assert!(!a.any(|&x| x > 5));
  /// ```
  fn any(&self, predicate: impl FnMut(&Item) -> bool) -> bool;

  /// Counts elements of an collection that satisfy a predicate.
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
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.count_by(|&x| x == 2), 1);
  ///
  /// assert_eq!(a.count_by(|&x| x == 5), 0);
  /// ```
  fn count_by(&self, predicate: impl FnMut(&Item) -> bool) -> usize;

  /// Searches for an element of an collection that satisfies a predicate.
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
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.find(|&x| x == 2), Some(&2));
  ///
  /// assert_eq!(a.find(|&x| x == 5), None);
  /// ```
  fn find(&self, predicate: impl FnMut(&Item) -> bool) -> Option<&Item>;

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
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
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
  fn fold<B>(&self, init: B, function: impl FnMut(B, &Item) -> B) -> B;

  /// Returns the element that gives the maximum value with respect to the
  /// specified comparison function.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(*a.max_by(|x, y| x.cmp(y)).unwrap(), 5);
  /// ```
  fn max_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the maximum element of a collection.
  ///
  /// If several elements are equally maximum, the last element is
  /// returned. If the collection is empty, [`None`] is returned.
  ///
  /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
  /// incomparable. You can work around this by using [`Collectible::reduce`]:
  /// ```
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let b: Vec<u32> = Vec::new();
  ///
  /// assert_eq!(a.max_entry(), Some(&3));
  /// assert_eq!(b.max_entry(), None);
  /// ```
  #[inline]
  fn max_entry(&self) -> Option<&Item> where Item: Ord {
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
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![-3_i32, 0, 1, 5, -10];
  /// assert_eq!(*a.min_by(|x, y| x.cmp(y)).unwrap(), -10);
  /// ```
  fn min_by(&self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Option<&Item>;

  /// Returns the minimum element of an collection.
  ///
  /// If several elements are equally minimum, the first element is returned.
  /// If the collection is empty, [`None`] is returned.
  ///
  /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
  /// incomparable. You can work around this by using [`Collectible::reduce`]:
  /// ```
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let b: Vec<u32> = Vec::new();
  ///
  /// assert_eq!(a.min_entry(), Some(&1));
  /// assert_eq!(b.min_entry(), None);
  /// ```
  #[inline]
  fn min_entry(&self) -> Option<&Item> where Item: Ord {
    self.min_by(Ord::cmp)
  }
}

#[inline]
pub(crate) fn all<'a, A: 'a>(mut iterator: impl Iterator<Item = &'a A>, predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.all(predicate)
}

#[inline]
pub(crate) fn any<'a, A: 'a>(mut iterator: impl Iterator<Item = &'a A>, predicate: impl FnMut(&A) -> bool) -> bool {
  iterator.any(predicate)
}

#[inline]
pub(crate) fn count_by<'a, A: 'a>(
  iterator: impl Iterator<Item = &'a A>, mut predicate: impl FnMut(&A) -> bool,
) -> usize {
  iterator.filter(|&x| predicate(x)).count()
}

#[inline]
pub(crate) fn fold<'a, A: 'a, B>(
  iterator: impl Iterator<Item = &'a A>, init: B, function: impl FnMut(B, &A) -> B,
) -> B {
  iterator.fold(init, function)
}
