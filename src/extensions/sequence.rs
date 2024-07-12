#![allow(missing_docs)]

use crate::extensions::util::unfold::unfold;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};
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
  // variations
  // variations_repetitive

  /// Creates a new collection by inserting an element into specified index
  /// in the original collection.
  ///
  /// if the specified index exceeds this collection size, no elements are inserted.
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
  /// assert_eq!(a.add_at(0, 3), vec![3, 1, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_at(1, 3), vec![1, 3, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_at(2, 3), vec![1, 2, 3]);
  /// assert_eq!(e.add_at(0, 1), vec![1]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.add_at(3, 3), vec![1, 2]);
  /// ```
  #[inline]
  fn add_at(self, index: usize, addition: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.add_all_at(index, iter::once(addition))
  }

  /// Creates a new collection by inserting all elements of another collection
  /// into specified index in the original collection.
  ///
  /// if the specified index exceeds this collection size, no elements are inserted.
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
  /// assert_eq!(a.add_all_at(0, vec![3, 4]), vec![3, 4, 1, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_all_at(1, vec![3, 4]), vec![1, 3, 4, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.add_all_at(2, vec![3, 4]), vec![1, 2, 3, 4]);
  /// # let a = source.clone();
  /// assert_eq!(e.add_all_at(0, vec![1, 2]), vec![1, 2]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.add_all_at(3, vec![3, 4]), vec![1, 2]);
  /// ```
  fn add_all_at(self, index: usize, additions: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut added = additions.into_iter();
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

  // FIXME - fix failing test case
  /// Creates a new collection containing tuples of k-fold cartesian product of specified size
  /// from the elements of the original collection.
  ///
  /// Members are generated based on element positions, not values.
  /// Therefore, if this collection contains duplicate elements, the resulting tuples will too.
  /// To obtain cartesian product of unique elements, use `.unique().cartesian_product()`.
  ///
  /// The order or combined values is preserved.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// // assert_eq!(a.cartesian_product(1), vec![vec![1], vec![2], vec![3]]);
  /// // assert_eq!(
  /// //  a.cartesian_product(2), vec![
  /// //    vec![1, 1], vec![1, 2], vec![1, 3],
  /// //    vec![2, 1], vec![2, 2], vec![2, 3],
  /// //    vec![3, 1], vec![3, 2], vec![3, 3],
  /// //  ]
  /// // );
  ///
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  ///
  /// // assert_eq!(a.cartesian_product(0), empty_result);
  /// // assert_eq!(a.cartesian_product(4), empty_result);
  /// ```
  fn cartesian_product(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a new collection by splitting the original collection elements
  /// into non-overlapping subsequences of specified `size`.
  ///
  /// The chunks are collections and do not overlap. If `size` does not divide
  /// the length of the slice, then the last chunk will not have length `size`.
  ///
  /// See [`chunked_exact`] for a variant of this function that returns chunks of always exactly
  /// `chunk_size` elements, and [`rchunked`] for the same function but starting at the
  /// end of this collection.
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
  /// Creates a new collection by splitting the original collection elements
  /// into non-overlapping subsequences of specified `size`.
  ///
  /// The chunks are collections and do not overlap. If `size` does not divide
  /// the length of the slice, then the last up to `size-1` elements will be omitted.
  ///
  /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
  /// resulting code better than in the case of [`chunks`].
  ///
  /// See [`chunked`] for a variant of this function that also returns the remainder as a smaller chunk.
  ///
  /// [`chunked`]: Sequence::chunked
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

  // // FIXME - fix failing test case
  // /// Creates a new collection by splitting the original collection into non-overlapping
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
  // /// // assert_eq!(chunked, vec![vec![1, 2], vec![-1], vec![1, 2]])
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
  //     let prev = first;
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

  /// Creates a new collection by omitting an element at specified index
  /// in the original collection.
  ///
  /// if the specified index exceeds this collection size, no elements are deleted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.delete_at(0), vec![2, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.delete_at(1), vec![1, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.delete_at(2), vec![1, 2]);
  /// assert_eq!(e.delete_at(0), vec![]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.delete_at(3), vec![1, 2, 3]);
  /// ```
  #[inline]
  fn delete_at(self, index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if i == index { None } else { Some(x) }).collect()
  }

  /// Creates a new collection by omitting all elements at specified indices
  /// in the original collection.
  ///
  /// if the specified index exceeds this collection size, no elements are inserted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.delete_all_at(vec![0, 2]), vec![2]);
  /// # let a = source.clone();
  /// assert_eq!(a.delete_all_at(vec![1, 3]), vec![1, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.delete_all_at(vec![0, 1, 2, 3]), vec![]);
  ///
  /// assert_eq!(e.delete_all_at(vec![1, 2]), vec![]);
  /// # let a = source.clone();
  /// assert_eq!(a.delete_all_at(vec![3, 4]), vec![1, 2, 3]);
  /// ```
  #[inline]
  fn delete_all_at(self, indices: impl IntoIterator<Item = usize>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let positions: BTreeSet<usize> = BTreeSet::from_iter(indices);
    self.into_iter().enumerate().filter_map(|(i, x)| if positions.contains(&i) { None } else { Some(x) }).collect()
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

  /// Creates a new collection which contains original collection elements
  /// and their indices.
  ///
  /// The new collection contains pairs of `(i, val)`, where `i` is the
  /// current index of iteration and `val` is the original collection element.
  ///
  /// `enumerate()` keeps its count as an [`usize`]. If you want to count by a
  /// different sized integer, the [`zip`] function provides similar
  /// functionality.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so enumerating more than
  /// [`usize::MAX`] elements either produces the wrong result or panics. If
  /// debug assertions are enabled, a panic is guaranteed.
  ///
  /// [`zip`]: Sequence::zip
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.enumerate(), vec![(0, 1), (1, 2), (2, 3)]);
  /// ```
  #[inline]
  fn enumerate(self) -> Self::This<(usize, Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(usize, Item)>: FromIterator<(usize, Item)>,
  {
    self.into_iter().enumerate().collect()
  }

  /// Creates a new collection containing an element
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
  fn intersperse(self, interval: usize, value: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.intersperse_with(interval, || value.clone())
  }

  fn intersperse_with(self, interval: usize, mut to_value: impl FnMut() -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    assert_ne!(interval, 0, "interval must be non-zero");
    let mut iterator = self.into_iter();
    let mut value = iter::repeat(to_value());
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

  /// Creates a new collection by moving an element at an index into specified index
  /// in the original collection.
  ///
  /// if the source index exceeds this collection size, no elements are moved.
  /// if the target index exceeds this collection size, the element is only removed.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3, 4, 5];
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(a.move_at(1, 3), vec![1, 3, 4, 2, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_at(2, 4), vec![1, 2, 4, 5, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_at(0, 5), vec![2, 3, 4, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_at(3, 1), vec![1, 4, 2, 3, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_at(4, 0), vec![5, 1, 2, 3, 4]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.move_at(3, 3), vec![1, 2, 3, 4, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.move_at(5, 1), vec![1, 2, 3, 4, 5]);
  /// ```
  fn move_at(self, source_index: usize, target_index: usize) -> Self
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

  // FIXME - fix failing test case
  /// Creates a new collection containing combinations with repetition of specified size
  /// from the elements of the original collection.
  ///
  /// Combinations are generated based on element positions, not values.
  /// Therefore, if this collection contains duplicate elements, the resulting combinations will too.
  /// To obtain combination with repetition of unique elements, use `.unique().multicombinations()`.
  ///
  /// The order or combined values is preserved.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.multicombinations(0), vec![vec![]]);
  /// // assert_eq!(a.multicombinations(1), vec![vec![1], vec![2], vec![3]]);
  /// //assert_eq!(
  /// //  a.multicombinations(2),
  /// //  vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![2, 2], vec![2, 3], vec![3, 3]]
  /// //);
  /// //assert_eq!(
  /// //  a.multicombinations(3), vec![
  /// //    vec![1, 1, 1], vec![1, 1, 2], vec![1, 1, 3], vec![1, 2, 2], vec![1, 2, 3],
  /// //    vec![1, 3, 3], vec![2, 2, 2], vec![2, 2, 3], vec![2, 3, 3], vec![3, 3, 3],
  /// //  ]
  /// //);
  ///
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  /// assert_eq!(e.multicombinations(2), empty_result);
  /// ```
  fn multicombinations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a new collection by padding the original collection to a minimum length of
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
  fn pad_left<I>(self, size: usize, value: Item) -> Self
  where
    Item: Clone,
    I: ExactSizeIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    self.pad_left_with(size, |_| value.clone())
  }

  /// Creates a new collection by padding the original collection to a minimum length of
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
  fn pad_left_with<I>(self, size: usize, mut to_value: impl FnMut(usize) -> Item) -> Self
  where
    Item: Clone,
    I: ExactSizeIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let original_start = size - iterator.len();
    unfold(0_usize, |position| {
      let result = if *position < original_start { Some(to_value(*position)) } else { iterator.next() };
      *position += 1;
      result
    })
    .collect()
  }

  /// Creates a new collection by padding the original collection to a minimum length of
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
  fn pad_right(self, size: usize, value: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.pad_right_with(size, |_| value.clone())
  }

  /// Creates a new collection by padding the original collection to a minimum length of
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
  fn pad_right_with(self, size: usize, mut to_value: impl FnMut(usize) -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    unfold(0_usize, |position| {
      let result = iterator.next().or_else(|| if *position < size { Some(to_value(*position)) } else { None });
      *position += 1;
      result
    })
    .collect()
  }

  // FIXME - implement
  // fn permutations(self) -> Self::This<Self>;

  /// Creates a new collection by replacing an element at specified index
  /// in the original collection.
  ///
  /// if the specified index exceeds this collection size, no elements are replaced.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.replace_at(1, 4), vec![1, 4, 3]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.replace_at(3, 5), vec![1, 2, 3]);
  /// assert_eq!(e.replace_at(0, 1), vec![]);
  /// ```
  #[inline]
  fn replace_at(self, index: usize, replacement: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.replace_all_at(index..(index + 1), iter::once(replacement))
  }

  /// Creates a new collection by replacing all elements at specified indices in this collection
  /// by elements from another collection.
  ///
  /// if the specified index exceeds this collection size, no elements are replaced.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.replace_all_at(vec![0, 2], vec![4, 5]), vec![4, 2, 5]);
  /// # let a = source.clone();
  /// assert_eq!(a.replace_all_at(vec![1, 3], vec![4, 5]), vec![1, 4, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.replace_all_at(vec![0, 2], vec![4]), vec![4, 2, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.replace_all_at(vec![0, 2], vec![4, 5, 6]), vec![4, 2, 5]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.replace_all_at(vec![3, 4], vec![4, 5]), vec![1, 2, 3]);
  /// assert_eq!(e.replace_all_at(vec![0], vec![1]), vec![]);
  /// ```
  fn replace_all_at(
    self, indices: impl IntoIterator<Item = usize>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let positions: BTreeSet<usize> = BTreeSet::from_iter(indices);
    let mut replacement_iterator = replacements.into_iter();
    unfold(0_usize, |position| {
      iterator.next().map(|item| {
        let result = if positions.contains(position) { replacement_iterator.next().unwrap_or(item) } else { item };
        *position += 1;
        result
      })
    })
    .collect()
  }

  /// Creates a new collection by reversing the original collection's direction.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.rev(), vec![3, 2, 1]);
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

  /// A collection method that reduces this collection's elements to a single,
  /// final value, starting from the back.
  ///
  /// This is the reverse version of [`Iterator::fold()`]: it takes elements
  /// starting from the back of this collection.
  ///
  /// `rfold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of this collection, `rfold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// This is a non-consuming variant of [`rfold`].
  ///
  /// Note: `rfold()` combines elements in a *right-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *left-associative* version of `rfold()`, see [`Iterator::fold()`].
  ///
  /// [`rfold`]: crate::Ordered::rfold
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
  /// // the sum of all the elements of a
  /// let sum = a.rfold_to(0, |acc, x| acc + x);
  ///
  /// assert_eq!(sum, 6);
  /// ```
  ///
  /// This example demonstrates the right-associative nature of `rfold()`:
  /// it builds a string, starting with an initial value
  /// and continuing with each element from the back until the front:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let numbers = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// let result = numbers.rfold_to(zero, |acc, x| {
  ///   format!("({x} + {acc})")
  /// });
  ///
  /// assert_eq!(result, "(1 + (2 + (3 + (4 + (5 + 0)))))");
  /// ```
  #[inline]
  fn rfold_to<B, I>(self, initial_value: B, function: impl FnMut(B, Item) -> B) -> B
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + Sized,
  {
    let iterator = self.into_iter();
    iterator.rfold(initial_value, function)
  }

  /// A collection adapter which, like [`fold`], holds internal state, but
  /// unlike [`fold`], produces a new collection.
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
  /// [`fold`]: crate::Traversable::fold
  /// [`scan_to`]: crate::extensions::collectible::Collectible::scan_to
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4];
  ///
  /// let mut scan = a.scan(1, |state, &x| {
  ///   // each iteration, we'll multiply the state by the element ...
  ///   *state = *state * x;
  ///
  ///   // ... and terminate if the state exceeds 6
  ///   if *state > 6 {
  ///     return None;
  ///   }
  ///   // ... else yield the negation of the state
  ///   Some(-*state)
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
  /// [`scan`]: crate::extensions::collectible::Collectible::scan
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4];
  ///
  /// let mut scan = a.scan_to(1, |state, x| {
  ///   // each iteration, we'll multiply the state by the element ...
  ///   *state = *state * x;
  ///
  ///   // ... and terminate if the state exceeds 6
  ///   if *state > 6 {
  ///     return None;
  ///   }
  ///   // ... else yield the negation of the state
  ///   Some(-*state)
  /// });
  ///
  /// assert_eq!(scan, vec![-1, -2, -6]);
  /// ```
  #[inline]
  fn scan_to<S, B>(self, initial_state: S, function: impl FnMut(&mut S, Item) -> Option<B>) -> Self::This<B>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().scan(initial_state, function).collect()
  }

  /// Creates a new collection that skips the first `n` elements from the original collection.
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

  /// Creates a new collection without initial elements based on a predicate.
  ///
  /// [`skip`]: Collectible::skip
  ///
  /// `skip_while()` takes a closure as an argument. It will call this
  /// closure on each element of this collection, and ignore elements
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

  /// Creates a new collection by sorting this collection.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// When applicable, unstable sorting is preferred because it is generally faster than stable
  /// sorting, and it doesn't allocate auxiliary memory.
  /// See [`sorted_unstable`](Sequence::sorted_unstable).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is an adaptive, iterative merge sort inspired by
  /// [timsort](https://en.wikipedia.org/wiki/Timsort).
  /// It is designed to be very fast in cases where the collection is nearly sorted, or consists of
  /// two or more sorted sequences concatenated one after another.
  ///
  /// Also, it allocates temporary storage half the size of `self`, but for short collections a
  /// non-allocating insertion sort is used instead.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![-5, 4, 1, -3, 2];
  ///
  /// assert_eq!(v.sorted(), vec![-5, -3, 1, 2, 4]);
  /// ```
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

  /// Creates a new collection by sorting this collection with comparator function.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// The comparator function must define a total ordering for the elements in the collection. If
  /// the ordering is not total, the order of the elements is unspecified. An order is a
  /// total order if it is (for all `a`, `b` and `c`):
  ///
  /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  ///
  /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
  /// `partial_cmp` as our sort function when we know the collection doesn't contain a `NaN`.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let floats = vec![5_f64, 4.0, 1.0, 3.0, 2.0];
  ///
  /// let sorted = floats.sorted_by(|a, b| a.partial_cmp(b).unwrap());
  ///
  /// assert_eq!(sorted, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
  /// ```
  ///
  /// When applicable, unstable sorting is preferred because it is generally faster than stable
  /// sorting, and it doesn't allocate auxiliary memory.
  /// See [`sorted_unstable_by`](Sequence::sorted_unstable_by).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is an adaptive, iterative merge sort inspired by
  /// [timsort](https://en.wikipedia.org/wiki/Timsort).
  /// It is designed to be very fast in cases where the collection is nearly sorted, or consists of
  /// two or more sorted sequences concatenated one after another.
  ///
  /// Also, it allocates temporary storage half the size of `self`, but for short collections a
  /// non-allocating insertion sort is used instead.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![5, 4, 1, 3, 2];
  ///
  /// let sorted = v.sorted_by(|a, b| a.cmp(b));
  ///
  /// assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
  /// ```
  #[inline]
  fn sorted_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by(compare);
    result.into_iter().collect()
  }

  /// Creates a new collection by sorting this collection with a key extraction function.
  ///
  /// During sorting, the key function is called at most once per element, by using
  /// temporary storage to remember the results of key evaluation.
  /// The order of calls to the key function is unspecified and may change in future versions
  /// of the standard library.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* + *n* \* log(*n*))
  /// worst-case, where the key function is *O*(*m*).
  ///
  /// For simple key functions (e.g., functions that are property accesses or
  /// basic operations), [`sorted_by_key`](Sequence::sorted_by_key) is likely to be
  /// faster.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on collections with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// In the worst case, the algorithm allocates temporary storage in a `Vec<(K, usize)>` the
  /// length of the collection.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![-5_i32, 4, 32, -3, 2];
  ///
  /// let sorted = v.sorted_by_cached_key(|k| k.to_string());
  ///
  /// assert_eq!(sorted, vec![-3, -5, 2, 32, 4]);
  /// ```
  #[inline]
  fn sorted_by_cached_key<K: Ord>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by_cached_key(to_key);
    result.into_iter().collect()
  }

  /// Creates a new collection by sorting this collection with a key extraction function.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* \* log(*n*))
  /// worst-case, where the key function is *O*(*m*).
  ///
  /// For expensive key functions (e.g. functions that are not simple property accesses or
  /// basic operations), [`sorted_by_cached_key`](Sequence::sorted_by_cached_key) is likely to be
  /// significantly faster, as it does not recompute element keys.
  ///
  /// When applicable, unstable sorting is preferred because it is generally faster than stable
  /// sorting, and it doesn't allocate auxiliary memory.
  /// See [`sorted_unstable_by_key`](Sequence::sorted_unstable_by_key).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is an adaptive, iterative merge sort inspired by
  /// [timsort](https://en.wikipedia.org/wiki/Timsort).
  /// It is designed to be very fast in cases where the collection is nearly sorted, or consists of
  /// two or more sorted sequences concatenated one after another.
  ///
  /// Also, it allocates temporary storage half the size of `self`, but for short collections a
  /// non-allocating insertion sort is used instead.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![-5_i32, 4, 1, -3, 2];
  ///
  /// let sorted = v.sorted_by_key(|k| k.abs());
  ///
  /// assert_eq!(sorted, vec![1, 2, -3, 4, -5]);
  /// ```
  #[inline]
  fn sorted_by_key<K: Ord>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_by_key(to_key);
    result.into_iter().collect()
  }

  /// Creates a new collection by sorting this collection, but might not preserve the order of equal elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on collections with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
  /// collection consists of several concatenated sorted sequences.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![-5, 4, 1, -3, 2];
  ///
  /// assert_eq!(v.sorted_unstable(), vec![-5, -3, 1, 2, 4]);
  /// ```
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

  /// Creates a new collection by sorting this collection with a comparator function,
  /// but might not preserve the order of equal elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// The comparator function must define a total ordering for the elements in the collection. If
  /// the ordering is not total, the order of the elements is unspecified. An order is a
  /// total order if it is (for all `a`, `b` and `c`):
  ///
  /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  ///
  /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
  /// `partial_cmp` as our sort function when we know the collection doesn't contain a `NaN`.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let floats = vec![5_f64, 4.0, 1.0, 3.0, 2.0];
  ///
  /// let sorted = floats.sorted_unstable_by(|a, b| a.partial_cmp(b).unwrap());
  ///
  /// assert_eq!(sorted, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
  /// ```
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on collections with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
  /// collection consists of several concatenated sorted sequences.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![5, 4, 1, 3, 2];
  ///
  /// let sorted = v.sorted_unstable_by(|a, b| a.cmp(b));
  ///
  /// assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
  /// ```
  #[inline]
  fn sorted_unstable_by(self, compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable_by(compare);
    result.into_iter().collect()
  }

  /// Creates a new collection by sorting this collection with a key extraction function,
  /// but might not preserve the order of equal elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*m* \* *n* \* log(*n*)) worst-case, where the key function is
  /// *O*(*m*).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on slices with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// Due to its key calling strategy, [`sorted_unstable_by_key`](Sequence::sorted_unstable_by_key)
  /// is likely to be slower than [`sorted_by_cached_key`](Sequence::.sorted_by_cached_key) in
  /// cases where the key function is expensive.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let v = vec![-5_i32, 4, 1, -3, 2];
  ///
  /// let sorted = v.sorted_unstable_by_key(|k| k.abs());
  ///
  /// assert_eq!(sorted, vec![1, 2, -3, 4, -5]);
  /// ```
  #[inline]
  fn sorted_unstable_by_key<K: Ord>(self, to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut result = self.into_iter().collect::<Vec<Item>>();
    result.sort_unstable_by_key(to_key);
    result.into_iter().collect()
  }

  /// Creates a new collection by only including elements in the specified range.
  ///
  /// if the specified index exceeds this collection size, no elements are inserted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.slice(0..2), vec![1, 2]);
  /// # let a = source.clone();
  /// assert_eq!(a.slice(1..4), vec![2, 3]);
  /// # let a = source.clone();
  /// assert_eq!(a.slice(0..5), vec![1, 2, 3]);
  /// # let a = source.clone();
  /// assert_eq!(e.slice(0..1), vec![]);
  ///
  /// # let a = source.clone();
  /// assert_eq!(a.slice(3..3), vec![]);
  /// ```
  #[inline]
  fn slice(self, range: impl RangeBounds<usize>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter(|(index, _)| range.contains(index)).map(|(_, x)| x).collect()
  }

  /// Creates a new collection from this collection stepping by
  /// the given amount for each retained element.
  ///
  /// Note: The first element of the collection will always be returned,
  /// regardless of the step given.
  ///
  /// # Panics
  ///
  /// The method will panic if the given step is `0`.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![0, 1, 2, 3, 4, 5];
  ///
  /// assert_eq!(a.step_by(2), vec![0, 2, 4]);
  /// ```
  #[inline]
  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().step_by(step).collect()
  }

  /// Creates a new collection by omitting duplicate elements.
  ///
  /// Duplicates are detected using hash and equality.
  ///
  /// The algorithm is stable, returning the non-duplicate items in the order
  /// in which they occur in this collection. In a set of duplicate
  /// items, the first item encountered is the item retained.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3, 1];
  ///
  /// assert_eq!(a.unique(), vec![1, 2, 3]);
  /// ```
  #[allow(unused_results)]
  fn unique(self) -> Self
  where
    Item: Eq + Hash + Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut occurred: HashSet<Item> = HashSet::with_capacity(iterator.size_hint().0);
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

  /// Creates a new collection that yields the first `n` elements, or fewer
  /// if the original collection has fewer than `n` elements.
  ///
  /// `take(n)` yields elements until `n` elements are yielded or the end of
  /// this collection is reached (whichever happens first).
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

  /// Creates a new collection without trailing elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of this collection, and yield elements
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

  /// Creates a new collection by omitting duplicate elements.
  ///
  /// Duplicates are detected by comparing the key they map to
  /// with the result of the keying function `to_key` using hash and equality.
  ///
  /// The algorithm is stable, returning the non-duplicate items in the order
  /// in which they occur in this collection. In a set of duplicate
  /// items, the first item encountered is the item retained.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec!["a", "bb", "aa", "c", "ccc"];
  ///
  /// assert_eq!(a.unique_by(|x| x.len()), vec!["a", "bb", "ccc"]);
  /// ```
  #[allow(unused_results)]
  fn unique_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut occurred: HashSet<K> = HashSet::with_capacity(iterator.size_hint().0);
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

  /// Creates a two collection by splitting this collection of pairs.
  ///
  /// `unzip()` produces two collections: one from the left elements of the pairs,
  /// and one from the right elements.
  ///
  /// This function is, in some sense, the opposite of [`zip`].
  ///
  /// [`zip`]: Sequence::zip
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![(1, 2), (3, 4), (5, 6)];
  ///
  /// let (left, right) = a.unzip();
  ///
  /// assert_eq!(left, vec![1, 3, 5]);
  /// assert_eq!(right, vec![2, 4, 6]);
  /// ```
  #[inline]
  fn unzip<A, B>(self) -> (Self::This<A>, Self::This<B>)
  where
    Self: IntoIterator<Item = (A, B)> + Sized,
    Self::This<A>: Default + Extend<A>,
    Self::This<B>: Default + Extend<B>,
  {
    self.into_iter().unzip()
  }

  /// Creates a new collection consisting of overlapping windows of `N` elements
  /// of this collection, starting at the beginning of the collection.
  ///
  /// This is the generic equivalent of [`windows`].
  ///
  /// If `N` is greater than the size of the collection, it will return no windows.
  ///
  /// # Panics
  ///
  /// Panics if `N` is 0. This check will most probably get changed to a compile time
  /// error before this method gets stabilized.
  ///
  /// [`windows`]: slice::windows
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.windowed(2), vec![vec![1, 2], vec![2, 3]]);
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  /// assert_eq!(e.windowed(1), empty_result);
  /// ```
  fn windowed(&self, size: usize) -> Self::This<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
    Self::This<Self>: FromIterator<Self>;

  /// Creates a new collection consisting of overlapping windows of `N` elements
  /// of this collection, starting at the beginning of the collection and wrapping
  /// back to the first elements when the window would otherwise exceed this collection length.
  ///
  /// If `N` is greater than the size of the collection, it will return no windows.
  ///
  /// # Panics
  ///
  /// Panics if `N` is 0. This check will most probably get changed to a compile time
  /// error before this method gets stabilized.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e: Vec<i32> = Vec::new();
  ///
  /// assert_eq!(a.windowed_circular(2), vec![vec![1, 2], vec![2, 3], vec![3, 1]]);
  /// let empty_result: Vec<Vec<i32>> = Vec::new();
  /// assert_eq!(e.windowed(1), empty_result);
  /// ```
  fn windowed_circular(&self, size: usize) -> Self::This<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
    Self::This<Self>: FromIterator<Self>;

  /// 'Zips up' this collection with another collection into a single collection of pairs.
  ///
  /// `zip()` returns a new collection containing pairs where the first element comes from
  /// this collection, and the second element comes from the other collection.
  ///
  /// In other words, it zips two collections together, into a single one.
  ///
  /// The resulting collection length is the length of the shorter collection.
  ///
  /// To 'undo' the result of zipping up two collections, see [`unzip`].
  ///
  /// [`unzip`]: Sequence::unzip
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a1 = vec![1, 2, 3];
  /// let a2 = vec![4, 5, 6];
  ///
  /// assert_eq!(a1.zip(a2), vec![(1, 4), (2, 5), (3, 6)]);
  /// ```
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
  let mut combination = Vec::from_iter(iter::once(-2).chain(iter::repeat(0).take(k - 1)));
  unfold((size + 1).saturating_sub(k), |current_slot| {
    if *current_slot == 0 {
      return None;
    }
    *current_slot = k;
    let result = Some(collect_by_index(&values, &combination[1..]));
    while combination[*current_slot] >= (size - 1) as i64 {
      *current_slot -= 1;
    }
    combination[*current_slot] += 1;
    for index in &mut combination[(*current_slot + 1)..k] {
      *index = 0;
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

pub(crate) fn multicombinations<'a, Item, Collection>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection>
where
  Item: Clone + 'a,
  Collection: FromIterator<Item> + Sized,
{
  if k == 0 {
    return vec!(Collection::from_iter(iter::empty()));
  }
  let values = Vec::from_iter(iterator);
  let size = values.len();
  let mut combination = Vec::from_iter(iter::once(-2).chain(iter::repeat(0).take(k - 1)));
  unfold((size + 1).saturating_sub(k), |current_slot| {
    if *current_slot == 0 {
      return None;
    }
    *current_slot = k;
    let result = Some(collect_by_index(&values, &combination[1..]));
    while combination[*current_slot] >= (size - 1) as i64 {
      *current_slot -= 1;
    }
    let current_index = combination[*current_slot] + 1;
    for index in &mut combination[*current_slot..k] {
      *index = current_index;
    }
    result
  })
  .collect()
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

#[inline]
pub(crate) fn collect_by_index<Item, Result>(values: &[&Item], indices: &[i64]) -> Result
where
  Item: Clone,
  Result: FromIterator<Item>,
{
  Result::from_iter(indices.iter().map(|index| values[*index as usize].clone()))
}
