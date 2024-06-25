#![deny(missing_docs)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use crate::extensions::iterable::Iterable;

/// Consuming collection operations.
///
/// Methods have the following properties:
///
/// - Consumes the collection and its elements
/// - May create a new collection
///
pub trait Collectible<Item>: IntoIterator<Item = Item> {
  // FIXME - implement these methods
  // cartesian_product
  // combinations
  // combinations_repetitive
  // powerset
  // group_fold
  // group_fold_with
  // group_reduce
  // partition_map_to

  /// Original collection type
  type This<I>;

  /// Creates a collection by appending an element to the original collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = vec![1, 2];
  ///
  /// assert_eq!(a.add(3), [1, 2, 3]);
  /// ```
  fn add(self, value: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  /// Creates a collection by appending all elements of another collection to
  /// the original collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = vec![1, 2];
  ///
  /// assert_eq!(a.add_all(vec![3, 4]), [1, 2, 3, 4]);
  /// ```
  #[inline]
  fn add_all(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  /// Creates a collection from the original collection without
  /// the first occurrence of an element.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.delete(&2), vec![1, 2, 3]);
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

  /// Creates a collection from the original collection without
  /// the first occurrences of elements found in another collection.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 3];
  /// let b = vec![1, 3];
  ///
  /// assert_eq!(a.delete_all(&b), vec![2, 3]);
  /// ```
  fn delete_all<'a>(self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    let iterator = elements.iterator();
    let mut redundant: HashMap<&Item, usize> = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      *redundant.entry(item).or_default() += 1;
    }
    self
      .into_iter()
      .filter(|x| match redundant.get_mut(x) {
        Some(count) => {
          if *count > 0 {
            *count -= 1;
            false
          } else {
            true
          }
        }
        None => true,
      })
      .collect()
  }

  // FIXME - implement
  // fn combinations(self, k: usize) -> Self::This<Self>
  // where
  //   Item: Clone,
  //   Self: FromIterator<Item> + Sized,
  //   Self::This<Self>: FromIterator<Self>,
  // {
  //   if k == 0 {
  //     return Self::This::from_iter(iter::empty());
  //   }
  //   let values = Vec::from_iter(self.into_iter());
  //   if k > values.len() {
  //     return Self::This::from_iter(iter::empty());
  //   }
  //   let mut indices = Vec::from_iter(0..k);
  //   /// [1, 2, 3]
  //   /// [1, 2, 4]
  //   /// [1, 2, 5]
  //   /// [1, 3, 4]
  //   /// [1, 3, 5]
  //   /// [1, 4, 5]
  //   /// [2, 3, 4]
  //   /// [2, 3, 5]
  //   /// [2, 4, 5]
  //   /// [3, 4, 5]
  //   unfold(k - 1, |slot| {
  //     if *slot >= k {
  //       return None;
  //     }
  //     let result = Some(Self::from_iter(indices.iter().map(|index| values[*index].clone())));
  //     if indices[*slot] < *slot + k {
  //       indices[*slot] += 1;
  //       for index in (*slot + 1)..k {
  //         indices[index] = indices[index - 1] + 1;
  //       }
  //     } else {
  //       while indices[*slot] >= *slot + k {
  //         if *slot > 0 {
  //           *slot -= 1;
  //         } else {
  //           *slot = k;
  //         }
  //       }
  //     }
  //     result
  //   })
  //   .collect()
  // }

  /// Creates a collection containing an element
  /// specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(Vec::fill(1, 2), vec![1, 1]);
  /// ```
  #[inline]
  fn fill(value: Item, size: usize) -> Self
  where
    Item: Clone,
    Self: FromIterator<Item>,
  {
    iter::repeat(value).take(size).collect()
  }

  /// Creates a collection containing a result of a function
  /// specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(Vec::fill_with(|| 1, 2), vec![1, 1]);
  /// ```
  #[inline]
  fn fill_with(mut value: impl FnMut() -> Item, size: usize) -> Self
  where
    Item: Clone,
    Self: FromIterator<Item>,
  {
    iter::repeat(value()).take(size).collect()
  }

  /// Creates a collection by filtering the original collection using a
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
  /// let a = vec![0i32, 1, 2];
  ///
  /// let filtered = a.filter(|x| x.is_positive());
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
  /// let mut filtered = a.filter(|&x| *x > 1); // both & and *
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

  /// Creates a collection by filtering and mapping the original collection.
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
  /// let filter_mapped = a.map(|&s| s.parse::<i32>()).filter(|s| s.is_ok()).map_to(|s| s.unwrap());
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// Creates a collection by filters and maps the original collection.
  ///
  /// The returned collection contains only the `value`s for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map` can be used to make chains of [`filter`] and [`map`] more
  /// concise. The example below shows how a `map().filter().map()` can be
  /// shortened to a single call to `filter_map`.
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
  /// Here's the same example, but with [`filter`] and [`map`]:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.map(|s| s.parse::<i32>()).filter(|s| s.is_ok()).map_to(|s| s.unwrap());
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

  /// Applies function to the elements of a collection and returns
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

  /// Creates a collection by flattens the original nested collection.
  ///
  /// This is useful when you have a collection of iterables and
  /// you want to remove one level of indirection.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![vec![1, 2, 3, 4], vec![5, 6]];
  /// let flattened = a.flat();
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
  /// assert_eq!(flattened, vec![1, -1, 2, -2, 3, -3]);
  /// ```
  ///
  /// Flattening works on any `IntoIterator` type, including `Option` and `Result`:
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let options = vec![Some(123), Some(321), None, Some(231)];
  /// let flattened_options: Vec<_> = options.flat();
  /// assert_eq!(flattened_options, vec![123, 321, 231]);
  ///
  /// let results = vec![Ok(123), Ok(321), Err(456), Ok(231)];
  /// let flattened_results: Vec<_> = results.flat();
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
  /// assert_eq!(d2, vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
  ///
  /// let d1 = d3.flat().flat();
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

  /// Creates a collection by applying the given closure `f` to each element
  /// of the original collection and flattens the nested collection.
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
  /// [`flat`]: Collectible::flatten
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
  /// let flattened = a.flat_map(|&x| vec![x, -x]);
  /// assert_eq!(flattened, vec![1, -1, 2, -2, 3, -3]);
  /// ```
  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>;

  /// Creates a collection by applying the given closure `f` to each element
  /// of the original collection and flattens the nested collection.
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
  /// [`flat`]: Collectible::flatten
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
  /// let flattened = a.flat_map_to(|x| vec![x, -x]);
  /// assert_eq!(flattened, vec![1, -1, 2, -2, 3, -3]);
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
  /// assert_eq!(grouped, HashMap::from([
  ///   (0, vec![2]),
  ///   (1, vec![1, 3])
  /// ]));
  /// ```
  fn group_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    let iterator = self.into_iter();
    let mut result: HashMap<K, Self> = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      result.entry(to_key(&item)).or_default().extend(iter::once(item));
    }
    result.shrink_to_fit();
    result
  }

  /// Creates a collection by retaining the values representing the intersection
  /// of the original collection with another collection i.e., the values that are
  /// both in `self` and `other`.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// use std::collections::HashSet;
  /// let a = vec![1, 2, 3];
  /// let b = vec![4, 2, 3, 4];
  ///
  /// // Print 2, 3.
  /// for x in a.clone().intersect(&b) {
  ///     println!("{x}");
  /// }
  ///
  /// let intersection: Vec<_> = a.intersect(&b);
  /// assert_eq!(intersection, [2, 3]);
  /// ```
  #[inline]
  fn intersect<'a>(self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    let retained: HashSet<&Item> = HashSet::from_iter(elements.iterator());
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }

  /// Creates a collection by applying the given closure `f` to each element in
  /// the original collection.
  ///
  /// The closure `f` takes a reference to an element of type `A` and returns a value of type `R`.
  /// The resulting other are collected into a new collection of the same type.
  ///
  /// This is a non-consuming variant of [`map_to`].
  ///
  /// [`map_to`]: Collectible::map_to
  ///
  /// # Arguments
  ///
  /// * `self` - the collection to apply the mapping to.
  /// * `f` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new collection of the same type, containing the mapped other.
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
  /// assert_eq!(mapped, vec![2, 3, 4]);
  /// ```
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// Creates a collection by applying the given closure `f` to each element in
  /// the original collection.
  ///
  /// The closure `f` takes a reference to an element of type `A` and returns a value of type `R`.
  /// The resulting other are collected into a new collection of the same type.
  ///
  /// This is a consuming variant of [`map`].
  ///
  /// [`map`]: Collectible::map
  ///
  /// # Arguments
  ///
  /// * `self` - the collection to apply the mapping to.
  /// * `f` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new collection of the same type, containing the mapped other.
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

  /// Creates a collection containing the n largest elements of
  /// the original collection in descending order.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![6, 9, 1, 0, 4, 8, 7, 2, 3, 5];
  ///
  /// let largest = a.largest(5);
  ///
  /// // FIXME - correct the error
  /// // assert_eq!(largest, vec![9, 8, 7, 6, 5]);
  /// ```
  fn largest(self, n: usize) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut heap = iterator.by_ref().map(|x| Reverse(x)).take(n).collect::<BinaryHeap<_>>();
    for item in iterator {
      if (*heap.peek().unwrap()).0 < item {
        *heap.peek_mut().unwrap() = Reverse(item);
      }
    }
    heap.into_iter().rev().map(|x| x.0).collect()
  }

  /// Creates two new collections from the original collection using by applying
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

  /// Creates two new collections with arbitrary element types from the original collection
  /// by applying specified function.
  ///
  /// The function passed to `partition()` can return `Ok`, or `Err`.
  /// `partition()` returns a pair, all the `Ok` values contained, and all the `Err` values.
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

  /// Creates two new collections with arbitrary element types from the original collection
  /// by applying specified function.
  ///
  /// The function passed to `partition()` can return `Ok`, or `Err`.
  /// `partition()` returns a pair, all the `Ok` values contained, and all the `Err` values.
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
  #[inline]
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

  // FIXME - implement
  // fn powerset(self) -> Self::This<Self>;

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
  /// let product = a.product();
  ///
  /// assert_eq!(product, 24);
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
  /// If the collection is empty, returns [`None`]; otherwise, returns the
  /// result of the reduction.
  ///
  /// The reducing function is a closure with two arguments: an 'accumulator', and an element.
  /// For collections with at least one element, this is the same as [`fold()`]
  /// with the first element of the collection as the initial accumulator value, folding
  /// every subsequent element into it.
  ///
  /// [`fold()`]: crate::Traversable::fold
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let reduced: i32 = (1..10).reduce(|acc, e| acc + e).unwrap();
  /// assert_eq!(reduced, 45);
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// let folded: i32 = (1..10).fold(0, |acc, e| acc + e);
  /// assert_eq!(reduced, folded);
  /// ```
  #[inline]
  fn reduce(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item>
  where
    Self: Sized,
  {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }

  /// Creates a collection from the original collection by replacing the
  /// first occurrence of an element with a replacement value.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 3];
  /// let b = vec![2, 3];
  ///
  /// assert_eq!(a.replace(&3, 4), vec![1, 2, 4, 3]);
  /// ```
  fn replace(self, value: &Item, replacement: Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut replaced = Some(replacement);
    self.into_iter().map(|item| {
      if &item == value {
        replaced.take().unwrap_or(item)
      } else {
        item
      }
    }).collect()
  }

  /// Creates a collection from the original collection by replacing the
  /// given occurrences of elements found in another collection with elements
  /// of a replacement collection.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 3];
  /// let b = vec![2, 3];
  ///
  /// assert_eq!(a.replace_all(&b, vec![4, 5]), vec![1, 4, 5, 3]);
  /// ```
  fn replace_all<'a>(
    self, elements: &'a impl Iterable<Item<'a> = &'a Item>, replacement: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = elements.iterator();
    let mut removed: HashMap<&Item, usize> = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      *removed.entry(item).or_default() += 1;
    }
    let mut replacement_items = replacement.into_iter();
    self
      .into_iter()
      .flat_map(|item| match removed.get_mut(&item) {
        Some(count) => {
          if *count > 0 {
            *count -= 1;
            replacement_items.next().or(Some(item))
          } else {
            Some(item)
          }
        }
        None => Some(item),
      })
      .collect()
  }

  /// A collection adapter which, like [`fold`], holds internal state, but
  /// unlike [`fold`], produces a new collection.
  ///
  /// [`fold`]: crate::Traversable::fold
  ///
  /// `scan()` takes two arguments: an initial value which seeds the internal
  /// state, and a closure with two arguments, the first being a mutable
  /// reference to the internal state and the second a collection element.
  /// The closure can assign to the internal state to share state between
  /// iterations.
  ///
  /// On iteration, the closure will be applied to each element of the
  /// collection and the return value from the closure, an [`Option`], is
  /// returned by the `next` method. The closure can return
  /// `Some(value)` to yield `value`, or `None` to end the iteration.
  ///
  /// This is a non-consuming variant of [`scan_to`].
  ///
  /// [`scan_to`]: Collectible::scan_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4];
  ///
  /// let mut scan = a.scan(1, |state, &x| {
  ///     // each iteration, we'll multiply the state by the element ...
  ///     *state = *state * x;
  ///
  ///     // ... and terminate if the state exceeds 6
  ///     if *state > 6 {
  ///         return None;
  ///     }
  ///     // ... else yield the negation of the state
  ///     Some(-*state)
  /// });
  ///
  /// assert_eq!(scan, vec![-1, -2, -6]);
  /// ```
  fn scan<S, B>(&self, initial_state: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// A collection adapter which, like [`fold`], holds internal state, but
  /// unlike [`fold`], produces a new collection.
  ///
  /// [`fold`]: crate::Traversable::fold
  ///
  /// `scan()` takes two arguments: an initial value which seeds the internal
  /// state, and a closure with two arguments, the first being a mutable
  /// reference to the internal state and the second a collection element.
  /// The closure can assign to the internal state to share state between
  /// iterations.
  ///
  /// On iteration, the closure will be applied to each element of the
  /// collection and the return value from the closure, an [`Option`], is
  /// returned by the `next` method. The closure can return
  /// `Some(value)` to yield `value`, or `None` to end the iteration.
  ///
  /// This is a consuming variant of [`scan`].
  ///
  /// [`scan`]: Collectible::scan
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4];
  ///
  /// let mut scan = a.scan_to(1, |state, x| {
  ///     // each iteration, we'll multiply the state by the element ...
  ///     *state = *state * x;
  ///
  ///     // ... and terminate if the state exceeds 6
  ///     if *state > 6 {
  ///         return None;
  ///     }
  ///     // ... else yield the negation of the state
  ///     Some(-*state)
  /// });
  ///
  /// assert_eq!(scan, vec![-1, -2, -6]);
  /// ```
  fn scan_to<S, B>(self, initial_state: S, function: impl FnMut(&mut S, Item) -> Option<B>) -> Self::This<B>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().scan(initial_state, function).collect()
  }

  /// Creates a collection containing the n smallest elements of
  /// the original collection in descending order.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![6, 9, 1, 0, 4, 8, 7, 2, 3, 5];
  ///
  /// let smallest = a.smallest(5);
  ///
  /// // FIXME - correct the error
  /// // assert_eq!(smallest, vec![1, 2, 3, 4, 5]);
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

  /// Sums the elements of a collection.
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
  /// let sum = a.sum();
  ///
  /// assert_eq!(sum, 6);
  #[inline]
  fn sum(self) -> Item
  where
    Item: Sum,
    Self: Sized,
  {
    self.into_iter().sum()
  }

  /// Creates a collection containing a single element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = Vec::unit(1);
  ///
  /// assert_eq!(a, vec![1]);
  #[inline]
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item> + Sized,
  {
    iter::once(value).collect()
  }
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
