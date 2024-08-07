use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, LinkedList, VecDeque};
use std::hash::Hash;
use std::iter;
use std::iter::{Product, Sum};

use crate::core::unfold::unfold;
use crate::extensions::{collect_by_index, frequencies};
use crate::Iterable;

/// Consuming collection operations.
///
/// Methods have the following properties:
///
/// - Consumes the collection and its elements
/// - May create a new collection
///
pub trait CollectionTo<Item> {
  /// This collection type constructor
  type This<I>;

  /// Creates a new collection by appending an element to this collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.add(3), vec![1, 2, 3, 3]);
  /// ```
  #[inline]
  fn add(self, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().chain(iter::once(element)).collect()
  }

  /// Creates a new collection by appending all elements of another collection to
  /// this collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.add_multi(vec![3, 4]), vec![1, 2, 3, 3, 4]);
  /// ```
  #[inline]
  fn add_multi(self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().chain(elements).collect()
  }

  /// Transforms this collection into specified collection type.
  ///
  /// `collect()` can take any collection, and turn it into a relevant
  /// collection. This can be used in a variety of contexts.
  ///
  /// `collect()` can also create instances of types that are not typical
  /// collections. For example, a [`String`] can be built from [`char`]s,
  /// and a collection of [`Result<T, E>`][`Result`] items can be collected
  /// into `Result<Collection<T>, E>`. See the examples below for more.
  ///
  /// Because `collect()` is so general, it can cause problems with type
  /// inference. As such, `collect()` is one of the few times you'll see
  /// the syntax affectionately known as the 'turbofish': `::<>`. This
  /// helps the inference algorithm understand specifically which collection
  /// you're trying to collect into.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::LinkedList;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let collected: LinkedList<i32> = a.collect();
  ///
  /// assert_eq!(collected, LinkedList::from([1, 2, 3]));
  /// ```
  ///
  /// Note that we needed the `::LinkedList<i32>` on the left-hand side. This is because
  /// we could collect into, for example, a [`VecDeque<T>`] instead:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::VecDeque;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let collected: VecDeque<i32> = a.collect();
  ///
  /// assert_eq!(collected, VecDeque::from([1, 2, 3]));
  /// ```
  ///
  /// Using the 'turbofish' instead of annotating `collected`:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::VecDeque;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.collect::<VecDeque<i32>>(), VecDeque::from([1, 2, 3]));
  /// ```
  ///
  /// Because `collect()` only cares about what you're collecting into, you can
  /// still use a partial type hint, `_`, with the turbofish:
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::VecDeque;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.collect::<VecDeque<_>>(), VecDeque::from([1, 2, 3]));
  /// ```
  ///
  /// Using `collect()` to make a [`String`]:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec!['h', 'e', 'l', 'l', 'o'];
  ///
  /// let hello: String = a.collect();
  ///
  /// assert_eq!("hello", hello);
  /// ```
  ///
  /// If you have a list of [`Result<T, E>`][`Result`]s, you can use `collect()` to
  /// see if any of them failed:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![Ok(1), Err("nope"), Ok(3), Err("bad")];
  ///
  /// let result: Result<Vec<_>, &str> = a.collect();
  ///
  /// // gives us the first error
  /// assert_eq!(Err("nope"), result);
  ///
  /// let b = vec![Ok(1), Ok(3)];
  ///
  /// let result: Result<Vec<_>, &str> = b.collect();
  ///
  /// // gives us the list of answers
  /// assert_eq!(Ok(vec![1, 3]), result);
  /// ```
  ///
  /// [`VecDeque<T>`]: ../../std/collections/struct.VecDeque.html
  /// [`iter`]: Iterator::next
  /// [`String`]: ../../std/string/struct.String.html
  /// [`char`]: type@char
  #[inline]
  fn collect<B>(self) -> B
  where
    B: FromIterator<Item>,
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().collect()
  }

  /// Creates a new collection containing combinations of specified size from the elements
  /// of this collection.
  ///
  /// Combinations for sequences are generated based on element positions, not values.
  /// Therefore, if a sequence contains duplicate elements, the resulting combinations will too.
  /// To obtain combinations of unique elements for sequences, use [`unique()`]`.combinations()`.
  ///
  /// The order of combination values is preserved for sequences.
  ///
  /// [`unique()`]: crate::SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.combinations(0), vec![vec![]]);
  /// assert_eq!(a.combinations(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(a.combinations(2), vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
  /// assert_eq!(a.combinations(3), vec![vec![1, 2, 3]]);
  ///
  /// assert_eq!(a.combinations(4), Vec::<Vec<i32>>::new());
  /// assert_eq!(e.combinations(1), Vec::<Vec<i32>>::new());
  /// ```
  fn combinations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

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
  /// # let a_source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.delete(&2), vec![1, 2, 3]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.delete(&4), vec![1, 2, 2, 3]);
  /// assert_eq!(e.delete(&2), vec![]);
  /// ```
  #[inline]
  fn delete(self, element: &Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut removed = false;
    self
      .into_iter()
      .filter(|x| {
        if !removed && element == x {
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
  /// # let a_source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.delete_multi(&vec![1, 2]), vec![2, 3]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.delete_multi(&vec![4]), vec![1, 2, 2, 3]);
  /// assert_eq!(e.delete_multi(&vec![1]), vec![]);
  /// ```
  fn delete_multi<'a>(self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut deleted: HashMap<&Item, usize> = frequencies(elements.iterator());
    self
      .into_iter()
      .filter(|x| {
        if let Some(count) = deleted.get_mut(x) {
          if *count > 0 {
            *count -= 1;
            return false;
          }
        }
        true
      })
      .collect()
  }

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
  fn fill_with(mut element: impl FnMut() -> Item, size: usize) -> Self
  where
    Item: Clone,
    Self: FromIterator<Item>,
  {
    iter::repeat(element()).take(size).collect()
  }

  /// Creates a new collection by filtering this collection using a
  /// closure to determine if an element should be retained.
  ///
  /// Given an element the closure must return `true` or `false`. The returned
  /// collection will contain only the elements for which the closure returns
  /// true.
  ///
  /// This is a non-consuming variant of [`filter_ref()`].
  ///
  /// [`filter_ref()`]: CollectionTo::filter_ref
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
  /// assert_eq!(
  ///   a.filter(|&x| x > 1),
  ///   vec![2, 3]
  /// );
  /// ```
  ///
  /// Because the closure passed to `filter()` takes a reference, and some
  /// collections may contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![&1, &2, &3];
  ///
  /// assert_eq!(
  ///   // need two *s!
  ///   a.filter(|x| **x > 2),
  ///   vec![&3]
  /// );
  /// ```
  ///
  /// It's common to instead use destructuring on the argument to strip away
  /// one:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![&1, &2, &3];
  ///
  /// assert_eq!(
  ///   // both & and *
  ///   a.filter(|&x| *x > 2),
  ///   vec![&3]
  /// );
  /// ```
  ///
  /// or both:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![&0, &1, &2];
  ///
  /// assert_eq!(
  ///   // two &s
  ///   a.filter(|&&x| x > 1),
  ///   vec![&2]
  /// );
  /// ```
  ///
  /// of these layers.
  #[inline]
  fn filter(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().filter(predicate).collect()
  }

  /// Creates a new collection by filters and maps this collection.
  ///
  /// The returned collection contains only the `value`s for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map()` can be used to make chains of [`filter()`] and [`map()`] more
  /// concise. The example below shows how a `filter().map()` can be shortened to a
  /// single call to `filter_map`.
  ///
  /// This is a consuming variant of [`filter_map_ref()`].
  ///
  /// [`filter()`]: CollectionTo::filter
  /// [`map()`]: CollectionTo::map
  /// [`filter_map_ref()`]: CollectionTo::filter_map_ref
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
  /// assert_eq!(
  ///   a.filter_map(|x| if x % 2 == 0 { Some(x + 1) } else { None }),
  ///   vec![3]
  /// );
  /// ```
  ///
  /// Here's the same example, but with [`filter()`] and [`map()`]:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.filter(|&x| x % 2 == 0).map(|x| x + 1),
  ///   vec![3]
  /// );
  /// ```
  #[inline]
  fn filter_map<B>(self, function: impl FnMut(Item) -> Option<B>) -> Self::This<B>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().filter_map(function).collect()
  }

  /// Creates a new collection by filtering and mapping this collection.
  ///
  /// The returned collection contains only the `value`s for which the supplied
  /// closure returns `Some(value)`.
  ///
  /// `filter_map_ref()` can be used to make chains of [`filter()`] and [`map_ref()`] more
  /// concise. The example below shows how a `filter().map_ref()` can be shortened to a
  /// single call to `filter_map()`.
  ///
  /// This is a non-consuming variant of [`filter_map()`].
  ///
  /// [`filter()`]: CollectionTo::filter
  /// [`map_ref()`]: CollectionTo::map_ref
  /// [`filter_map()`]: CollectionTo::filter_map_ref
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
  /// assert_eq!(
  ///   a.filter_map_ref(|&x| if x % 2 == 0 { Some(x + 1) } else { None } ),
  ///   vec![3]
  /// );
  /// ```
  ///
  /// Here's the same example, but with [`filter()`] and [`map_ref()`]:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.filter(|&x| x % 2 == 0).map_ref(|x| x + 1),
  ///   vec![3]
  /// );
  /// ```
  fn filter_map_ref<B>(&self, function: impl FnMut(&Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// Creates a new collection by filtering this collection using a
  /// closure to determine if an element should be retained.
  ///
  /// Given an element the closure must return `true` or `false`. The returned
  /// collection will contain only the elements for which the closure returns
  /// true.
  ///
  /// This is a non-consuming variant of [`filter()`].
  ///
  /// [`filter()`]: CollectionTo::filter
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
  /// assert_eq!(
  ///   a.filter_ref(|&x| x > 1),
  ///   vec![2, 3]
  /// );
  /// ```
  ///
  /// Because the closure passed to `filter()` takes a reference, and some
  /// collections may contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![&1, &2, &3];
  ///
  /// assert_eq!(
  ///   // need two *s!
  ///   a.filter_ref(|x| **x > 2),
  ///   vec![&3]
  /// );
  /// ```
  ///
  /// It's common to instead use destructuring on the argument to strip away
  /// one:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![&1, &2, &3];
  ///
  /// assert_eq!(
  ///   // both & and *
  ///   a.filter_ref(|&x| *x > 2),
  ///   vec![&3]
  /// );
  /// ```
  ///
  /// or both:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![&0, &1, &2];
  ///
  /// assert_eq!(
  ///   // two &s
  ///   a.filter_ref(|&&x| x > 1),
  ///   vec![&2]
  /// );
  /// ```
  ///
  /// of these layers.
  fn filter_ref(&self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Item: Clone;

  /// Applies function to the elements of this collection and returns
  /// the first non-none result.
  ///
  /// `find_map()` can be used to make chains of [`find()`] and [`map()`] more concise.
  ///
  /// `find_map(f)` is equivalent to `find().map()`.
  ///
  /// This is a consuming variant of [`find_map_ref()`].
  ///
  /// [`find()`]: crate::Collection::find
  /// [`map()`]: CollectionTo::map_ref
  /// [`find_map_ref()`]: crate::Collection::find_map_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.find_map(|x| if x % 2 == 0 { Some(x) } else { None } ),
  ///   Some(2)
  /// );
  /// ```
  #[inline]
  fn find_map<B>(self, function: impl FnMut(Item) -> Option<B>) -> Option<B>
  where
    Self: IntoIterator<Item = Item> + Sized,
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
  /// use cantrip::*;
  ///
  /// let a = vec![vec![1, 2], vec![3]];
  ///
  /// assert_eq!(a.flat(), vec![1, 2, 3]);
  /// ```
  ///
  /// Mapping and then flattening:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   // Vec is iterable because it supports IntoIterator
  ///   a.map_ref(|&x| vec![x, -x]).flat(),
  ///   vec![1, -1, 2, -2, 3, -3]
  /// );
  /// ```
  ///
  /// You can also rewrite this in terms of [`flat_map()`], which is preferable
  /// in this case since it conveys intent more clearly:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   // Vec is iterable because it supports IntoIterator
  ///   a.flat_map_ref(|&x| vec![x, -x]),
  ///   vec![1, -1, 2, -2, 3, -3]
  /// );
  /// ```
  ///
  /// Flattening works on any `IntoIterator` type, including `Option` and `Result`:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let options = vec![Some(123), Some(321), None, Some(231)];
  /// let results = vec![Ok(123), Ok(321), Err(456), Ok(231)];
  ///
  /// assert_eq!(options.flat(), vec![123, 321, 231]);
  ///
  /// assert_eq!(results.flat(), vec![123, 321, 231]);
  /// ```
  ///
  /// Flattening only removes one level of nesting at a time:
  ///
  /// ```
  /// use cantrip::*;
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
  /// [`flat_map()`]: CollectionTo::flat_map_ref
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
  /// The `flat_map()` method is very useful, but only when the closure
  /// argument produces values. If it produces an iterable value instead, there's
  /// an extra layer of indirection. `flat_map()` will remove this extra layer
  /// on its own.
  ///
  /// You can think of `flat_map(f)` as the semantic equivalent
  /// of mapping, and then flattening as in [`map(f)`][`.flat()`].
  ///
  /// Another way of thinking about `flat_map()`: [`map()`]'s closure returns
  /// one item for each element, and `flat_map()`'s closure returns an
  /// iterable value for each element.
  ///
  /// This is a consuming variant of [`flat_map_ref()`].
  ///
  /// [`map()`]: CollectionTo::map
  /// [`map(f)`]: CollectionTo::map
  /// [`.flat()`]: CollectionTo::flat
  /// [`flat_map_ref()`]: CollectionTo::flat_map_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   // Vec is iterable because it implements IntoIterator
  ///   a.flat_map(|x| vec![x, -x]),
  ///   vec![1, -1, 2, -2, 3, -3]
  /// );
  /// ```
  #[inline]
  fn flat_map<B, R>(self, function: impl FnMut(Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().flat_map(function).collect()
  }

  /// Creates a new collection by applying the given closure `function` to each element
  /// of this collection and flattens the nested collection.
  ///
  /// The `flat_map_ref()` method is very useful, but only when the closure
  /// argument produces values. If it produces an iterable value instead, there's
  /// an extra layer of indirection. `flat_map_ref()` will remove this extra layer
  /// on its own.
  ///
  /// You can think of `flat_map_ref(f)` as the semantic equivalent
  /// of mapping, and then flatttening as in [`map_ref(f)`][`.flat()`]`.
  ///
  /// Another way of thinking about `flat_map_ref()`: [`map_ref()`]'s closure returns
  /// one item for each element, and `flat_map_ref()`'s closure returns an
  /// iterable value for each element.
  ///
  /// This is a non-consuming variant of [`flat_map()`].
  ///
  /// [`map_ref()`]: CollectionTo::map_ref
  /// [`map_ref(f)`]: CollectionTo::map_ref
  /// [`.flat()`]: CollectionTo::flat
  /// [`flat_map()`]: CollectionTo::flat_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   // Vec is iterable because it implements IntoIterator
  ///   a.flat_map_ref(|&x| vec![x, -x]),
  ///   vec![1, -1, 2, -2, 3, -3]
  /// );
  /// ```
  fn flat_map_ref<B, R>(&self, function: impl FnMut(&Item) -> R) -> Self::This<B>
  where
    R: IntoIterator<Item = B>,
    Self::This<B>: FromIterator<B>;

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
  /// After applying this closure to every element of this collection, `fold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// This is a consuming variant of [`fold_ref()`].
  ///
  /// Note: [`reduce()`] can be used to use the first element as the initial
  /// value, if the accumulator type and item type is the same.
  ///
  /// Note: `fold()` combines elements in a *left-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *right-associative* version of `fold()`, see [`rfold()`].
  ///
  /// [`fold_ref()`]: crate::Collection::fold_ref
  /// [`reduce()`]: CollectionTo::reduce
  /// [`rfold()`]: crate::SequenceTo::rfold
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
  /// assert_eq!(
  ///   a.fold(0, |acc, x| acc + x),
  ///   6
  /// );
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
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// assert_eq!(
  ///   a.fold(zero, |acc, x| {
  ///     format!("({acc} + {x})")
  ///   }),
  ///   "(((((0 + 1) + 2) + 3) + 4) + 5)"
  /// );
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
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// let mut result = 0;
  ///
  /// // for loop:
  /// for i in &a {
  ///   result = result + i;
  /// }
  ///
  /// // fold:
  /// let result2 = a.fold(0, |acc, x| acc + x);
  ///
  /// // they're the same
  /// assert_eq!(result, result2);
  /// ```
  #[inline]
  fn fold<B>(self, initial_value: B, function: impl FnMut(B, Item) -> B) -> B
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    self.into_iter().fold(initial_value, function)
  }

  /// Creates a map of keys mapped to collections of elements according to
  /// specified discriminator function.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.group_by(|x| x % 2), HashMap::from([
  ///     (0, vec![2]),
  ///     (1, vec![1, 3])
  /// ]));
  /// ```
  fn group_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> HashMap<K, Self>
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
  {
    let iterator = self.into_iter();
    let mut result = HashMap::<K, Self>::with_capacity(iterator.size_hint().0);
    for item in iterator {
      result.entry(to_key(&item)).or_default().extend(iter::once(item));
    }
    result
  }

  /// Creates a map of keys mapped and folded to values according to
  /// specified discriminator and folding operation functions.
  ///
  /// The discriminator function takes a reference to an element and returns a group key.
  /// The folding operation takes an accumulator and a closure and returns a new element.
  /// The closure returns the value that the accumulator should have for the next iteration.
  ///
  /// This is a consuming variant of [`group_fold_ref()`].
  ///
  /// [`group_fold_ref()`]: crate::Collection::group_fold_ref
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.group_fold(|x| x % 2, 0, |acc, x| acc + x),
  ///   HashMap::from([
  ///     (0, 2),
  ///     (1, 4),
  /// ]));
  /// ```
  fn group_fold<K, B>(
    self, mut to_key: impl FnMut(&Item) -> K, initial_value: B, mut function: impl FnMut(B, Item) -> B,
  ) -> HashMap<K, B>
  where
    K: Eq + Hash,
    B: Clone,
    Self: IntoIterator<Item = Item> + Sized,
  {
    let iterator = self.into_iter();
    let mut result = HashMap::with_capacity(iterator.size_hint().0);
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
  /// This is a consuming variant of [`group_reduce_ref()`].
  ///
  /// [`group_reduce_ref()`]: crate::Collection::group_reduce_ref
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashMap;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.group_reduce(|x| x % 2, |acc, x| acc + x),
  ///   HashMap::from([
  ///     (0, 2),
  ///     (1, 4),
  /// ]));
  /// ```
  fn group_reduce<K>(
    self, mut to_key: impl FnMut(&Item) -> K, mut function: impl FnMut(Item, Item) -> Item,
  ) -> HashMap<K, Item>
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized,
  {
    let iterator = self.into_iter();
    let mut result = HashMap::with_capacity(iterator.size_hint().0);
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
  /// use [`unique()`]`.intersect()`.
  ///
  /// The order of retained values is preserved for sequences.
  ///
  /// [`unique()`]: crate::SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  /// use std::collections::HashSet;
  ///
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// let intersection = a.intersect(&vec![4, 3, 2, 2, 5]);
  ///
  /// assert_eq!(intersection, vec![2, 2, 3]);
  ///
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
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
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

  /// Creates a new collection containing the n largest elements of
  /// this collection in descending order.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![2, 1, 3];
  /// let a = vec![2, 1, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.largest(2), vec![3, 2]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.largest(4), vec![3, 2, 1]);
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
      if let Some(mut top) = heap.peek_mut() {
        if item > top.0 {
          *top = Reverse(item);
        }
      }
    }
    let result = unfold(|| heap.pop()).map(|x| x.0).collect::<Vec<_>>();
    result.into_iter().rev().collect()
  }

  /// Creates a new collection by applying the given closure `function` to each element in
  /// this collection.
  ///
  /// The closure `function` takes a reference to an element of type
  /// `Item` and returns a value of type `R`.
  /// The resulting other are collected into a new collection of the same type.
  ///
  /// This is a consuming variant of [`map_ref()`].
  ///
  /// [`map_ref()`]: CollectionTo::map_ref
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
  /// assert_eq!(
  ///   a.map(|x| x + 1),
  ///   vec![2, 3, 4]
  /// );
  /// ```
  #[inline]
  fn map<B>(self, function: impl FnMut(Item) -> B) -> Self::This<B>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().map(function).collect()
  }

  /// Creates a new collection by applying the given closure `function` to
  /// each element in this collection.
  ///
  /// The closure `function` takes a reference to an element of type
  /// `Item` and returns a value of type `R`.
  /// The resulting other are collected into a new collection of the same type.
  ///
  /// This is a non-consuming variant of [`map()`].
  ///
  /// [`map()`]: CollectionTo::map
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
  /// assert_eq!(
  ///   a.map_ref(|&x| x + 1),
  ///   vec![2, 3, 4]
  /// );
  /// ```
  fn map_ref<B>(&self, function: impl FnMut(&Item) -> B) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

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
  /// let (even, odd) = a.partition(|&x| x % 2 == 0);
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

  /// Creates a new collection containing all partitions of this collection.
  ///
  /// Partitions for sequences are generated based on element positions, not values.
  /// Therefore, if a sequence contains duplicate elements, the resulting partitionss will too.
  /// To obtain partitions of unique elements for sequences, use [`unique()`]`.partitions()`.
  ///
  /// The order of partition values is preserved for sequences.
  ///
  /// [`unique()`]: crate::SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.partitions(), vec![
  ///   vec![vec![1, 2, 3]],
  ///   vec![vec![1, 2], vec![3]],
  ///   vec![vec![1, 3], vec![2]],
  ///   vec![vec![1], vec![2, 3]],
  ///   vec![vec![1], vec![2], vec![3]],
  /// ]);
  ///
  /// assert_eq!(e.partitions(), Vec::<Vec<Vec<i32>>>::new());
  /// ```
  fn partitions(&self) -> Vec<Vec<Self>>
  where
    Item: Clone,
    Self: Sized;

  /// Creates two new collections with arbitrary element types from this collection
  /// by applying specified function.
  ///
  /// The function passed to `partition_map()` can return `Ok`, or `Err`.
  /// `partition_map()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a consuming variant of [`partition_map()`].
  ///
  /// [`partition_map()`]: CollectionTo::partition_map_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let (even, odd) = a.partition_map(|x| if x % 2 == 0 { Ok(x + 3) } else { Err(x) });
  ///
  /// assert_eq!(even, vec![5]);
  /// assert_eq!(odd, vec![1, 3]);
  /// ```
  fn partition_map<A, B>(self, mut function: impl FnMut(Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
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

  /// Creates two new collections with arbitrary element types from this collection
  /// by applying specified function.
  ///
  /// The function passed to `partition_map_ref()` can return `Ok`, or `Err`.
  /// `partition_map_ref()` returns a pair, all the `Ok` values contained, and all the `Err` values.
  ///
  /// This is a non-consuming variant of [`partition_map()`].
  ///
  /// [`partition_map()`]: CollectionTo::partition_map
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let (even, odd) = a.partition_map_ref(|&x| if x % 2 == 0 { Ok(x + 3) } else { Err(x) });
  ///
  /// assert_eq!(even, vec![5]);
  /// assert_eq!(odd, vec![1, 3]);
  /// ```
  fn partition_map_ref<A, B>(&self, function: impl FnMut(&Item) -> Result<A, B>) -> (Self::This<A>, Self::This<B>)
  where
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>;

  /// Creates a new collection containing all sub-collections of this collection.
  ///
  /// Sub-collections for sequences are generated based on element positions, not values.
  /// Therefore, if a sequence contains duplicate elements, the resulting sub-collections will too.
  /// To obtain combinations of unique elements for sequences, use [`unique()`]`.powerset()`.
  ///
  /// The order of sub-collection values is preserved for sequences.
  ///
  /// [`unique()`]: crate::SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
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
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.product(), 6);
  ///
  /// assert_eq!(e.product(), 1);
  /// ```
  #[inline]
  fn product(self) -> Item
  where
    Item: Product,
    Self: IntoIterator<Item = Item> + Sized,
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
  /// For collections with at least one element, this is the same as [`fold()`]
  /// with the first element of this collection as the initial accumulator value, folding
  /// every subsequent element into it.
  ///
  /// This is a consuming variant of [`reduce_ref()`].
  ///
  /// [`fold()`]: CollectionTo::fold
  /// [`reduce_ref()`]: crate::Collection::reduce_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.reduce(|acc, e| acc + e),
  ///   Some(6)
  /// );
  ///
  /// // Which is equivalent to doing it with `fold`:
  /// # let a = a_source.clone();
  /// let folded = a.fold(0, |acc, e| acc + e);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(
  ///   a.reduce(|acc, e| acc + e).unwrap(),
  ///   folded
  /// );
  /// ```
  #[inline]
  fn reduce(self, function: impl FnMut(Item, Item) -> Item) -> Option<Item>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    let mut iterator = self.into_iter();
    iterator.next().map(|result| iterator.fold(result, function))
  }

  /// Creates a new collection containing the n smallest elements of
  /// this collection in descending order.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![2, 3, 1];
  /// let a = vec![2, 3, 1];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.smallest(2), vec![1, 2]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.smallest(4), vec![1, 2, 3]);
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
      if let Some(mut top) = heap.peek_mut() {
        if item < *top {
          *top = item;
        }
      }
    }
    let result = unfold(|| heap.pop()).collect::<Vec<_>>();
    result.into_iter().rev().collect()
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
  /// # let a_source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.substitute(&2, 4), vec![1, 4, 2, 3]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.substitute(&4, 5), vec![1, 2, 2, 3]);
  /// assert_eq!(e.substitute(&1, 2), vec![]);
  /// ```
  #[inline]
  fn substitute(self, element: &Item, replacement: Item) -> Self
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut replaced = Some(replacement);
    self.into_iter().map(|item| if &item == element { replaced.take().unwrap_or(item) } else { item }).collect()
  }

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
  /// # let a_source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.substitute_multi(&vec![2, 3], vec![4, 5]), vec![1, 4, 2, 5]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.substitute_multi(&vec![2, 2], vec![4, 5]), vec![1, 4, 5, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.substitute_multi(&vec![2, 4], vec![4, 5]), vec![1, 4, 2, 3]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.substitute_multi(&vec![4, 5], vec![1, 1]), vec![1, 2, 2, 3]);
  /// assert_eq!(e.substitute_multi(&vec![1], vec![2]), vec![]);
  /// ```
  fn substitute_multi<'a>(
    self, elements: &'a impl Iterable<Item<'a> = &'a Item>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Item: Eq + Hash + 'a,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let elements_iterator = elements.iterator();
    let mut replaced = HashMap::<&Item, LinkedList<Item>>::with_capacity(elements_iterator.size_hint().0);
    for (item, replacement) in elements_iterator.zip(replacements.into_iter()) {
      replaced.entry(item).or_default().push_back(replacement);
    }
    self
      .into_iter()
      .map(|x| if let Some(items) = replaced.get_mut(&x) { items.pop_front().unwrap_or(x) } else { x })
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
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.sum(), 6);
  ///
  /// assert_eq!(e.sum(), 0);
  /// ```
  #[inline]
  fn sum(self) -> Item
  where
    Item: Sum,
    Self: IntoIterator<Item = Item> + Sized,
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
  /// assert_eq!(Vec::unit(1), vec![1]);
  #[inline]
  fn unit(element: Item) -> Self
  where
    Self: FromIterator<Item>,
  {
    iter::once(element).collect()
  }
}

#[inline]
pub(crate) fn combinations<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  compute_combinations(&values, k)
}

pub(crate) fn compute_combinations<'a, Item, Collection>(values: &[&Item], k: usize) -> Vec<Collection>
where
  Item: Clone + 'a,
  Collection: FromIterator<Item>,
{
  let size = values.len();
  let mut combination = Vec::from_iter(iter::once(i64::MIN).chain(0..(k as i64)));
  let mut current_slot = (size + 1).saturating_sub(k);
  unfold(|| {
    if current_slot == 0 {
      return None;
    }
    current_slot = k;
    let tuple = Some(collect_by_index(values, &combination[1..]));
    while combination[current_slot] >= (size + current_slot - k) as i64 - 1 {
      current_slot -= 1;
    }
    let mut new_index = combination[current_slot];
    for index in &mut combination[current_slot..=k] {
      new_index += 1;
      *index = new_index;
    }
    tuple
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

#[inline]
pub(crate) fn partitions<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>,
) -> Vec<Vec<Collection>> {
  let values = Vec::from_iter(iterator);
  if values.is_empty() {
    return vec![];
  }
  let size = values.len() as i64;
  let mut result = Vec::new();
  let mut stack = VecDeque::<(i64, Vec<Vec<i64>>)>::with_capacity(values.len());
  stack.push_back((0, vec![]));

  while let Some((current_index, mut partition)) = stack.pop_back() {
    if current_index == size {
      result.push(partition.iter().map(|tuple| collect_by_index(&values, tuple)).collect());
      continue;
    }
    for index in 0..partition.len() {
      let mut new_partition = partition.clone();
      new_partition[index].push(current_index);
      stack.push_front((current_index + 1, new_partition));
    }
    partition.push([current_index].to_vec());
    stack.push_front((current_index + 1, partition));
  }
  result
}

pub(crate) fn powerset<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  let sizes = 1..=values.len();
  iter::once(Collection::from_iter(iter::empty()))
    .chain(sizes.flat_map(|size| compute_combinations::<Item, Collection>(&values, size)))
    .collect()
}
