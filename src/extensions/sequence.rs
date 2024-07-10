#![allow(missing_docs)]

use crate::extensions::util::unfold::unfold;
use crate::Iterable;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, LinkedList};
use std::hash::Hash;
use std::iter;
use std::ops::RangeBounds;

/// Sequence operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a sequence
/// - May consume the collection and its elements
/// - May create a new collection
///
pub trait Sequence<Item> {
  type This<I>;

  // FIXME - add documentation
  // FIXME - implement these methods
  // coalesce
  // chunked_by
  // permutations
  // powersequence
  // slice
  // subsequence
  // variations
  // variations_repetitive

  /// Creates a collection by inserting an element into specified index
  /// in the original collection.
  ///
  /// if the specified index exceeds the collection size, no elements are inserted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2];
  /// let a = vec![1, 2];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.add_at(0, 3), [3, 1, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_at(1, 3), [1, 3, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_at(2, 3), [1, 2, 3]);
  /// assert_eq!(e.add_at(0, 1), [1]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.add_at(3, 3), [1, 2]);
  /// ```
  #[inline]
  fn add_at(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.add_all_at(index, iter::once(element))
  }

  /// Creates a collection by inserting all elements of another collection
  /// into specified index in the original collection.
  ///
  /// if the specified index exceeds the collection size, no elements are inserted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2];
  /// let a = vec![1, 2];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.add_all_at(0, vec![3, 4]), [3, 4, 1, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_all_at(1, vec![3, 4]), [1, 3, 4, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_all_at(2, vec![3, 4]), [1, 2, 3, 4]);
  /// # let a = source.clone();
  /// assert_eq!(e.add_all_at(0, vec![1, 2]), [1, 2]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.add_all_at(3, vec![3, 4]), [1, 2]);
  /// ```
  fn add_all_at(self, index: usize, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut added = elements.into_iter();
    unfold(0_usize, |position| {
      if *position >= index {
        added.next().or_else(|| {
          *position += 1;
          iterator.next()
        })
      } else {
        *position += 1;
        iterator.next()
      }
    })
    .collect()
  }

  /// Tests if all elements of the collection are equal.
  ///
  /// `all_equal()` returns `true` if all elements of the collection are equal
  /// and `false` if a pair of unequal elements exist.
  ///
  /// An empty collection returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 1, 1];
  /// let b = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert!(a.all_equal());
  /// assert!(e.all_equal());
  ///
  /// assert!(!b.all_equal());
  /// ```
  fn all_equal(&self) -> bool
  where
    Item: PartialEq;

  /// Tests if all elements of the collection are unique.
  ///
  /// `all_equal()` returns `true` if all elements of the collection are unique
  /// and `false` if a pair of equal elements exist.
  ///
  /// An empty collection returns `true`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let b = vec![1, 1, 1];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert!(a.all_unique());
  /// assert!(e.all_unique());
  ///
  /// assert!(!b.all_unique());
  /// ```
  fn all_unique(&self) -> bool
  where
    Item: Eq + Hash;

  // FIXME - fix failing test case
  /// Creates a collection containing members of k-fold cartesian product of specified size
  /// from the elements of the original collection.
  ///
  /// The order or combined values is preserved.
  /// Combinations are generated based on element positions, not values.
  ///
  /// To obtain cartesian product of unique elements, use `.unique().cartesian_product()`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.cartesian_product(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(
  ///   a.cartesian_product(2), vec![
  ///     vec![1, 1], vec![1, 2], vec![1, 3],
  ///     vec![2, 1], vec![2, 2], vec![2, 3],
  ///     vec![3, 1], vec![3, 2], vec![3, 3],
  ///   ]
  /// );
  ///
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  ///
  /// // assert_eq!(a.cartesian_product(0), empty_result);
  /// assert_eq!(a.cartesian_product(4), empty_result);
  /// assert_eq!(e.cartesian_product(2), empty_result);
  /// ```
  fn cartesian_product(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a collection by splitting the original collection elements
  /// into non-overlapping subsequences of specified `size`.
  ///
  /// The chunks are collections and do not overlap. If `size` does not divide
  /// the length of the slice, then the last chunk will not have length `size`.
  ///
  /// See [`chunked_exact`] for a variant of this function that returns chunks of always exactly
  /// `chunk_size` elements, and [`rchunked`] for the same function but starting at the
  /// end of the collection.
  ///
  /// [`chunked_exact`]: Sequence::chunked_exact
  /// [`rchunked`]: crate::Reversible::rchunked
  ///
  /// # Panics
  ///
  /// Panics if chunk `size` is 0.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, -1, 1, 2];
  /// let a = vec![1, 2, -1, 1, 2];
  ///
  /// assert_eq!(a.chunked(3), vec![vec![1, 2, -1], vec![1, 2]]);
  /// # let a = source.clone();
  /// assert_eq!(a.chunked(2), vec![vec![1, 2], vec![-1, 1], vec![2]]);
  /// # let a = source.clone();
  /// assert_eq!(a.chunked(1), vec![vec![1], vec![2], vec![-1], vec![1], vec![2]]);
  /// ```
  #[inline]
  fn chunked(self, size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    chunked(self, size, false)
  }

  // FIXME - fix failing test case
  /// Creates a collection by splitting the original collection elements
  /// into non-overlapping subsequences of specified `size`.
  ///
  /// The chunks are collections and do not overlap. If `size` does not divide
  /// the length of the slice, then the last up to `size-1` elements will be omitted.
  ///
  /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
  /// resulting code better than in the case of [`chunks`].
  ///
  /// See [`chunked`] for a variant of this function that also returns the remainder as a smaller
  /// chunk, and [`rchunked_exact`] for the same function but starting at the end of the collection.
  ///
  /// [`chunked`]: Sequence::chunked
  /// [`rchunked_exact`]: crate::Reversible::rchunked_exact
  ///
  /// # Panics
  ///
  /// Panics if chunk `size` is 0.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, -1, 1, 2];
  /// let a = vec![1, 2, -1, 1, 2];
  ///
  /// assert_eq!(a.chunked_exact(3), vec![vec![1, 2, -1]]);
  /// # let a = source.clone();
  /// assert_eq!(a.chunked_exact(2), vec![vec![1, 2], vec![-1, 1]]);
  /// # let a = source.clone();
  /// // assert_eq!(a.chunked_exact(1), vec![vec![1], vec![2], vec![-1], vec![1], vec![2]]);
  /// ```
  #[inline]
  fn chunked_exact(self, size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    chunked(self, size, true)
  }

  // FIXME - implement
  // /// Creates a collection by splitting the original collection into non-overlapping
  // /// subsequences according to specified separator predicate.
  // ///
  // /// The predicate is called for every pair of consecutive elements,
  // /// meaning that it is called on `slice[0]` and `slice[1]`,
  // /// followed by `slice[1]` and `slice[2]`, and so on.
  // ///
  // /// # Example
  // ///
  // /// ```
  // /// use cantrip::*;
  // ///
  // /// let a = vec![1, 2, -1, 1, 2];
  // ///
  // /// let chunked = a.chunked_by(|&x| x >= 0);
  // /// // FIXME - correct errors
  // /// assert_eq!(chunked, vec![vec![1, 2], vec![-1], vec![1, 2]])
  // /// ```
  // fn chunked_by(self, mut predicate: impl FnMut(&Item, &Item) -> bool) -> Self::This<Self>
  // where
  //   Self: IntoIterator<Item = Item> + Default + Extend<Item>,
  //   Self::This<Self>: Default + Extend<Self>,
  // {
  //   let mut result = Self::This::default();
  //   let mut chunk = Self::default();
  //   let mut index: usize = 0;
  //   let mut iterator = self.into_iter();
  //   iterator.next().map(|first| {
  //     let mut prev = first;
  //     for item in iterator {
  //       if index > 0 && predicate(&prev, &item) {
  //         chunk.extend(iter::once(prev));
  //         result.extend(iter::once(chunk));
  //         chunk = Self::default();
  //         index = 0;
  //       } else {
  //         chunk.extend(iter::once(prev));
  //       }
  //       chunk.extend(iter::once(item));
  //       index += 1;
  //     }
  //     if index > 0 {
  //       result.extend(iter::once(chunk));
  //     }
  //   });
  //   result
  // }

  // FIXME - implement
  // fn coalesce(self, mut function: impl FnMut(Item, Item) -> Result<Item, (Item, Item)>) -> Self
  // where
  //   Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  // {
  //   self.into_iter().scan(None, |last, item| {
  //     match last {
  //       Some(value) => match function(*value, item) {
  //         Ok(current) => None,
  //         Err((xx, current)) => None,
  //       },
  //       None => {
  //         *last = Some(item);
  //         None
  //       },
  //     }
  //   })
  //   // let mut prev = None;
  //   // let mut next = None;
  //   // self
  //   //   .into_iter()
  //   //   .filter_map(|item| match next {
  //   //     Some(value) => {
  //   //       next = None;
  //   //       Some(value)
  //   //     }
  //   //     None => match prev {
  //   //       Some(prev_value) => match function(prev_value, item) {
  //   //         Ok(value) => {
  //   //           prev = None;
  //   //           Some(value)
  //   //         }
  //   //         Err((value, next_value)) => {
  //   //           prev = None;
  //   //           next = Some(next_value);
  //   //           Some(value)
  //   //         }
  //   //       },
  //   //       None => {
  //   //         prev = Some(item);
  //   //         None
  //   //       }
  //   //     },
  //   //   })
  //     .collect()
  // }

  /// Computes the length of the longest common prefix shared by a collection and another collection.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.common_prefix_length(&vec![1, 2, 3, 4]), 3);
  /// assert_eq!(a.common_prefix_length(&vec![1, 2]), 2);
  ///
  /// assert_eq!(a.common_prefix_length(&vec![]), 0);
  /// ```
  fn common_prefix_length<'a>(&'a self, elements: &'a impl Iterable<Item<'a> = &'a Item>) -> usize
  where
    Item: PartialEq + 'a;

  #[inline]
  fn cycle(self, n: usize) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let values = self.into_iter().collect::<Vec<Item>>();
    let size = values.len() * n;
    values.into_iter().cycle().take(size).collect()
  }

  #[inline]
  fn delete_at(self, index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if i == index { None } else { Some(x) }).collect()
  }

  #[inline]
  fn delete_range(self, range: impl RangeBounds<usize>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if range.contains(&i) { None } else { Some(x) }).collect()
  }

  // FIXME - consider creating a non-consuming version or removing clone bound
  fn duplicates(self) -> Self
  where
    Item: Eq + Hash + Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut frequencies: HashMap<Item, usize> = HashMap::with_capacity(iterator.size_hint().0);
    iterator
      .flat_map(|item| {
        let count = frequencies.entry(item.clone()).or_default();
        *count += 1;
        if *count == 1 {
          Some(item)
        } else {
          None
        }
      })
      .collect()
  }

  // FIXME - consider creating a non-consuming version or removing clone bound
  fn duplicates_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut frequencies: HashMap<K, usize> = HashMap::with_capacity(iterator.size_hint().0);
    iterator
      .flat_map(|item| {
        let count = frequencies.entry(to_key(&item)).or_default();
        *count += 1;
        if *count == 1 {
          Some(item)
        } else {
          None
        }
      })
      .collect()
  }

  #[inline]
  fn enumerate(self) -> Self::This<(usize, Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(usize, Item)>: FromIterator<(usize, Item)>,
  {
    self.into_iter().enumerate().collect()
  }

  /// Creates a collection containing an element
  /// specified number of times.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// assert_eq!(Vec::fill(1, 2), vec![1, 1]);
  /// assert_eq!(Vec::fill(1, 0), vec![]);
  /// ```
  #[inline]
  fn fill(value: Item, size: usize) -> Self
  where
    Item: Clone,
    Self: FromIterator<Item>,
  {
    iter::repeat(value).take(size).collect()
  }

  fn frequencies(self) -> HashMap<Item, usize>
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized,
  {
    let iterator = self.into_iter();
    let mut result = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      *result.entry(item).or_default() += 1;
    }
    result
  }

  fn frequencies_by<K: Eq + Hash>(self, mut to_key: impl FnMut(Item) -> K) -> HashMap<K, usize>
  where
    Self: IntoIterator<Item = Item> + Sized,
  {
    let iterator = self.into_iter();
    let mut result = HashMap::with_capacity(iterator.size_hint().0);
    for item in iterator {
      *result.entry(to_key(item)).or_default() += 1;
    }
    result
  }

  // FIXME - fix failing test case
  /// Creates a new collection from the original collection without
  /// the last element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.init(), vec![1, 2]);
  ///
  /// // assert_eq!(e.init(), vec![]);
  /// ```
  #[inline]
  fn init<I>(self) -> Self
  where
    I: ExactSizeIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let size = iterator.len() - 1;
    iterator.take(size).collect()
  }

  fn interleave(self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator_left = self.into_iter();
    let mut iterator_right = elements.into_iter();
    unfold(true, |left| {
      let result = if *left {
        iterator_left.next().or(iterator_right.next())
      } else {
        iterator_right.next().or(iterator_left.next())
      };
      *left = !*left;
      result
    })
    .collect()
  }

  #[inline]
  fn interleave_shortest(self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().zip(elements).flat_map(|(item1, item2)| iter::once(item1).chain(iter::once(item2))).collect()
  }

  #[inline]
  fn intersperse(self, interval: usize, element: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.intersperse_with(interval, || element.clone())
  }

  fn intersperse_with(self, interval: usize, mut to_element: impl FnMut() -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    assert_ne!(interval, 0, "interval must be non-zero");
    let mut iterator = self.into_iter();
    let mut value = iter::repeat(to_element());
    unfold((0_usize, false), |(position, inserted)| {
      if !*inserted && *position % interval == 0 {
        *inserted = true;
        value.next()
      } else {
        *inserted = false;
        *position += 1;
        iterator.next()
      }
    })
    .collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  // FIXME - fix failing test case
  /// Creates a collection containing combinations with repetition of specified size
  /// from the elements of the original collection.
  ///
  /// The order or combined values is preserved.
  /// Combinations are generated based on element positions, not values.
  ///
  /// To obtain combination with repetition of unique elements, use `.unique().multicombinations()`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.multicombinations(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(
  ///   a.multicombinations(2),
  ///   vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 2], vec![2, 3], vec![3, 3]]
  /// );
  /// assert_eq!(
  ///   a.multicombinations(3), vec![
  ///     vec![1, 1, 1], vec![1, 1, 2], vec![1, 1, 3], vec![1, 2, 2], vec![1, 2, 3],
  ///     vec![1, 3, 3], vec![2, 2, 2], vec![2, 2, 3], vec![2, 3, 3], vec![3, 3, 3],
  ///   ]
  /// );
  ///
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  /// // assert_eq!(a.multicombinations(0), empty_result);
  /// assert_eq!(a.multicombinations(4), empty_result);
  /// assert_eq!(e.multicombinations(2), empty_result);
  /// ```
  fn multicombinations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a collection by padding the original collection to a minimum length of
  /// `size` and filling missing elements with specified value, starting from the back.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let padded = a.pad_left(5, 4);
  ///
  /// assert_eq!(padded, vec![4, 4, 1, 2, 3]);
  /// ```
  #[inline]
  fn pad_left<I>(self, size: usize, element: Item) -> Self
  where
    Item: Clone,
    I: ExactSizeIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.pad_left_with(size, |_| element.clone())
  }

  /// Creates a collection by padding the original collection to a minimum length of
  /// `size` and filling missing elements using a closure `to_element`, starting from the back.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let padded = a.pad_left_with(5, |i| 2 * i);
  ///
  /// assert_eq!(padded, vec![0, 2, 1, 2, 3]);
  /// ```
  #[inline]
  fn pad_left_with<I>(self, size: usize, mut to_element: impl FnMut(usize) -> Item) -> Self
  where
    Item: Clone,
    I: ExactSizeIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let original_start = size - iterator.len();
    unfold(0_usize, |position| {
      let result = if *position < original_start { Some(to_element(*position)) } else { iterator.next() };
      *position += 1;
      result
    })
    .collect()
  }

  /// Creates a collection by padding the original collection to a minimum length of
  /// `size` and filling missing elements with specified value.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let padded = a.pad_right(5, 4);
  ///
  /// assert_eq!(padded, vec![1, 2, 3, 4, 4]);
  /// ```
  #[inline]
  fn pad_right(self, size: usize, element: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.pad_right_with(size, |_| element.clone())
  }

  /// Creates a collection by padding the original collection to a minimum length of
  /// `size` and filling missing elements using a closure `to_element`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let padded = a.pad_right_with(5, |x| 2 * x);
  ///
  /// assert_eq!(padded, vec![1, 2, 3, 6, 8]);
  /// ```
  fn pad_right_with(self, size: usize, mut to_element: impl FnMut(usize) -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    unfold(0_usize, |position| {
      let result = iterator.next().or_else(|| if *position < size { Some(to_element(*position)) } else { None });
      *position += 1;
      result
    })
    .collect()
  }

  /// Creates a collection by moving an element at an index into specified index
  /// in the original collection.
  ///
  /// if the source index exceeds the collection size, no elements are moved.
  /// if the target index exceeds the collection size, the element is only removed.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3, 4, 5];
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(a.move_item(1, 3), vec![1, 3, 4, 2, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_item(2, 4), vec![1, 2, 4, 5, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_item(0, 5), vec![2, 3, 4, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_item(3, 1), vec![1, 4, 2, 3, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_item(4, 0), vec![5, 1, 2, 3, 4]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.move_item(3, 3), vec![1, 2, 3, 4, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_item(5, 1), vec![1, 2, 3, 4, 5]);
  /// ```
  fn move_item(self, source_index: usize, target_index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut stored = LinkedList::<Item>::new();
    unfold(0_usize, |position| {
      if *position >= source_index {
        if *position >= target_index {
          stored.pop_front().or_else(|| iterator.next())
        } else {
          if *position == source_index {
            if let Some(x) = iterator.next() {
              stored.push_back(x)
            }
          }
          *position += 1;
          iterator.next()
        }
      } else if *position >= target_index {
        let mut store = true;
        while store && *position < source_index {
          iterator.next().map(|x| stored.push_back(x)).unwrap_or_else(|| store = false);
          *position += 1;
        }
        iterator.next().or_else(|| stored.pop_front())
      } else {
        *position += 1;
        iterator.next()
      }
    })
    .collect()
  }

  // FIXME - implement
  // fn permutations(self) -> Self::This<Self>;

  /// Creates a collection by reversing the original collection's direction.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let reversed = a.rev();
  ///
  /// assert_eq!(reversed, vec![3, 2, 1]);
  /// ```
  #[inline]
  fn rev<I>(self) -> Self
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    iterator.rev().collect()
  }

  #[inline]
  fn rscan<S, B, I>(self, initial_state: S, function: impl FnMut(&mut S, Item) -> Option<B>) -> Self::This<B>
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<B>,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().rev().scan(initial_state, function).collect()
  }

  /// Creates a collection that skips the first `n` elements from the original collection.
  ///
  /// `skip(n)` skips elements until `n` elements are skipped or the end of the
  /// collection is reached (whichever happens first). After that, all the remaining
  /// elements are yielded. In particular, if the original collection is too short,
  /// then the returned collection is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.skip(2), vec![3]);
  ///
  /// assert_eq!(e.skip(1), vec![]);
  /// ```
  #[inline]
  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().skip(n).collect()
  }

  /// Creates a collection without initial elements based on a predicate.
  ///
  /// [`skip`]: Collectible::skip
  ///
  /// `skip_while()` takes a closure as an argument. It will call this
  /// closure on each element of the collection, and ignore elements
  /// until it returns `false`.
  ///
  /// After `false` is returned, `skip_while()`'s job is over, and the
  /// rest of the elements are yielded.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 0, 1];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.skip_while(|&x| x < 0), vec![0, 1]);
  ///
  /// assert_eq!(e.skip_while(|&x| x < 0), vec![]);
  /// ```
  #[inline]
  fn skip_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().skip_while(predicate).collect()
  }

  #[inline]
  fn sorted(self) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort();
    result.into_iter().collect()
  }

  #[inline]
  fn sorted_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by(compare);
    result.into_iter().collect()
  }

  #[inline]
  fn sorted_by_cached_key<K: Ord>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by_cached_key(to_key);
    result.into_iter().collect()
  }

  #[inline]
  fn sorted_by_key<K: Ord>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by_key(to_key);
    result.into_iter().collect()
  }

  #[inline]
  fn sorted_unstable(self) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable();
    result.into_iter().collect()
  }

  #[inline]
  fn sorted_unstable_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable_by(compare);
    result.into_iter().collect()
  }

  #[inline]
  fn sorted_unstable_by_key<K: Ord>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable_by_key(to_key);
    result.into_iter().collect()
  }

  #[inline]
  fn replace_at(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.replace_range(index..(index + 1), iter::once(element))
  }

  fn replace_range(self, range: impl RangeBounds<usize>, replacement: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut values = replacement.into_iter();
    unfold(0_usize, |position| {
      let item = iterator.next();
      let result = if range.contains(position) { values.next() } else { item };
      *position += 1;
      result
    })
    .collect()
  }

  #[inline]
  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().step_by(step).collect()
  }

  /// Creates a new collection from the original collection without
  /// the first element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.tail(), vec![2, 3]);
  ///
  /// assert_eq!(e.tail(), vec![]);
  /// ```
  #[inline]
  fn tail(self) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().skip(1).collect()
  }

  /// Creates a collection that yields the first `n` elements, or fewer
  /// if the original collection has fewer than `n` elements.
  ///
  /// `take(n)` yields elements until `n` elements are yielded or the end of
  /// the collection is reached (whichever happens first).
  /// The returned collection is a prefix of length `n` if the original collection
  /// contains at least `n` elements, otherwise it contains all the
  /// (fewer than `n`) elements of the original collection.
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
  /// assert_eq!(a.take(2), vec![1, 2]);
  /// ```
  ///
  /// If less than `n` elements are available,
  /// `take` will limit itself to the size of the original collection:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.take(5), vec![1, 2]);
  ///
  /// assert_eq!(e.take(1), vec![]);
  /// ```
  #[inline]
  fn take(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().take(n).collect()
  }

  /// Creates a collection without trailing elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of the collection, and yield elements
  /// while it returns `true`.
  ///
  /// After `false` is returned, `take_while()`'s job is over, and the
  /// rest of the elements are ignored.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 0, 1];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.take_while(|&x| x <= 0), vec![-1, 0]);
  ///
  /// assert_eq!(e.take_while(|&x| x <= 0), vec![]);
  /// ```
  #[inline]
  fn take_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().take_while(predicate).collect()
  }

  #[allow(unused_results)]
  fn unique(self) -> Self
  where
    Item: Eq + Hash + Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut occurred = HashSet::with_capacity(iterator.size_hint().0);
    iterator
      .flat_map(|item| {
        if !occurred.contains(&item) {
          occurred.insert(item.clone());
          Some(item)
        } else {
          None
        }
      })
      .collect()
  }

  #[allow(unused_results)]
  fn unique_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut occurred = HashSet::with_capacity(iterator.size_hint().0);
    iterator
      .filter(|item| {
        let key = to_key(item);
        if occurred.contains(&key) {
          false
        } else {
          occurred.insert(key);
          true
        }
      })
      .collect()
  }

  #[inline]
  fn unzip<A, B>(self) -> (Self::This<A>, Self::This<B>)
  where
    Self: IntoIterator<Item = (A, B)> + Sized,
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>,
  {
    self.into_iter().unzip()
  }

  fn windowed(&self, size: usize) -> Self::This<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
    Self::This<Self>: FromIterator<Self>;

  fn windowed_circular(&self, size: usize) -> Self::This<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
    Self::This<Self>: FromIterator<Self>;

  #[inline]
  fn zip<T>(self, elements: impl IntoIterator<Item = T>) -> Self::This<(Item, T)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(Item, T)>: FromIterator<(Item, T)>,
  {
    self.into_iter().zip(elements).collect()
  }
}

pub(crate) fn cartesian_product<'a, Item, Collection>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection>
where
  Item: Clone + 'a,
  Collection: FromIterator<Item> + Sized,
{
  let values = Vec::from_iter(iterator);
  let size = values.len();
  let mut combination = Vec::fill(0, k);
  unfold(k > size, |done| {
    if *done {
      return None;
    }
    let result = Some(Collection::from_iter(combination.iter().map(|index| values[*index].clone())));
    let mut current_slot = k - 1;
    while combination[current_slot] >= size - 1 {
      if current_slot > 0 {
        current_slot -= 1;
      } else {
        *done = true;
        return result;
      }
    }
    combination[current_slot] += 1;
    #[allow(clippy::needless_range_loop)]
    for slot in (current_slot + 1)..k {
      combination[slot] = 0;
    }
    result
  })
  .collect()
}

pub(crate) fn common_prefix_length<'a, Item>(
  iterator: impl Iterator<Item = &'a Item>, elements: &'a impl Iterable<Item<'a> = &'a Item>,
) -> usize
where
  Item: PartialEq + 'a,
{
  let mut result = 0_usize;
  for (item, element) in iterator.zip(elements.iterator()) {
    if item != element {
      return result;
    }
    result += 1;
  }
  result
}

pub(crate) fn multicombinations<'a, Item, Collection>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection>
where
  Item: Clone + 'a,
  Collection: FromIterator<Item> + Sized,
{
  if k == 0 {
    return Vec::from_iter(iter::once(Collection::from_iter(iter::empty())));
  }
  let values = Vec::from_iter(iterator);
  let size = values.len();
  let mut combination = Vec::fill(0, k);
  unfold(k > size, |done| {
    if *done {
      return None;
    }
    let result = Some(Collection::from_iter(combination.iter().map(|index| values[*index].clone())));
    let mut current_slot = k - 1;
    while combination[current_slot] >= size - 1 {
      if current_slot > 0 {
        current_slot -= 1;
      } else {
        *done = true;
        return result;
      }
    }
    let current_index = combination[current_slot] + 1;
    #[allow(clippy::needless_range_loop)]
    for slot in current_slot..k {
      combination[slot] = current_index;
    }
    result
  })
  .collect()
}

pub(crate) fn chunked<Item, Collection, Result>(collection: Collection, size: usize, exact: bool) -> Result
where
  Collection: IntoIterator<Item = Item> + Default + Extend<Item>,
  Result: Default + Extend<Collection>,
{
  assert_ne!(size, 0, "chunk size must be non-zero");
  let mut result = Result::default();
  let mut chunk = Collection::default();
  let mut index: usize = 0;
  for item in collection.into_iter() {
    if index > 0 && index == size {
      result.extend(iter::once(chunk));
      chunk = Collection::default();
      index = 0;
    }
    chunk.extend(iter::once(item));
    index += 1;
  }
  if index > 0 && !exact {
    result.extend(iter::once(chunk));
  }
  result
}

#[allow(unused_results)]
pub(crate) fn windowed<'a, Item, Collection, Result>(
  mut iterator: impl Iterator<Item = &'a Item>, size: usize,
) -> Result
where
  Item: Clone + 'a,
  Collection: FromIterator<Item>,
  Result: FromIterator<Collection>,
{
  assert_ne!(size, 0, "window size must be non-zero");
  let mut window: LinkedList<Item> = LinkedList::new();
  unfold((), |_| {
    while window.len() < size {
      match iterator.next() {
        Some(item) => window.push_back(item.clone()),
        None => return None,
      }
    }
    let result = Some(Collection::from_iter(window.clone()));
    window.pop_front();
    result
  })
  .collect()
}

#[allow(unused_results)]
pub(crate) fn windowed_circular<'a, Item, Collection, Result>(
  mut iterator: impl Iterator<Item = &'a Item>, size: usize,
) -> Result
where
  Item: Clone + 'a,
  Collection: FromIterator<Item>,
  Result: FromIterator<Collection>,
{
  assert_ne!(size, 0, "window size must be non-zero");
  let mut window: LinkedList<Item> = LinkedList::new();
  let mut init: LinkedList<Item> = LinkedList::new();
  unfold((), |_| {
    while window.len() < size {
      match iterator.next() {
        Some(item) => {
          window.push_back(item.clone());
          if init.len() < size - 1 {
            init.push_back(item.clone());
          }
        }
        None => match init.pop_front() {
          Some(item) => {
            window.push_back(item);
          }
          None => return None,
        },
      }
    }
    let result = Some(Collection::from_iter(window.clone()));
    window.pop_front();
    result
  })
  .collect()
}
