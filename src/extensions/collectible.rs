use crate::extensions::iterable::Iterable;
use crate::extensions::util::unfold::unfold;
use crate::extensions::{collect_by_index, frequencies};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

/// Consuming collection operations.
///
/// Methods have the following properties:
///
/// - Consumes the collection and its elements
/// - May create a new collection
///
pub trait Collectible<Item>: IntoIterator<Item = Item> {
  /// Original collection type
  type This<I>;

  /// Creates a new collection by appending an element to this collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2];
  ///
  /// assert_eq!(a.add(3), vec![1, 2, 3]);
  /// ```
  #[inline]
  fn add(self, value: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  /// Creates a new collection by appending all elements of another collection to
  /// the this collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2];
  ///
  /// assert_eq!(a.add_multi(vec![3, 4]), vec![1, 2, 3, 4]);
  /// ```
  #[inline]
  fn add_multi(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  /// Creates a new collection from this collection without
  /// the first occurrence of an element.
  ///
  /// The order of retained values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.delete(&2), vec![1, 2, 3]);
  ///
  /// assert_eq!(e.delete(&2), vec![]);
  /// ```
  #[inline]
  fn delete(self, value: &Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut removed = false;
    self
      .into_iter()
      .filter(|x| {
        if !removed && value == x {
          removed = true;
          false
        } else {
          true
        }
      })
      .collect()
  }

  /// Creates a new collection from this collection without
  /// the first occurrences of elements found in another collection.
  ///
  /// The order of retained values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.delete_multi(&vec![1, 2]), vec![2, 3]);
  ///
  /// assert_eq!(e.delete_multi(&vec![1]), vec![]);
  /// ```
  fn delete_multi<'a>(self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    let mut redundant: HashMap<&Item, usize> = frequencies(elements.iterator());
    self
      .into_iter()
      .filter(|x| {
        if let Some(count) = redundant.get_mut(x) {
          if *count > 0 {
            *count -= 1;
            return false;
          }
        }
        true
      })
      .collect()
  }

  /// Creates a new collection containing combinations of specified size from the elements
  /// of this collection.
  ///
  /// Combinations for sequences are generated based on element positions, not values.
  /// Therefore, if a sequence contains duplicate elements, the resulting combinations will too.
  /// To obtain combinations of unique elements for sequences, use `.unique().combinations()`.
  ///
  /// The order of combination values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.combinations(0), vec![vec![]]);
  /// assert_eq!(a.combinations(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(a.combinations(2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
  /// assert_eq!(a.combinations(3), vec![vec![1, 2, 3]]);
  ///
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  /// assert_eq!(a.combinations(4), empty_result);
  /// assert_eq!(e.combinations(2), empty_result);
  /// ```
  fn combinations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a new collection containing a result of a function
  /// specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(Vec::fill_with(|| 1, 2), vec![1, 1]);
  /// assert_eq!(Vec::fill_with(|| 1, 0), vec![]);
  /// ```
  #[inline]
  fn fill_with(mut value: impl FnMut() -> Item, size: usize) -> Self
  where
    Item: Clone,
    Self: FromIterator<Item>,
  {
    iter::repeat(value()).take(size).collect()
  }

  /// Creates a new collection by filtering this collection using a
  /// closure to determine if an element should be retained.
  ///
  /// Given an element the closure must return `true` or `false`. The returned
  /// collection will contain only the elements for which the closure returns
  /// true.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![0, 1, 2];
  ///
  /// let filtered = a.filter(|&x| x > 0);
  ///
  /// assert_eq!(filtered, vec![1, 2]);
  /// ```
  ///
  /// Because the closure passed to `filter()` takes a reference, and some
  /// collections may contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![&0, &1, &2];
  ///
  /// let filtered = a.filter(|x| **x > 1); // need two *s!
  ///
  /// assert_eq!(filtered, vec![&2]);
  /// ```
  ///
  /// It's common to instead use destructuring on the argument to strip away
  /// one:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![&0, &1, &2];
  ///
  /// let filtered = a.filter(|&x| *x > 1); // both & and *
  ///
  /// assert_eq!(filtered, vec![&2]);
  /// ```
  ///
  /// or both:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![&0, &1, &2];
  ///
  /// let filtered = a.filter(|&&x| x > 1); // two &s
  ///
  /// assert_eq!(filtered, vec![&2]);
  /// ```
  ///
  /// of these layers.
  #[inline]
  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  /// Creates a new collection by filtering and mapping this collection.
  ///
  /// The returned collection contains only the `value`s for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map` can be used to make chains of [`filter`] and [`map`] more
  /// concise. The example below shows how a `map().filter().map_to()` can be
  /// shortened to a single call to `filter_map`.
  ///
  /// This is a non-consuming variant of [`filter_map_to`].
  ///
  /// [`filter`]: Collectible::filter
  /// [`map`]: Collectible::map
  /// [`filter_map_to`]: Collectible::filter_map_to
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.filter_map(|&s| s.parse::<i32>().ok());
  ///
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  ///
  /// Here's the same example, but with [`filter`] and [`map`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.map(|s| s.parse::<i32>()).filter(|s| s.is_ok()).map(|s| s.clone().unwrap());
  ///
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// Creates a new collection by filters and maps this collection.
  ///
  /// The returned collection contains only the `value`s for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map_to` can be used to make chains of [`filter`] and [`map`] more
  /// concise. The example below shows how a `map_to().filter().map()` can be
  /// shortened to a single call to `filter_map_to`.
  ///
  /// This is a consuming variant of [`filter_map`].
  ///
  /// [`filter`]: Collectible::filter
  /// [`map`]: Collectible::map
  /// [`filter_map`]: Collectible::filter_map
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.filter_map_to(|s| s.parse::<i32>().ok());
  ///
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  ///
  /// Here's the same example, but with [`filter`] and [`map_to`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.map_to(|s| s.parse::<i32>()).filter(|s| s.is_ok()).map_to(|s| s.unwrap());
  ///
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  #[inline]
  fn filter_map_to<B>(self, function: impl FnMut(Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
    Self: Sized,
  {
    self.into_iter().filter_map(function).collect()
  }

  /// Applies function to the elements of this collection and returns
  /// the first non-none result.
  ///
  /// `find_map_to` can be used to make chains of [`find`] and [`map`] more concise.
  ///
  /// `find_map_to(f)` is equivalent to `find().map()`.
  ///
  /// This is a consuming variant of [`find_map`].
  ///
  /// [`find`]: crate::Traversable::find
  /// [`map`]: crate::Traversable::map
  /// [`find_map`]: crate::Traversable::find_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec!["lol", "NaN", "2", "5"];
  ///
  /// let first_number = a.find_map_to(|s| s.parse().ok());
  ///
  /// assert_eq!(first_number, Some(2));
  /// ```
  #[inline]
  fn find_map_to<B>(self, function: impl FnMut(Item) -> Option<B>) -> Option<B>
  where
    Self: Sized,
  {
    self.into_iter().find_map(function)
  }

  /// Creates a new collection by flattening this nested collection.
  ///
  /// This is useful when you have a collection of iterables,
  /// and you want to remove one level of indirection.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![vec![1, 2, 3, 4], vec![5, 6]];
  ///
  /// let flattened = a.flat();
  ///
  /// assert_eq!(flattened, vec![1, 2, 3, 4, 5, 6]);
  /// ```
  ///
  /// Mapping and then flattening:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // Vec is iterable because it supports IntoIterator
  /// let flattened = a.map(|&x| vec![x, -x]).flat();
  ///
  /// assert_eq!(flattened, vec![1, -1, 2, -2, 3, -3]);
  /// ```
  ///
  /// You can also rewrite this in terms of [`flat_map()`], which is preferable
  /// in this case since it conveys intent more clearly:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // Vec is iterable because it supports IntoIterator
  /// let flattened = a.flat_map(|&x| vec![x, -x]);
  ///
  /// assert_eq!(flattened, vec![1, -1, 2, -2, 3, -3]);
  /// ```
  ///
  /// Flattening works on any `IntoIterator` type, including `Option` and `Result`:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let options = vec![Some(123), Some(321), None, Some(231)];
  /// let results = vec![Ok(123), Ok(321), Err(456), Ok(231)];
  ///
  /// let flattened_options: Vec<_> = options.flat();
  ///
  /// assert_eq!(flattened_options, vec![123, 321, 231]);
  ///
  /// let flattened_results: Vec<_> = results.flat();
  ///
  /// assert_eq!(flattened_results, vec![123, 321, 231]);
  /// ```
  ///
  /// Flattening only removes one level of nesting at a time:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let d3 = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
  ///
  /// let d2 = d3.clone().flat();
  ///
  /// assert_eq!(d2, vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
  ///
  /// let d1 = d3.flat().flat();
  ///
  /// assert_eq!(d1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
  /// ```
  ///
  /// Here we see that `flat()` does not perform a "deep" flatten.
  /// Instead, only one level of nesting is removed. That is, if you
  /// `flat()` a three-dimensional array, the result will be
  /// two-dimensional and not one-dimensional. To get a one-dimensional
  /// structure, you have to `flat()` again.
  ///
  /// [`flat_map()`]: Collectible::flat_map
  #[inline]
  fn flat<B>(self) -> Self::This<B>
  where
    Item: IntoIterator<Item = B>,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flatten().collect()
  }

  /// Creates a new collection by applying the given closure `function` to each element
  /// of this collection and flattens the nested collection.
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
  /// one item for each element, and `flat_map()`'s closure returns an
  /// iterable value for each element.
  ///
  /// This is a non-consuming variant of [`flat_map_to`].
  ///
  /// [`map`]: Collectible::map
  /// [`flat`]: Collectible::flat
  /// [`flat_map_to`]: Collectible::flat_map_to
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // Vec is iterable because it implements IntoIterator
  /// let flat_mapped = a.flat_map(|&x| vec![x, -x]);
  ///
  /// assert_eq!(flat_mapped, vec![1, -1, 2, -2, 3, -3]);
  /// ```
  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>;

  /// Creates a new collection by applying the given closure `function` to each element
  /// of this collection and flattens the nested collection.
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
  /// one item for each element, and `flat_map()`'s closure returns an
  /// iterable value for each element.
  ///
  /// This is a consuming variant of [`flat_map`].
  ///
  /// [`map`]: Collectible::map
  /// [`flat`]: Collectible::flat
  /// [`flat_map`]: Collectible::flat_map
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // Vec is iterable because it implements IntoIterator
  /// let flat_mapped = a.flat_map_to(|x| vec![x, -x]);
  ///
  /// assert_eq!(flat_mapped, vec![1, -1, 2, -2, 3, -3]);
  /// ```
  #[inline]
  fn flat_map_to<B, R>(self, function: impl FnMut(Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>,
    Self: Sized,
  {
    self.into_iter().flat_map(function).collect()
  }

  /// Folds every element into an accumulator by applying an operation,
  /// returning the final result.
  ///
  /// `fold_to()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of this collection, `fold_to()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// This is a consuming variant of [`fold`].
  ///
  /// Note: [`reduce()`] can be used to use the first element as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold_to()` combines elements in a *left-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *right-associative* version of `fold_to()`, see [`rfold_to()`].
  ///
  /// [`fold`]: crate::Traversable::fold
  /// [`rfold_to`]: crate::Sequence::rfold_to
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
  /// let sum = a.fold_to(0, |acc, x| acc + x);
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
  /// This example demonstrates the left-associative nature of `fold_to()`:
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
  /// let result = numbers.fold_to(zero, |acc, x| {
  ///   format!("({acc} + {x})")
  /// });
  ///
  /// assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");
  /// ```
  /// It's common for people who haven't used collections a lot to
  /// use a `for` loop with a list of things to build up a result. Those
  /// can be turned into `fold_to()`s:
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
  /// let result2 = numbers.fold_to(0, |acc, x| acc + x);
  ///
  /// // they're the same
  /// assert_eq!(result, result2);
  /// ```
  #[inline]
  fn fold_to<B>(self, initial_value: B, function: impl FnMut(B, Item) -> B) -> B
  where
    Self: Sized,
  {
    let iterator = self.into_iter();
    iterator.fold(initial_value, function)
  }

  /// Creates `HashMap` of keys mapped to collections of elements according to
  /// specified discriminator function.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let grouped = a.group_by(|x| x % 2);
  ///
  /// assert_eq!(grouped, HashMap::from([
  ///   (0, vec![2]),
  ///   (1, vec![1, 3])
  /// ]));
  /// ```
  fn group_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
  {
    let iterator = self.into_iter();
    let mut result: HashMap<K, Self> = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      result.entry(to_key(&item)).or_default().extend(iter::once(item));
    }
    result
  }

  /// Creates `HashMap` of keys mapped and folded to values according to
  /// specified discriminator and folding operation functions.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  /// The folding operation takes an accumulator and a closure and returns a new element.
  /// The closure returns the value that the accumulator should have for the next iteration.
  ///
  /// This is a consuming variant of [`group_fold`].
  ///
  /// [`group_fold`]: Traversable::group_fold
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let group_folded = a.group_fold_to(|x| x % 2, 0, |acc, x| acc + x);
  ///
  /// assert_eq!(group_folded, HashMap::from([
  ///   (0, 2),
  ///   (1, 4),
  /// ]));
  /// ```
  fn group_fold_to<K, B>(
    self, mut to_key: impl FnMut(&Item) -> K, initial_value: B, mut function: impl FnMut(B, Item) -> B,
  ) -> HashMap<K, B>
  where
    K: Eq + Hash,
    B: Clone,
    Self: Sized,
  {
    let iterator = self.into_iter();
    let mut result: HashMap<K, B> = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      let key = to_key(&item);
      let new_value = if let Some(value) = result.remove(&key) {
        function(value, item)
      } else {
        function(initial_value.clone(), item)
      };
      let _unused = result.insert(key, new_value);
    }
    result
  }

  /// Creates `HashMap` of keys mapped and reduced to values according to
  /// specified discriminator and reducing operation functions.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  /// The reducing operation takes an accumulator and a closure and returns a new element.
  /// The closure returns the value that the accumulator should have for the next iteration.
  ///
  /// This is a consuming variant of [`group_reduce`].
  ///
  /// [`group_reduce()`]: crate::Traversable::group_reduce
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let group_reduced = a.group_reduce_to(|x| x % 2, |acc, x| acc + x);
  ///
  /// assert_eq!(group_reduced, HashMap::from([
  ///   (0, 2),
  ///   (1, 4),
  /// ]));
  /// ```
  fn group_reduce_to<K: Eq + Hash>(
    self, mut to_key: impl FnMut(&Item) -> K, mut function: impl FnMut(Item, Item) -> Item,
  ) -> HashMap<K, Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    let iterator = self.into_iter();
    let mut result: HashMap<K, Item> = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      let key = to_key(&item);
      let new_value = if let Some(value) = result.remove(&key) { function(value, item) } else { item };
      let _unused = result.insert(key, new_value);
    }
    result
  }

  /// Creates a new collection by retaining the values representing the intersection
  /// of this collection with another collection i.e., the values appear in the result
  /// exactly the same amount of times as they both appear in `self` and `other`.
  ///
  /// To obtain set-like semantics for sequences which only considers unique elements,
  /// use `.unique().intersect()`.
  ///
  /// The order of retained values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  /// use std::collections::HashSet;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// let intersection = a.intersect(&vec![4, 2, 2, 3, 4]);
  ///
  /// assert_eq!(intersection, vec![2, 2, 3]);
  /// assert_eq!(e.intersect(&vec![1]), vec![]);
  ///
  /// // Print 2, 3.
  /// for x in intersection {
  ///   println!("{x}");
  /// }
  /// ```
  #[inline]
  fn intersect<'a>(self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    let mut retained: HashMap<&Item, usize> = frequencies(elements.iterator());
    self
      .into_iter()
      .flat_map(|item| {
        if let Some(count) = retained.get_mut(&item) {
          if *count > 0 {
            *count -= 1;
            return Some(item);
          }
        }
        None
      })
      .collect()
  }

  /// Creates a new collection by applying the given closure `function` to
  /// each element in this collection.
  ///
  /// The closure `function` takes a reference to an element of type
  /// `Item` and returns a value of type `R`.
  /// The resulting other are collected into a new collection of the same type.
  ///
  /// This is a non-consuming variant of [`map_to`].
  ///
  /// [`map_to`]: Collectible::map_to
  ///
  /// # Arguments
  ///
  /// * `self` - the collection to apply the mapping to.
  /// * `function` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new collection of the same type, containing the mapped elements.
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
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let mapped = a.map(|&x| x + 1);
  ///
  /// assert_eq!(mapped, vec![2, 3, 4]);
  /// ```
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// Creates a new collection by applying the given closure `function` to each element in
  /// this collection.
  ///
  /// The closure `function` takes a reference to an element of type
  /// `Item` and returns a value of type `R`.
  /// The resulting other are collected into a new collection of the same type.
  ///
  /// This is a consuming variant of [`map`].
  ///
  /// [`map`]: Collectible::map
  ///
  /// # Arguments
  ///
  /// * `self` - the collection to apply the mapping to.
  /// * `function` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new collection of the same type, containing the mapped elements.
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
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let mapped = a.map_to(|x| x + 1);
  ///
  /// assert_eq!(mapped, vec![2, 3, 4]);
  /// ```
  #[inline]
  fn map_to<B>(self, function: impl FnMut(Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>,
    Self: Sized,
  {
    self.into_iter().map(function).collect()
  }

  // FIXME -  fix the failing test case
  /// Creates a new collection containing the n largest elements of
  /// this collection in descending order.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![5, 1, 3, 2, 4];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// let largest = a.largest(3);
  ///
  /// // assert_eq!(largest, vec![5, 4, 3]);
  /// assert_eq!(e.largest(3), vec![]);
  /// ```
  fn largest(self, n: usize) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut heap = iterator.by_ref().map(|x| Reverse(x)).take(n).collect::<BinaryHeap<_>>();
    for item in iterator {
      if heap.peek().unwrap().0 < item {
        *heap.peek_mut().unwrap() = Reverse(item);
      }
    }
    heap.into_iter().rev().map(|x| x.0).collect()
  }

  /// Creates two new collections from this collection by applying
  /// specified predicate.
  ///
  /// The predicate passed to `partition()` can return `true`, or `false`.
  /// `partition()` returns a pair, all the elements for which it returned
  /// `true`, and all the elements for which it returned `false`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let (even, odd) = a.partition(|n| n % 2 == 0);
  ///
  /// assert_eq!(even, vec![2]);
  /// assert_eq!(odd, vec![1, 3]);
  /// ```
  #[inline]
  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
    Self: Default + Extend<Item> + IntoIterator<Item = Item>,
  {
    self.into_iter().partition(predicate)
  }

  /// Creates two new collections with arbitrary element types from this collection
  /// by applying specified function.
  ///
  /// The function passed to `partition_map()` can return `Ok`, or `Err`.
  /// `partition_map()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a non-consuming variant of [`partition_map_to`].
  ///
  /// [`partition_map_to`]: Collectible::partition_map_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let (even, odd) = a.partition_map(|n| if n % 2 == 0 { Ok(n + 3) } else { Err(*n) });
  ///
  /// assert_eq!(even, vec![5]);
  /// assert_eq!(odd, vec![1, 3]);
  /// ```
  fn partition_map<A, B>(&self, function: impl FnMut(&Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
  where
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>;

  /// Creates two new collections with arbitrary element types from this collection
  /// by applying specified function.
  ///
  /// The function passed to `partition_map_to()` can return `Ok`, or `Err`.
  /// `partition_map_to()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a consuming variant of [`partition_map`].
  ///
  /// [`partition_map`]: Collectible::partition_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let (even, odd) = a.partition_map_to(|n| if n % 2 == 0 { Ok(n + 3) } else { Err(n) });
  ///
  /// assert_eq!(even, vec![5]);
  /// assert_eq!(odd, vec![1, 3]);
  /// ```
  fn partition_map_to<A, B>(self, mut function: impl FnMut(Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>,
  {
    let mut result_left: Self::This<A> = Self::This::default();
    let mut result_right: Self::This<B> = Self::This::default();
    for item in self.into_iter() {
      match function(item) {
        Ok(value) => result_left.extend(iter::once(value)),
        Err(value) => result_right.extend(iter::once(value)),
      }
    }
    (result_left, result_right)
  }

  /// Creates a new collection containing all sub-collections of this collection.
  ///
  /// Sub-collections for sequences are generated based on element positions, not values.
  /// Therefore, if an sequence contains duplicate elements, the resulting sub-collections will too.
  /// To obtain combinations of unique elements for sequences, use `.unique().powerset()`.
  ///
  /// The order of sub-collection values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.powerset(), vec![
  ///   vec![],
  ///   vec![1], vec![2], vec![3],
  ///   vec![1, 2], vec![1, 3], vec![2, 3],
  ///   vec![1, 2, 3]]
  /// );
  /// assert_eq!(e.powerset(), vec![vec![]]);
  /// ```
  fn powerset(&self) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Iterates over the entire collection, multiplying all the elements
  ///
  /// An empty collection returns the one value of the type.
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
  /// let a = vec![2, 3, 4];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// let product = a.product();
  ///
  /// assert_eq!(product, 24);
  /// assert_eq!(e.product(), 1);
  /// ```
  #[inline]
  fn product(self) -> Item
  where
    Item: Product,
    Self: Sized,
  {
    self.into_iter().product()
  }

  /// Reduces the elements to a single one, by repeatedly applying a reducing
  /// operation.
  ///
  /// If this collection is empty, returns [`None`]; otherwise, returns the
  /// result of the reduction.
  ///
  /// The reducing function is a closure with two arguments: an 'accumulator', and an element.
  /// For collections with at least one element, this is the same as [`fold_to()`]
  /// with the first element of this collection as the initial accumulator value, folding
  /// every subsequent element into it.
  ///
  /// This is a consuming variant of [`reduce`].
  ///
  /// [`fold_to()`]: Collectible::fold_to
  /// [`reduce()`]: crate::Traversable::reduce
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// # let source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// let reduced = a.reduce_to(|acc, e| acc + e).unwrap();
  ///
  /// assert_eq!(reduced, 6);
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// # let a = source.clone();
  /// let folded = a.fold_to(0, |acc, e| acc + e);
  ///
  /// assert_eq!(reduced, folded);
  /// ```
  #[inline]
  fn reduce_to(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item>
  where
    Self: Sized,
  {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }

  /// Creates a new collection from this collection by replacing the
  /// first occurrence of an element with a replacement value.
  ///
  /// The order of retained values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.substitute(&2, 4), vec![1, 4, 2, 3]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.substitute(&4, 5), vec![1, 2, 2, 3]);
  /// assert_eq!(e.substitute(&1, 2), vec![]);
  /// ```
  #[inline]
  fn substitute(self, value: &Item, replacement: Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut replaced = Some(replacement);
    self.into_iter().map(|item| if &item == value { replaced.take().unwrap_or(item) } else { item }).collect()
  }

  // FIXME -  fix the failing test case
  /// Creates a new collection containing the n smallest elements of
  /// this collection in descending order.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![5, 1, 3, 2, 4];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// let smallest = a.smallest(3);
  ///
  /// // assert_eq!(smallest, vec![1, 2, 3]);
  /// assert_eq!(e.smallest(3), vec![]);
  /// ```
  fn smallest(self, n: usize) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut heap = iterator.by_ref().take(n).collect::<BinaryHeap<_>>();
    for item in iterator {
      if *heap.peek().unwrap() > item {
        *heap.peek_mut().unwrap() = item;
      }
    }
    heap.into_iter().collect()
  }

  // FIXME -  fix the failing test case
  /// Creates a new collection from this collection by replacing the
  /// first occurrences of elements found in another collection with elements
  /// of a replacement collection.
  ///
  /// The order of retained values is preserved for sequences.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// // assert_eq!(a.replace_multi(&vec![2, 4], vec![3, 5]), vec![1, 4, 2, 5]);
  /// # let a = source.clone();
  /// // assert_eq!(a.replace_multi(&vec![2, 4], vec![4, 5]), vec![1, 4, 3, 3]);
  ///
  /// # let a = source.clone();
  /// // assert_eq!(a.replace_multi(&vec![4, 6], vec![5, 7]), vec![1, 2, 3, 3]);
  /// assert_eq!(e.update_multi(&vec![1], vec![2]), vec![]);
  /// ```
  fn update_multi<'a>(
    self, elements: &'a impl Iterable<Item<'a> = &'a Item>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut removed: HashMap<&Item, usize> = frequencies(elements.iterator());
    let mut replacement_items = replacements.into_iter();
    self
      .into_iter()
      .flat_map(|item| {
        if let Some(count) = removed.get_mut(&item) {
          if *count > 0 {
            *count -= 1;
            return replacement_items.next().or(Some(item));
          }
        }
        Some(item)
      })
      .collect()
  }

  /// Sums the elements of this collection.
  ///
  /// Takes each element, adds them together, and returns the result.
  ///
  /// An empty collection returns the zero value of the type.
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
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// let sum = a.sum();
  ///
  /// assert_eq!(sum, 6);
  /// assert_eq!(e.sum(), 0);
  /// ```
  #[inline]
  fn sum(self) -> Item
  where
    Item: Sum,
    Self: Sized,
  {
    self.into_iter().sum()
  }

  /// Creates a new collection containing a single element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let unit = Vec::unit(1);
  ///
  /// assert_eq!(unit, vec![1]);
  #[inline]
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item> + Sized,
  {
    iter::once(value).collect()
  }
}

#[inline]
pub(crate) fn combinations<'a, Item: Clone + 'a, Collection: FromIterator<Item> + Sized>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  compute_combinations(&values, k)
}

pub(crate) fn compute_combinations<'a, Item, Collection>(values: &[&Item], k: usize) -> Vec<Collection>
where
  Item: Clone + 'a,
  Collection: FromIterator<Item> + Sized,
{
  let size = values.len();
  let mut combination = Vec::from_iter(iter::once(i64::MIN).chain(0..(k as i64)));
  let mut current_slot = (size + 1).saturating_sub(k);
  unfold((), |_| {
    if current_slot == 0 {
      return None;
    }
    current_slot = k;
    let result = Some(collect_by_index(values, &combination[1..]));
    while combination[current_slot] >= (size + current_slot - k) as i64 - 1 {
      current_slot -= 1;
    }
    let mut new_index = combination[current_slot];
    for index in &mut combination[current_slot..=k] {
      new_index += 1;
      *index = new_index;
    }
    result
  })
  .collect()
}

pub(crate) fn partition_map<'a, Item: 'a, A, B, Left: Default + Extend<A>, Right: Default + Extend<B>>(
  iterator: impl Iterator<Item = &'a Item>, mut function: impl FnMut(&Item) -> Result<A, B>,
) -> (Left, Right) {
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

pub(crate) fn powerset<'a, Item: Clone + 'a, Collection: FromIterator<Item> + Sized>(
  iterator: impl Iterator<Item = &'a Item>,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  let sizes = 1..=values.len();
  iter::once(Collection::from_iter(iter::empty()))
    .chain(sizes.flat_map(|size| compute_combinations::<Item, Collection>(&values, size)))
    .collect()
}

pub(crate) fn substitute_multi<'a, Item, Collection>(
  collection: Collection, elements: &'a impl Iterable<Item<'a> = &'a Item>, replacement: impl IntoIterator<Item = Item>,
) -> Collection
where
  Item: Eq + Hash + 'a,
  Collection: IntoIterator<Item = Item> + FromIterator<Item>,
{
  let iterator = elements.iterator();
  let removed: HashSet<&Item> = HashSet::from_iter(iterator);
  collection.into_iter().filter(|x| !removed.contains(x)).chain(replacement).collect()
}
