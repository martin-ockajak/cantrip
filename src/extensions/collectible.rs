use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};
use crate::extensions::collections::iterable::Iterable;

/// Consuming collection operations.
///
/// Methods have the following properties:
///
/// - Consumes the collection and its elements
/// - May create a new collection
///
pub trait Collectible<Item>: IntoIterator<Item = Item> + Sized {
  type This<I>;

  fn add(self, value: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(value)).collect()
  }

  /// Retains the values representing the difference,
  /// i.e., the values that are in `self` but not in `other`.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let b = vec![4, 3, 4];
  ///
  /// // Can be seen as `a - b`.
  /// // Print 1, 2.
  /// for x in a.clone().diff(&b) {
  ///     println!("{x}");
  /// }
  ///
  /// let diff: Vec<_> = a.clone().diff(&b);
  /// assert_eq!(diff, vec![1, 2]);
  ///
  /// // Note that difference is not symmetric,
  /// // and `b - a` means something else:
  /// let diff: Vec<_> = b.diff(&a);
  /// assert_eq!(diff, vec![4, 4]);
  /// ```
  #[inline]
  fn diff<'a>(self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    let mut removed: HashSet<&Item> = HashSet::new();
    removed.extend(iterable.iterator());
    self.into_iter().filter(|x| !removed.contains(x)).collect()
  }

  #[inline]
  fn exclude(self, value: &Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut removed = false;
    self
      .into_iter()
      .filter(|x| {
        if removed {
          true
        } else {
          removed = true;
          value != x
        }
      })
      .collect()
  }

  /// Filters a collection using a closure to determine if an element should be retained.
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
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
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

  /// Filters and maps a collection.
  ///
  /// The returned collection contains only the `value`s for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map` can be used to make chains of [`filter`] and [`map`] more
  /// concise. The example below shows how a `map().filter().map()` can be
  /// shortened to a single call to `filter_map`.
  ///
  /// [`filter`]: Collectible::filter
  /// [`map`]: Collectible::map
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.filter_map(|s| s.parse::<i32>().ok());
  ///
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  ///
  /// Here's the same example, but with [`filter`] and [`map`]:
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec!["1", "two", "NaN", "four", "5"];
  ///
  /// let filter_mapped = a.map(|s| s.parse::<i32>()).filter(|s| s.is_ok()).map(|s| s.clone().unwrap());
  /// assert_eq!(filter_mapped, vec![1, 5]);
  /// ```
  fn filter_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  fn find_map<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Option<B>;

  /// Flattens a nested structure.
  ///
  /// This is useful when you have a collection of iterables and
  /// you want to remove one level of indirection.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![vec![1, 2, 3, 4], vec![5, 6]];
  /// let flattened = a.flat();
  /// assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);
  /// ```
  ///
  /// Mapping and then flattening:
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // Vec is iterable because it supports IntoIterator
  /// let merged: Vec<i32> = a.map(|x| vec![*x, -x]).flat();
  /// assert_eq!(merged, [1, -1, 2, -2, 3, -3]);
  /// ```
  ///
  /// You can also rewrite this in terms of [`flat_map()`], which is preferable
  /// in this case since it conveys intent more clearly:
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// // Vec is iterable because it supports IntoIterator
  /// let merged: Vec<i32> = a.flat_map(|x| vec![*x, -x]);
  /// assert_eq!(merged, [1, -1, 2, -2, 3, -3]);
  /// ```
  ///
  /// Flattening works on any `IntoIterator` type, including `Option` and `Result`:
  ///
  /// ```
  /// use cantrip::extensions::*;
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
  /// use cantrip::extensions::*;
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

  /// Applies the given closure `f` to each element in the container and flattens the nested structure.
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
  /// [`map`]: Collectible::map
  /// [`flat`]: Collectible::flatten
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
  /// // Vec is iterable because it implements IntoIterator
  /// let merged: Vec<i32> = a.flat_map(|x| vec![*x, -x]);
  /// assert_eq!(merged, [1, -1, 2, -2, 3, -3]);
  /// ```
  fn flat_map<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>;

  #[inline]
  fn grouped_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    let mut result: HashMap<K, Self> = HashMap::new();
    for item in self.into_iter() {
      let key = to_key(&item);
      result.entry(key).and_modify(|values| values.extend(iter::once(item))).or_default();
    }
    result
  }

  /// Retains the values representing the intersection,
  /// i.e., the values that are both in `self` and `other`.
  ///
  /// The order or retained values is preserved for ordered collections.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::extensions::*;
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
  fn intersect<'a>(self, iterable: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: FromIterator<Item>,
  {
    let mut retained: HashSet<&Item> = HashSet::new();
    retained.extend(iterable.iterator());
    self.into_iter().filter(|x| retained.contains(x)).collect()
  }

  /// Applies the given closure `f` to each element in the container.
  ///
  /// The closure `f` takes a reference to an element of type `A` and returns a value of type `R`.
  /// The resulting other are collected into a new container of the same type.
  ///
  /// # Arguments
  ///
  /// * `self` - the container to apply the mapping to.
  /// * `f` - the closure to apply to each element.
  ///
  /// # Returns
  ///
  /// A new container of the same type, containing the mapped other.
  ///
  /// # Safety
  ///
  /// The caller must ensure that the closure does not mutate any shared state while being executed.
  /// The closure must not panic while being executed, as this will lead to undefined behavior.
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::cantrip::extensions::*;
  ///
  /// let result: Vec<i32> = vec![1, 2, 3].map(|x| x + 1);
  /// ```
  fn map<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  // FIXME - implement n_largest
  // fn largest_by(self, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  // where
  //   Item: Ord,
  //   Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  // {
  //   largest_by(self, n, compare)
  // }

  #[inline]
  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable).collect()
  }

  #[inline]
  fn partition(self, predicate: impl FnMut(&Item) -> bool) -> (Self, Self)
  where
    Self: Sized + Default + Extend<Item> + IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().partition(predicate)
  }

  #[inline]
  fn product(self) -> Item
  where
    Item: Product,
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
  /// [`fold()`]: Traversable::fold
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::extensions::*;
  ///
  /// let reduced: i32 = (1..10).reduce(|acc, e| acc + e).unwrap();
  /// assert_eq!(reduced, 45);
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// let folded: i32 = (1..10).fold(0, |acc, e| acc + e);
  /// assert_eq!(reduced, folded);
  /// ```
  #[inline]
  fn reduce(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item> {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }

  #[inline]
  fn sum(self) -> Item
  where
    Item: Sum,
  {
    self.into_iter().sum()
  }

  #[inline]
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item> + Sized,
  {
    iter::once(value).collect()
  }
}

#[inline]
pub(crate) fn filter_map<'a, A: 'a, B, C>(
  iterator: impl Iterator<Item = &'a A>, function: impl FnMut(&A) -> Option<B>,
) -> C
where
  C: FromIterator<B>,
{
  iterator.filter_map(function).collect()
}

#[inline]
pub(crate) fn find_map<'a, A: 'a, B>(
  mut iterator: impl Iterator<Item = &'a A>, function: impl FnMut(&A) -> Option<B>,
) -> Option<B> {
  iterator.find_map(function)
}

#[inline]
pub(crate) fn flat_map<'a, A: 'a, B, R, C>(
  iterator: impl Iterator<Item = &'a A>, function: impl FnMut(&A) -> R,
) -> C
where
  R: IntoIterator<Item = B>,
  C: FromIterator<B>,
{
  iterator.flat_map(function).collect()
}

#[inline]
pub(crate) fn map<'a, A: 'a, B, C>(iterator: impl Iterator<Item = &'a A>, function: impl FnMut(&A) -> B) -> C
where
  C: FromIterator<B>,
{
  iterator.map(function).collect()
}

// fn largest_by<Item, Collection>(
//   collection: Collection, n: usize, compare: impl FnMut(&Item, &Item) -> Ordering,
// ) -> Collection
// where
//   Item: Ord,
//   Collection: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
// {
//   let heap = BinaryHeap::new_by(compare);
//   heap.extend(collection);
//   let mut result = Collection::default();
//   for _ in 0..n {
//     result.extend(iter::once(heap.pop().unwrap()));
//   }
//   result
// }
