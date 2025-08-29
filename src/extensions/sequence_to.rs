use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter;

use crate::core::unfold::unfold;

/// Sequence operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent a sequence
/// - May consume the sequence and its elements
/// - May create a new sequence
pub trait SequenceTo<Item> {
  /// This sequence type constructor
  type This<I>;

  /// Creates a new sequence by inserting an element into specified index
  /// in this sequence.
  ///
  /// if the specified index exceeds this sequence size, no elements are inserted.
  ///
  /// # Panics
  ///
  /// Panics if `index` is out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.add_at(0, 4), vec![4, 1, 2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.add_at(1, 4), vec![1, 4, 2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.add_at(3, 4), vec![1, 2, 3, 4]);
  ///
  /// assert_eq!(e.add_at(0, 1), vec![1]);
  /// ```
  fn add_at(self, index: usize, element: Item) -> Self;

  /// Creates a new sequence by inserting all elements of another collection
  /// into specified index in this sequence.
  ///
  /// if the specified index exceeds this sequence size, no elements are inserted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.add_at_multi(0, vec![4, 5]), vec![4, 5, 1, 2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.add_at_multi(1, vec![4, 5]), vec![1, 4, 5, 2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.add_at_multi(3, vec![4, 5]), vec![1, 2, 3, 4, 5]);
  ///
  /// assert_eq!(e.add_at_multi(0, vec![1, 2]), vec![1, 2]);
  /// ```
  fn add_at_multi(self, index: usize, elements: impl IntoIterator<Item = Item>) -> Self;

  /// Creates a new sequence containing tuples of k-fold cartesian product of specified size
  /// from the elements of this sequence.
  ///
  /// Members are generated based on element positions, not values.
  /// Therefore, if this sequence contains duplicate elements, the resulting tuples will too.
  /// To obtain cartesian product of unique elements, use [`unique()`]`.cartesian_product()`.
  ///
  /// The order or tuple values is preserved.
  ///
  /// [`unique()`]: SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.cartesian_product(0), vec![vec![]]);
  /// assert_eq!(a.cartesian_product(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(a.cartesian_product(2), vec![
  ///   vec![1, 1],
  ///   vec![1, 2],
  ///   vec![1, 3],
  ///   vec![2, 1],
  ///   vec![2, 2],
  ///   vec![2, 3],
  ///   vec![3, 1],
  ///   vec![3, 2],
  ///   vec![3, 3],
  /// ]);
  ///
  /// assert_eq!(e.cartesian_product(2), Vec::<Vec<i32>>::new());
  /// ```
  fn cartesian_product(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a new sequence by splitting this sequence elements
  /// into non-overlapping sub-sequences of specified `size`.
  ///
  /// The chunks are sequences and do not overlap. If `size` does not divide
  /// the length of the slice, then the last chunk will not have length `size`.
  ///
  /// See [`chunked_exact()`] for a variant of this function that returns chunks of
  /// always exactly `chunk_size` elements.
  ///
  /// [`chunked_exact()`]: SequenceTo::chunked_exact
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
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.chunked(2), vec![vec![1, 2], vec![3]]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.chunked(3), vec![vec![1, 2, 3]]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.chunked(1), vec![vec![1], vec![2], vec![3]]);
  /// ```
  #[inline]
  fn chunked(self, size: usize) -> Vec<Self>
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    chunked(self, size, false)
  }

  /// Creates a new sequence by splitting this sequence into non-overlapping
  /// sub-sequences according to specified separator predicate.
  ///
  /// The `split` predicate is called for every pair of consecutive elements,
  /// meaning that it is called on `slice[0]` and `slice[1]`,
  /// followed by `slice[1]` and `slice[2]`, and so on.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// let chunked_by = a.chunked_by(|&p, &n| p > 0 && n > 2);
  /// assert_eq!(chunked_by, vec![vec![1, 2], vec![3]]);
  /// let a = a_source.clone();
  /// assert_eq!(a.chunked_by(|_, _| false), vec![vec![1, 2, 3]]);
  /// let a = a_source.clone();
  /// assert_eq!(a.chunked_by(|_, _| true), vec![vec![1], vec![2], vec![3]]);
  ///
  /// assert_eq!(e.chunked_by(|_, _| true), Vec::<Vec<i32>>::new());
  /// ```
  fn chunked_by(self, mut split: impl FnMut(&Item, &Item) -> bool) -> Vec<Self>
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut chunk_empty = true;
    let mut last = iterator.next();
    unfold(|| {
      let mut chunk_done = false;
      let chunk = unfold(|| {
        if !chunk_done && let Some(previous) = last.take() {
          if let Some(current) = iterator.next() {
            chunk_done = split(&previous, &current);
            last = Some(current);
          }
          chunk_empty = false;
          return Some(previous);
        };
        None
      })
      .collect();
      if chunk_empty {
        None
      } else {
        chunk_empty = true;
        Some(chunk)
      }
    })
    .collect()
  }

  /// Creates a new sequence by splitting this sequence elements
  /// into non-overlapping sub-sequences of specified `size`.
  ///
  /// The chunks are sequences and do not overlap. If `size` does not divide
  /// the length of the slice, then the last up to `size-1` elements will be omitted.
  ///
  /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
  /// resulting code better than in the case of [`chunked()`].
  ///
  /// See [`chunked()`] for a variant of this function that also returns the remainder as a smaller chunk.
  ///
  /// [`chunked()`]: SequenceTo::chunked
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
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.chunked_exact(2), vec![vec![1, 2]]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.chunked_exact(3), vec![vec![1, 2, 3]]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.chunked_exact(1), vec![vec![1], vec![2], vec![3]]);
  /// ```
  #[inline]
  fn chunked_exact(self, size: usize) -> Vec<Self>
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    chunked(self, size, true)
  }

  /// Creates a new sequence by using the compression closure to
  /// optionally merge together consecutive elements of this sequence.
  ///
  /// The closure `merge` is passed two elements, `previous` and `current` and may
  /// return either (1) `Ok(merged)` to merge the two values or
  /// (2) `Err((previous, current)` to indicate they can't be merged.
  /// In (2), the value `previous` is included in the new sequence.
  /// Either (1) `merged` or (2) `current` becomes the previous value
  /// when coalesce continues with the next pair of elements to merge. The
  /// value that remains at the end is also included in the new sequence.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 1, 2, 1, 2, 2, 3];
  ///
  /// let coalesced = a.coalesce(|p, n| if p % 2 == n % 2 { Ok(p + n) } else { Err((p, n)) });
  ///
  /// assert_eq!(coalesced, vec![4, 1, 4, 3]);
  /// ```
  fn coalesce(self, mut function: impl FnMut(Item, Item) -> Result<Item, (Item, Item)>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut last = iterator.next();
    unfold(|| {
      loop {
        if let Some(previous) = last.take() {
          if let Some(current) = iterator.next() {
            match function(previous, current) {
              Ok(merged) => last = Some(merged),
              Err((new_previous, new_current)) => {
                last = Some(new_current);
                return Some(new_previous);
              }
            }
          } else {
            return Some(previous);
          }
        } else {
          return None;
        }
      }
    })
    .collect()
  }

  /// Creates a new sequence containing combinations with repetition of specified size
  /// from the elements of this sequence.
  ///
  /// Combinations are generated based on element positions, not values.
  /// Therefore, if this sequence contains duplicate elements, the resulting combinations will too.
  /// To obtain combination with repetition of unique elements, use [`unique()`]`.multicombinations()`.
  ///
  /// The order or combination values is preserved.
  ///
  /// [`unique()`]: SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.combinations_multi(0), vec![vec![]]);
  /// assert_eq!(a.combinations_multi(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(a.combinations_multi(2), vec![
  ///   vec![1, 1],
  ///   vec![1, 2],
  ///   vec![1, 3],
  ///   vec![2, 2],
  ///   vec![2, 3],
  ///   vec![3, 3]
  /// ]);
  /// assert_eq!(a.combinations_multi(3), vec![
  ///   vec![1, 1, 1],
  ///   vec![1, 1, 2],
  ///   vec![1, 1, 3],
  ///   vec![1, 2, 2],
  ///   vec![1, 2, 3],
  ///   vec![1, 3, 3],
  ///   vec![2, 2, 2],
  ///   vec![2, 2, 3],
  ///   vec![2, 3, 3],
  ///   vec![3, 3, 3],
  /// ]);
  ///
  /// assert_eq!(e.combinations_multi(1), Vec::<Vec<i32>>::new());
  /// ```
  fn combinations_multi(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a new sequence by omitting an element at specified index
  /// in this sequence.
  ///
  /// if the specified index exceeds this sequence size, no elements are deleted.
  ///
  /// # Panics
  ///
  /// Panics if `index` is out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.delete_at(0), vec![2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.delete_at(1), vec![1, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.delete_at(2), vec![1, 2]);
  /// ```
  fn delete_at(self, index: usize) -> Self;

  /// Creates a new sequence by omitting elements at specified indices
  /// in this sequence.
  ///
  /// if the specified index exceeds this sequence size, no elements are inserted.
  ///
  /// # Panics
  ///
  /// Panics if any of the `indices` is out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.delete_at_multi(vec![0, 2]), vec![2]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.delete_at_multi(vec![0, 1, 2]), vec![]);
  /// ```
  #[inline]
  fn delete_at_multi(self, indices: impl IntoIterator<Item = usize>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let positions = BTreeSet::from_iter(indices);
    self.into_iter().enumerate().filter_map(|(i, x)| if positions.contains(&i) { None } else { Some(x) }).collect()
  }

  /// Creates a new sequence by splitting this sequence into sub-sequences separated
  /// by elements equal to the specified `separator` value.
  /// Matched elements are not contained in the sub-sequences.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.divide(&2), vec![vec![1], vec![3]]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.divide(&0), vec![vec![1, 2, 3]]);
  /// ```
  ///
  /// If the first element is matched, an empty sequence will be the first
  /// element of the result. Similarly, if the last element in the sequence
  /// is matched, an empty sequence will be the last element of the result:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.divide(&1), vec![vec![], vec![2, 3]]);
  /// ```
  ///
  /// If two matched elements are directly adjacent, an empty sequence will be
  /// present between them:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.divide(&2), vec![vec![1], vec![], vec![3]]);
  /// ```
  #[inline]
  fn divide(self, separator: &Item) -> Vec<Self>
  where
    Item: PartialEq,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.divide_by(|x| x == separator)
  }

  /// Creates a new sequence by splitting this sequence into sub-sequences separated
  /// by elements that match the `separator` predicate.
  /// Matched elements are not contained in the sub-sequences.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.divide_by(|x| x % 2 == 0), vec![vec![1], vec![3]]);
  /// ```
  ///
  /// If the first element is matched, an empty sequence will be the first
  /// element of the result. Similarly, if the last element in the sequence
  /// is matched, an empty sequence will be the last element of the result:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(a.divide_by(|x| x % 2 == 1), vec![vec![], vec![2], vec![4], vec![]]);
  /// ```
  ///
  /// If two matched elements are directly adjacent, an empty sequence will be
  /// present between them:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.divide_by(|x| x % 2 == 0), vec![vec![1], vec![], vec![3]]);
  /// ```
  #[inline]
  fn divide_by(self, mut separator: impl FnMut(&Item) -> bool) -> Vec<Self>
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut empty = false;
    unfold(|| {
      if empty {
        return None;
      }
      let chunk = unfold(|| {
        if let Some(item) = iterator.next() {
          if !separator(&item) {
            return Some(item);
          }
        } else {
          empty = true;
        }
        None
      })
      .collect();
      Some(chunk)
    })
    .collect()
  }

  /// Creates a new collection by including only the elements of this collection
  /// that appear more than once.
  ///
  /// Duplicates are detected using hash and equality and each duplicate is included exactly once.
  ///
  /// The order or duplicate values is preserved.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.duplicates(), vec![2]);
  /// ```
  fn duplicates(self) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let mut occurred = HashSet::with_capacity(iterator.size_hint().0);
    let mut duplicated = HashSet::with_capacity(iterator.size_hint().0);
    iterator
      .flat_map(|item| {
        if !duplicated.contains(&item) {
          if let Some(result) = occurred.take(&item) {
            let _ = duplicated.insert(item);
            return Some(result);
          } else {
            let _ = occurred.insert(item);
          }
        }
        None
      })
      .collect()
  }

  /// Creates a new collection by including only the elements of this collection
  /// that appear more than once.
  ///
  /// Duplicates are detected by comparing the key they map to with the keying function
  /// `to_key` using hash and equality and each duplicate is included exactly once.
  ///
  /// The order or duplicate values is preserved.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.duplicates_by(|x| x % 2), vec![1, 3]);
  /// ```
  fn duplicates_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut occurred = HashMap::<K, Option<Item>>::with_capacity(iterator.size_hint().0);
    let mut stored: Option<Item> = None;
    unfold(|| {
      stored.take().or_else(|| {
        loop {
          if let Some(item) = iterator.next() {
            let key = to_key(&item);
            if let Some(value) = occurred.get_mut(&key) {
              stored = Some(item);
              return value.take();
            } else {
              let _unused = occurred.insert(key, Some(item));
            }
          } else {
            return None;
          }
        }
      })
    })
    .collect()
  }

  /// Creates a new sequence which contains elements of this sequence
  /// and their indices.
  ///
  /// The new sequence contains pairs of `(i, val)`, where `i` is the
  /// current index of iteration and `val` is an element of this sequence.
  ///
  /// `enumerate()` keeps its count as an [`usize`]. If you want to count by a
  /// different sized integer, the [`zip()`] function provides similar
  /// functionality.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so enumerating more than
  /// [`usize::MAX`] elements either produces the wrong result or panics. If
  /// debug assertions are enabled, a panic is guaranteed.
  ///
  /// [`zip()`]: SequenceTo::zip
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

  /// Creates a new sequence containing an element
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
  fn fill(element: Item, size: usize) -> Self
  where
    Item: Clone,
    Self: FromIterator<Item>,
  {
    iter::repeat_n(element, size).collect()
  }

  /// Creates a new sequence from this sequence without
  /// the last element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.init(), vec![1, 2]);
  ///
  /// assert_eq!(e.init(), vec![]);
  /// ```
  fn init(self) -> Self;

  /// Create a new sequence by interleaving the elements of this sequence with
  /// the elements of another collection.
  ///
  /// If one sequence is longer than another, the remaining elements of the
  /// longer sequence are added to the end of the new collection.
  ///
  /// Elements are added to the new collection in an alternating fashion.
  /// The first element comes from this sequence, the second element
  /// comes from the other collection and so on.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.interleave(vec![4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.interleave(vec![4, 5]), vec![1, 4, 2, 5, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.interleave(vec![]), vec![1, 2, 3]);
  /// ```
  fn interleave(self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator_left = self.into_iter();
    let mut iterator_right = elements.into_iter();
    let mut left = true;
    unfold(|| {
      let new_item = if left {
        iterator_left.next().or_else(|| iterator_right.next())
      } else {
        iterator_right.next().or_else(|| iterator_left.next())
      };
      left = !left;
      new_item
    })
    .collect()
  }

  /// Create a new sequence by interleaving the elements of this sequence with
  /// the elements of another collection.
  ///
  /// If one sequence is longer than another, the remaining elements of the
  /// longer sequence are omitted.
  ///
  /// Elements are added to the new collection in an alternating fashion.
  /// The first element comes from this sequence, the second element
  /// comes from the other collection and so on.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.interleave_exact(vec![4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.interleave_exact(vec![4, 5]), vec![1, 4, 2, 5]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.interleave_exact(vec![]), vec![]);
  /// ```
  #[inline]
  fn interleave_exact(self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().zip(elements).flat_map(|(item1, item2)| iter::once(item1).chain(iter::once(item2))).collect()
  }

  /// Creates a new sequence which places a copy of `separator` between
  /// elements of the original sequence with the distance between the inserted
  /// values determined by the specified `interval`.
  ///
  /// In case `separator` does not implement [`Clone`] or needs to be
  /// computed every time, use [`intersperse_with`].
  ///
  /// [`intersperse_with`]: SequenceTo::intersperse_with
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.intersperse(1, 0), vec![1, 0, 2, 0, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.intersperse(2, 0), vec![1, 2, 0, 3]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.intersperse(3, 0), vec![1, 2, 3]);
  /// ```
  #[inline]
  fn intersperse(self, interval: usize, element: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.intersperse_with(interval, || element.clone())
  }

  /// Creates a new sequence which places a value generated by `to_element`
  /// between elements of the original sequence with the distance between the
  /// inserted values determined by the specified `interval`.
  ///
  /// The specified closure will be called exactly once each time an item is
  /// placed between two adjacent items from the underlying sequence.
  /// The closure is not called if the underlying sequence yields less than
  /// two items and after the last item is yielded.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.intersperse_with(1, || 0), vec![1, 0, 2, 0, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.intersperse_with(2, || 0), vec![1, 2, 0, 3]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.intersperse_with(3, || 0), vec![1, 2, 3]);
  /// ```
  fn intersperse_with(self, interval: usize, mut to_value: impl FnMut() -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    assert_ne!(interval, 0, "interval must be non-zero");
    let mut iterator = self.into_iter();
    let mut index = 0_usize;
    let mut stored: Option<Item> = None;
    unfold(|| {
      stored.take().or_else(|| {
        iterator.next().map(|item| {
          let new_item = if index != 0 && index.is_multiple_of(interval) {
            stored = Some(item);
            to_value()
          } else {
            item
          };
          index += 1;
          new_item
        })
      })
    })
    .collect()
  }

  /// Creates a new sequence without trailing elements based on a predicate
  /// and a map the retained elements function.
  ///
  /// `map_while()` takes a closure as an argument. It will call this
  /// closure on each element of the sequence, and include elements
  /// while it returns [`Some(_)`][`Some`].
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
  /// assert_eq!(a.map_while(|&x| if x < 3 { Some(x + 1) } else { None }), vec![2, 3]);
  /// ```
  ///
  /// Here's the same example, but with [`take_while`] and [`map`]:
  ///
  /// [`take_while`]: Iterator::take_while
  /// [`map`]: Iterator::map
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(
  ///   a.map_ref(|&x| if x < 3 { Some(x + 1) } else { None })
  ///     .take_while(|x| x.is_some())
  ///     .map_ref(|x| x.unwrap()),
  ///   vec![2, 3]
  /// );
  /// ```
  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  /// Create a new sequence by merging it with another sequence in ascending order.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.merge(vec![0, 4, 5]), vec![0, 1, 2, 3, 4, 5]);
  /// ```
  fn merge(self, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Item: Ord,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.merge_by(elements, |l, r| l.cmp(r))
  }

  /// Create a new sequence by merging it with another sequence in ascending order
  /// with respect to the specified comparison function.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.merge_by(vec![0, 4, 5], |l, r| l.cmp(r)), vec![0, 1, 2, 3, 4, 5]);
  /// ```
  fn merge_by(self, elements: impl IntoIterator<Item = Item>, mut compare: impl FnMut(&Item, &Item) -> Ordering) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut elements_iterator = elements.into_iter();
    let mut last_left = iterator.next();
    let mut last_right = elements_iterator.next();
    unfold(|| match (last_left.take(), last_right.take()) {
      (Some(left), Some(right)) => {
        if compare(&left, &right) == Ordering::Less {
          last_left = iterator.next();
          last_right = Some(right);
          Some(left)
        } else {
          last_left = Some(left);
          last_right = elements_iterator.next();
          Some(right)
        }
      }
      (Some(left), None) => {
        last_left = iterator.next();
        Some(left)
      }
      (None, Some(right)) => {
        last_right = elements_iterator.next();
        Some(right)
      }
      (None, None) => None,
    })
    .collect()
  }

  /// Creates a new sequence by moving an element at an index into specified index
  /// in this sequence.
  ///
  /// If the target index exceeds this sequence size, the element is only removed.
  /// If the source index exceeds this sequence size, no elements are moved.
  ///
  /// # Panics
  ///
  /// Panics if `source_index` or `target_index` are out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.move_at(0, 2), vec![2, 3, 1]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.move_at(2, 1), vec![1, 3, 2]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.move_at(1, 1), vec![1, 2, 3]);
  /// ```
  fn move_at(self, source_index: usize, target_index: usize) -> Self;

  /// Creates a new sequence by padding this sequence to a minimum length of `size`
  /// and filling missing elements with specified value, starting from the back.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.pad_left(5, 4), vec![4, 4, 1, 2, 3]);
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

  /// Creates a new sequence by padding this sequence to a minimum length of `size`
  /// and filling missing elements using a closure `to_element`, starting from the back.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.pad_left_with(5, |i| 2 * i), vec![0, 2, 1, 2, 3]);
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
    let mut index = 0_usize;
    unfold(|| {
      let new_item = if index < original_start { Some(to_element(index)) } else { iterator.next() };
      index += 1;
      new_item
    })
    .collect()
  }

  /// Creates a new sequence by padding this sequence to a minimum length of
  /// `size` and filling missing elements with specified value.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.pad_right(5, 4), vec![1, 2, 3, 4, 4]);
  /// ```
  #[inline]
  fn pad_right(self, size: usize, element: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.pad_right_with(size, |_| element.clone())
  }

  /// Creates a new sequence by padding this sequence to a minimum length of
  /// `size` and filling missing elements using a closure `to_element`.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.pad_right_with(5, |x| 2 * x), vec![1, 2, 3, 6, 8]);
  /// ```
  fn pad_right_with(self, size: usize, mut to_element: impl FnMut(usize) -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut index = 0_usize;
    unfold(|| {
      let new_item = iterator.next().or_else(|| if index < size { Some(to_element(index)) } else { None });
      index += 1;
      new_item
    })
    .collect()
  }

  /// Creates a new sequence by reversing this sequence's direction.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
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
    self.into_iter().rev().collect()
  }

  /// Reduces this sequence's elements to a single, final value, starting from the back.
  ///
  /// This is the reverse version of [`Iterator::fold()`]: it takes elements
  /// starting from the back of this sequence.
  ///
  /// `rfold()` takes two arguments: an initial value, and a closure with two
  /// arguments: an 'accumulator', and an element. The closure returns the value that
  /// the accumulator should have for the next iteration.
  ///
  /// The initial value is the value the accumulator will have on the first
  /// call.
  ///
  /// After applying this closure to every element of this sequence, `rfold()`
  /// returns the accumulator.
  ///
  /// This operation is sometimes called 'reduce' or 'inject'.
  ///
  /// Folding is useful whenever you have a collection of something, and want
  /// to produce a single value from it.
  ///
  /// This is a consuming variant of [`rfold_ref()`].
  ///
  /// Note: `rfold()` combines elements in a *right-associative* fashion. For associative
  /// operators like `+`, the order the elements are combined in is not important, but for non-associative
  /// operators like `-` the order will affect the final result.
  /// For a *left-associative* version of `rfold()`, see [`fold()`].
  ///
  /// [`rfold_ref()`]: crate::Sequence::rfold_ref
  /// [`fold()`]: crate::CollectionTo::fold
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
  /// assert_eq!(a.rfold_ref(0, |acc, &x| acc + x), 6);
  /// ```
  ///
  /// This example demonstrates the right-associative nature of `rfold_to()`:
  /// it builds a string, starting with an initial value
  /// and continuing with each element from the back until the front:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3, 4, 5];
  ///
  /// let zero = "0".to_string();
  ///
  /// assert_eq!(
  ///   a.rfold(zero, |acc, x| { format!("({x} + {acc})") }),
  ///   "(1 + (2 + (3 + (4 + (5 + 0)))))"
  /// );
  /// ```
  #[inline]
  fn rfold<B, I>(self, initial_value: B, function: impl FnMut(B, Item) -> B) -> B
  where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + Sized,
  {
    self.into_iter().rfold(initial_value, function)
  }

  /// A sequence adapter which, like [`fold()`], holds internal state, but
  /// unlike [`fold()`], produces a new sequence.
  ///
  /// `scan()` takes two arguments: an initial value which seeds the internal
  /// state, and a closure with two arguments, the first being a mutable
  /// reference to the internal state and the second element of this sequence.
  /// The closure can assign to the internal state to share state between
  /// iterations.
  ///
  /// On iteration, the closure will be applied to each element of this
  /// sequence and the return value from the closure, an [`Option`], is
  /// returned by the `next` method. The closure can return
  /// `Some(value)` to yield `value`, or `None` to end the iteration.
  ///
  /// This is a consuming variant of [`scan()`].
  ///
  /// [`fold()`]: crate::CollectionTo::fold
  /// [`scan()`]: SequenceTo::scan_ref
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let mut scan = a.scan(1, |state, x| {
  ///   // each iteration, we'll multiply the state by the element ...
  ///   *state = *state * x;
  ///
  ///   // ... and terminate if the state exceeds 6
  ///   if *state > 2 {
  ///     return None;
  ///   }
  ///   // ... else yield the negation of the state
  ///   Some(-*state)
  /// });
  ///
  /// assert_eq!(scan, vec![-1, -2]);
  /// ```
  #[inline]
  fn scan<S, B>(self, initial_state: S, function: impl FnMut(&mut S, Item) -> Option<B>) -> Self::This<B>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<B>: FromIterator<B>,
  {
    self.into_iter().scan(initial_state, function).collect()
  }

  /// A sequence adapter which, like [`fold_ref()`], holds internal state, but
  /// unlike [`fold_ref()`], produces a new sequence.
  ///
  /// `scan_ref()` takes two arguments: an initial value which seeds the internal
  /// state, and a closure with two arguments, the first being a mutable
  /// reference to the internal state and the second element of this sequence.
  /// The closure can assign to the internal state to share state between
  /// iterations.
  ///
  /// On iteration, the closure will be applied to each element of this
  /// sequence and the return value from the closure, an [`Option`], is
  /// returned by the `next` method. The closure can return
  /// `Some(value)` to yield `value`, or `None` to end the iteration.
  ///
  /// This is a non-consuming variant of [`scan()`].
  ///
  /// [`fold_ref()`]: crate::Collection::fold_ref
  /// [`scan()`]: SequenceTo::scan
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// let mut scan = a.scan(1, |state, x| {
  ///   // each iteration, we'll multiply the state by the element ...
  ///   *state = *state * x;
  ///
  ///   // ... and terminate if the state exceeds 6
  ///   if *state > 2 {
  ///     return None;
  ///   }
  ///   // ... else yield the negation of the state
  ///   Some(-*state)
  /// });
  ///
  /// assert_eq!(scan, vec![-1, -2]);
  /// ```
  fn scan_ref<S, B>(&self, initial_state: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>
  where
    Self::This<B>: FromIterator<B>;

  /// Creates a new sequence that skips the first `n` elements from this sequence.
  ///
  /// `skip(n)` skips elements until `n` elements are skipped or the end of this
  /// sequence is reached (whichever happens first). After that, all the remaining
  /// elements are yielded. In particular, if this sequence is too short,
  /// then the returned sequence is empty.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.skip(2), vec![3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.skip(5), vec![]);
  /// ```
  #[inline]
  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().skip(n).collect()
  }

  /// Creates a new sequence without initial elements based on a predicate.
  ///
  /// [`skip`]: Collectible::skip
  ///
  /// `skip_while()` takes a closure as an argument. It will call this
  /// closure on each element of this sequence, and ignore elements
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
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.skip_while(|&x| x < 3), vec![3]);
  /// ```
  #[inline]
  fn skip_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().skip_while(predicate).collect()
  }

  /// Creates a new sequence by only including elements in the specified range.
  ///
  /// if the specified index exceeds this sequence size, no elements are inserted.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.slice(0, 2), vec![1, 2]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.slice(1, 3), vec![2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.slice(1, 1), vec![]);
  /// ```
  #[inline]
  fn slice<I>(self, start_index: usize, end_index: usize) -> Self
  where
    I: ExactSizeIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let size = iterator.len();
    if start_index > size {
      panic!(r#"start index (is {start_index:?}) should be <= len (is {size:?})"#)
    }
    if end_index > size {
      panic!(r#"end index (is {end_index:?}) should be <= len (is {size:?})"#)
    }
    iterator.enumerate().filter(|(index, _)| *index >= start_index && *index < end_index).map(|(_, x)| x).collect()
  }

  /// Creates a new sequence by sorting this sequence.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// When applicable, unstable sorting is preferred because it is generally faster than stable
  /// sorting, and it doesn't allocate auxiliary memory.
  /// See [`sorted_unstable()`](SequenceTo::sorted_unstable).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is an adaptive, iterative merge sort inspired by
  /// [timsort](https://en.wikipedia.org/wiki/Timsort).
  /// It is designed to be very fast in cases where this sequence is nearly sorted, or consists of
  /// two or more sorted sequences concatenated one after another.
  ///
  /// Also, it allocates temporary storage half the size of `self`, but for short sequences a
  /// non-allocating insertion sort is used instead.
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted(), vec![1, 2, 3]);
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

  /// Creates a new sequence by sorting this sequence with comparator function.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// The comparator function must define a total ordering for the elements in this sequence. If
  /// the ordering is not total, the order of the elements is unspecified. An order is a
  /// total order if it is (for all `a`, `b` and `c`):
  ///
  /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  ///
  /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
  /// `partial_cmp` as our sort function when we know this sequence doesn't contain a `NaN`.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_by(|a, b| a.partial_cmp(b).unwrap()), vec![1, 2, 3]);
  /// ```
  ///
  /// When applicable, unstable sorting is preferred because it is generally faster than stable
  /// sorting, and it doesn't allocate auxiliary memory.
  /// See [`sorted_unstable_by()`](SequenceTo::sorted_unstable_by).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is an adaptive, iterative merge sort inspired by
  /// [timsort](https://en.wikipedia.org/wiki/Timsort).
  /// It is designed to be very fast in cases where this sequence is nearly sorted, or consists of
  /// two or more sorted sequences concatenated one after another.
  ///
  /// Also, it allocates temporary storage half the size of `self`, but for short sequences a
  /// non-allocating insertion sort is used instead.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_by(|a, b| a.cmp(b)), vec![1, 2, 3]);
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

  /// Creates a new sequence by sorting this sequence with a key extraction function.
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
  /// basic operations), [`sorted_by_key()`](SequenceTo::sorted_by_key) is likely to be
  /// faster.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on sequences with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// In the worst case, the algorithm allocates temporary storage in a `Vec<(K, usize)>` the
  /// length of this sequence.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_by_cached_key(|k| k.to_string()), vec![1, 2, 3]);
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

  /// Creates a new sequence by sorting this sequence with a key extraction function.
  ///
  /// This sort is stable (i.e., does not reorder equal elements) and *O*(*m* \* *n* \* log(*n*))
  /// worst-case, where the key function is *O*(*m*).
  ///
  /// For expensive key functions (e.g. functions that are not simple property accesses or
  /// basic operations), [`sorted_by_cached_key()`](SequenceTo::sorted_by_cached_key) is likely to be
  /// significantly faster, as it does not recompute element keys.
  ///
  /// When applicable, unstable sorting is preferred because it is generally faster than stable
  /// sorting, and it doesn't allocate auxiliary memory.
  /// See [`sorted_unstable_by_key()`](SequenceTo::sorted_unstable_by_key).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is an adaptive, iterative merge sort inspired by
  /// [timsort](https://en.wikipedia.org/wiki/Timsort).
  /// It is designed to be very fast in cases where this sequence is nearly sorted, or consists of
  /// two or more sorted sequences concatenated one after another.
  ///
  /// Also, it allocates temporary storage half the size of `self`, but for short sequences a
  /// non-allocating insertion sort is used instead.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_by_key(|&k| -k), vec![3, 2, 1]);
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

  /// Creates a new sequence by sorting this sequence, but might not preserve the order of equal elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on sequences with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// It is typically faster than stable sorting, except in a few special cases, e.g., when this
  /// sequence consists of several concatenated sorted sequences.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_unstable(), vec![1, 2, 3]);
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

  /// Creates a new sequence by sorting this sequence with a comparator function,
  /// but might not preserve the order of equal elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// The comparator function must define a total ordering for the elements in this sequence. If
  /// the ordering is not total, the order of the elements is unspecified. An order is a
  /// total order if it is (for all `a`, `b` and `c`):
  ///
  /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  ///
  /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
  /// `partial_cmp` as our sort function when we know this sequence doesn't contain a `NaN`.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_unstable_by(|a, b| a.partial_cmp(b).unwrap()), vec![1, 2, 3]);
  /// ```
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on sequences with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// It is typically faster than stable sorting, except in a few special cases, e.g., when this
  /// sequence consists of several concatenated sorted sequences.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_unstable_by(|a, b| a.cmp(b)), vec![1, 2, 3]);
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

  /// Creates a new sequence by sorting this sequence with a key extraction function,
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
  /// Due to its key calling strategy, [`sorted_unstable_by_key()`](SequenceTo::sorted_unstable_by_key)
  /// is likely to be slower than [`sorted_by_cached_key()`](Sequence::.sorted_by_cached_key) in
  /// cases where the key function is expensive.
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![2, 3, 1];
  ///
  /// assert_eq!(a.sorted_unstable_by_key(|&k| -k), vec![3, 2, 1]);
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

  /// Creates a new sequence from this sequence stepping by
  /// the given amount for each retained element.
  ///
  /// Note: The first element of this sequence will always be returned,
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
  /// # let a_source = vec![1, 2, 2, 3];
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.step_by(3), vec![1, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.step_by(2), vec![1, 2]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.step_by(1), vec![1, 2, 2, 3]);
  /// ```
  #[inline]
  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().step_by(step).collect()
  }

  /// Creates a new sequence by replacing an element at specified index
  /// in this sequence.
  ///
  /// if the specified index exceeds this sequence size, no elements are replaced.
  ///
  /// # Panics
  ///
  /// Panics if `index` is out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.substitute_at(1, 4), vec![1, 4, 3]);
  /// ```
  fn substitute_at(self, index: usize, replacement: Item) -> Self;

  /// Creates a new sequence by replacing all elements at specified indices in this sequence
  /// by elements from another collection.
  ///
  /// if the specified index exceeds this sequence size, no elements are replaced.
  ///
  /// # Panics
  ///
  /// Panics if any of the `indices` is out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.substitute_at_multi(vec![0, 2], vec![4, 5]), vec![4, 2, 5]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.substitute_at_multi(vec![0, 2], vec![4]), vec![4, 2, 3]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.substitute_at_multi(vec![0, 2], vec![4, 5, 6]), vec![4, 2, 5]);
  /// ```
  fn substitute_at_multi(
    self, indices: impl IntoIterator<Item = usize>, replacements: impl IntoIterator<Item = Item>,
  ) -> Self;

  /// Creates a new sequence by swapping an elements at specified indices
  /// in this sequence.
  ///
  /// If one of the indices exceeds this sequence size, the element is only removed.
  /// If both indices index exceed this sequence size, no elements are swapped.
  ///
  /// # Panics
  ///
  /// Panics if `source_index` or `target_index` are out of bounds.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.swap_at(0, 2), vec![3, 2, 1]);
  ///
  /// # let a = a_source.clone();
  /// assert_eq!(a.swap_at(1, 1), vec![1, 2, 3]);
  /// ```
  fn swap_at(self, source_index: usize, target_index: usize) -> Self;

  /// Creates a new sequence from this sequence without
  /// the first element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.tail(), vec![2, 3]);
  ///
  /// assert_eq!(e.tail(), vec![]);
  /// ```
  fn tail(self) -> Self;

  /// Creates a new sequence that yields the first `n` elements, or fewer
  /// if this sequence has fewer than `n` elements.
  ///
  /// `take(n)` yields elements until `n` elements are yielded or the end of
  /// this sequence is reached (whichever happens first).
  /// The returned sequence is a prefix of length `n` if this sequence
  /// contains at least `n` elements, otherwise it contains all the
  /// (fewer than `n`) elements of this sequence.
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
  /// `take` will limit itself to the size of this sequence:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.take(2), vec![1, 2]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.take(5), vec![1, 2, 3]);
  /// ```
  #[inline]
  fn take(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().take(n).collect()
  }

  /// Creates a new sequence without trailing elements based on a predicate.
  ///
  /// `take_while()` takes a closure as an argument. It will call this
  /// closure on each element of this sequence, and yield elements
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
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.take_while(|&x| x < 3), vec![1, 2]);
  /// ```
  #[inline]
  fn take_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().take_while(predicate).collect()
  }

  /// Creates a new sequence by omitting duplicate elements.
  ///
  /// Duplicates are detected using hash and equality.
  ///
  /// The algorithm is stable, returning the non-duplicate items in the order
  /// in which they occur in this sequence. In a set of duplicate
  /// items, the first item encountered is the item retained.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.unique(), vec![1, 2, 3]);
  /// ```
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
          let _unused = occurred.insert(item.clone());
          Some(item)
        } else {
          None
        }
      })
      .collect()
  }

  /// Creates a new sequence by omitting duplicate elements.
  ///
  /// Duplicates are detected by comparing the key they map to
  /// with the result of the keying function `to_key` using hash and equality.
  ///
  /// The algorithm is stable, returning the non-duplicate items in the order
  /// in which they occur in this sequence. In a set of duplicate
  /// items, the first item encountered is the item retained.
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 2, 3];
  ///
  /// assert_eq!(a.unique_by(|x| x % 2), vec![1, 2]);
  /// ```
  fn unique_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    K: Eq + Hash,
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
          let _unused = occurred.insert(key);
          true
        }
      })
      .collect()
  }

  /// Creates two new sequences by splitting this sequence of pairs.
  ///
  /// `unzip()` produces two sequences: one from the left elements of the pairs,
  /// and one from the right elements.
  ///
  /// This function is, in some sense, the opposite of [`zip()`].
  ///
  /// [`zip()`]: SequenceTo::zip
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![(1, 1), (2, 2), (3, 3)];
  ///
  /// let (left, right) = a.unzip();
  ///
  /// assert_eq!(left, vec![1, 2, 3]);
  /// assert_eq!(right, vec![1, 2, 3]);
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

  /// Creates a new sequence containing variations of specified size
  /// from the elements of this sequence.
  ///
  /// Specifying size is equal to the length of this sequence produces all permutations of this sequence.
  ///
  /// Variations are generated based on element positions, not values.
  /// Therefore, if this sequence contains duplicate elements, the resulting variations will too.
  /// To obtain variations of unique elements, use [`unique()`]`.variations()`.
  ///
  /// The order or variation values is preserved.
  ///
  /// [`unique()`]: SequenceTo::unique
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.variations(0), vec![vec![]]);
  /// assert_eq!(a.variations(1), vec![vec![1], vec![2], vec![3]]);
  /// assert_eq!(a.variations(2), vec![
  ///   vec![1, 2],
  ///   vec![1, 3],
  ///   vec![2, 1],
  ///   vec![2, 3],
  ///   vec![3, 1],
  ///   vec![3, 2]
  /// ]);
  /// // Permutations
  /// assert_eq!(a.variations(3), vec![
  ///   vec![1, 2, 3],
  ///   vec![1, 3, 2],
  ///   vec![2, 1, 3],
  ///   vec![2, 3, 1],
  ///   vec![3, 1, 2],
  ///   vec![3, 2, 1],
  /// ]);
  ///
  /// assert_eq!(e.variations(1), Vec::<Vec<i32>>::new());
  /// ```
  fn variations(&self, k: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: Sized;

  /// Creates a new sequence consisting of overlapping windows of `N` elements
  /// of this sequence, starting at the beginning of this sequence.
  ///
  /// The step parameter determines the distance between the first elements of
  /// successive windows.
  ///
  /// If `N` is greater than the size of this sequence, it will return no windows.
  ///
  /// This is the generalized equivalent of [`windows()`].
  ///
  /// # Panics
  ///
  /// Panics if `N` is 0. This check will most probably get changed to a compile time
  /// error before this method gets stabilized.
  ///
  /// [`windows()`]: slice::windows
  ///
  /// # Examples
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.windowed(2, 1), vec![vec![1, 2], vec![2, 3]]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.windowed(2, 2), vec![vec![1, 2]]);
  ///
  /// assert_eq!(e.windowed(1, 1), Vec::<Vec<i32>>::new());
  /// ```
  fn windowed(&self, size: usize, step: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>;

  /// Creates a new sequence consisting of overlapping windows of `N` elements
  /// of this sequence, starting at the beginning of this sequence and wrapping
  /// back to the first elements when the window would otherwise exceed this sequence length.
  ///
  /// The step parameter determines the distance between the first elements of
  /// successive windows.
  ///
  /// If `N` is greater than the size of this sequence, it will return no windows.
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
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  /// let e = Vec::<i32>::new();
  ///
  /// assert_eq!(a.windowed_circular(2, 1), vec![vec![1, 2], vec![2, 3], vec![3, 1]]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.windowed_circular(2, 2), vec![vec![1, 2], vec![3, 1]]);
  ///
  /// assert_eq!(e.windowed_circular(1, 1), Vec::<Vec<i32>>::new());
  /// ```
  fn windowed_circular(&self, size: usize, step: usize) -> Vec<Self>
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>;

  /// 'Zips up' this sequence with another collection into a single sequence of pairs.
  ///
  /// `zip()` returns a new sequence containing pairs where the first element comes from
  /// this sequence, and the second element comes from the other collection.
  ///
  /// If any of the sequences contains more elements than the other one, the remaining elements
  /// are omitted and the resulting sequence length is the length of the shorter sequence.
  ///
  /// To 'undo' the result of zipping up two sequences, see [`unzip()`].
  ///
  /// [`unzip()`]: SequenceTo::unzip
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.zip(vec![4, 5, 6]), vec![(1, 4), (2, 5), (3, 6)]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.zip(vec![4, 5, 6, 7]), vec![(1, 4), (2, 5), (3, 6)]);
  /// # let a = a_source.clone();
  /// assert_eq!(a.zip(vec![4, 5]), vec![(1, 4), (2, 5)]);
  /// ```
  #[inline]
  fn zip<T>(self, elements: impl IntoIterator<Item = T>) -> Self::This<(Item, T)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(Item, T)>: FromIterator<(Item, T)>,
  {
    self.into_iter().zip(elements).collect()
  }

  /// 'Zips up' this sequence with another collection into a single sequence of pairs.
  ///
  /// `zip()` returns a new sequence containing pairs where the first element comes from
  /// this sequence, and the second element comes from the other collection.
  ///
  /// If this sequence contains fewer elements than the other one, additional elements
  /// are created by calling the `to_left_value` closure and the resulting sequence length
  /// is the length of the other sequence.
  ///
  /// If this sequence contains more elements than the other one, additional elements
  /// are created by calling the `to_right_value` closure and the resulting sequence length
  /// is the length of this sequence.
  ///
  /// To 'undo' the result of zipping up two sequences, see [`unzip()`].
  ///
  /// [`unzip()`]: SequenceTo::unzip
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// # let a_source = vec![1, 2, 3];
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.zip_padded(vec![4, 5, 6], || 1, || 2), vec![(1, 4), (2, 5), (3, 6)],);
  /// # let a = a_source.clone();
  /// assert_eq!(a.zip_padded(vec![4, 5, 6, 7], || 1, || 2), vec![(1, 4), (2, 5), (3, 6), (1, 7)],);
  /// # let a = a_source.clone();
  /// assert_eq!(a.zip_padded(vec![4, 5], || 1, || 2), vec![(1, 4), (2, 5), (3, 2)],);
  /// ```
  #[inline]
  fn zip_padded<T>(
    self, elements: impl IntoIterator<Item = T>, mut to_left_value: impl FnMut() -> Item,
    mut to_right_value: impl FnMut() -> T,
  ) -> Self::This<(Item, T)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(Item, T)>: FromIterator<(Item, T)>,
  {
    let mut left_iterator = self.into_iter();
    let mut right_iterator = elements.into_iter();
    unfold(|| match (left_iterator.next(), right_iterator.next()) {
      (Some(left), Some(right)) => Some((left, right)),
      (Some(left), None) => Some((left, to_right_value())),
      (None, Some(right)) => Some((to_left_value(), right)),
      (None, None) => None,
    })
    .collect()
  }
}

pub(crate) fn cartesian_product<'a, Item: Clone + 'a, Collection: FromIterator<Item> + Sized>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  let size = values.len();
  let mut product = Vec::from_iter(iter::once(i64::MIN).chain(iter::repeat_n(0, k)));
  let mut current_slot = (size + 1).saturating_sub(k);
  unfold(|| {
    if current_slot == 0 {
      return None;
    }
    current_slot = k;
    let tuple = Some(collect_by_index(&values, &product[1..]));
    while product[current_slot] >= (size - 1) as i64 {
      current_slot -= 1;
    }
    product[current_slot] += 1;
    for index in &mut product[(current_slot + 1)..=k] {
      *index = 0;
    }
    tuple
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

pub(crate) fn chunked<Item, Collection>(collection: Collection, size: usize, exact: bool) -> Vec<Collection>
where
  Collection: FromIterator<Item> + IntoIterator<Item = Item>,
{
  assert_ne!(size, 0, "chunk size must be non-zero");
  let mut iterator = collection.into_iter();
  unfold(|| {
    let mut chunk_size = 0;
    let chunk = unfold(|| {
      if chunk_size < size
        && let Some(item) = iterator.next()
      {
        chunk_size += 1;
        return Some(item);
      }
      None
    })
    .collect();
    if chunk_size == size || (!exact && chunk_size > 0) { Some(chunk) } else { None }
  })
  .collect()
}

pub(crate) fn combinations_multi<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  let size = values.len();
  let mut multi_combination = Vec::from_iter(iter::once(i64::MIN).chain(iter::repeat_n(0, k)));
  let mut current_slot = (size + 1).saturating_sub(k);
  unfold(|| {
    if current_slot == 0 {
      return None;
    }
    current_slot = k;
    let tuple = Some(collect_by_index(&values, &multi_combination[1..]));
    while multi_combination[current_slot] >= (size - 1) as i64 {
      current_slot -= 1;
    }
    let new_index = multi_combination[current_slot] + 1;
    for index in &mut multi_combination[current_slot..=k] {
      *index = new_index;
    }
    tuple
  })
  .collect()
}

pub(crate) fn variations<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>, k: usize,
) -> Vec<Collection> {
  let values = Vec::from_iter(iterator);
  let size = values.len();
  let mut variation = Vec::from_iter(iter::once(i64::MIN).chain(0..(k as i64)));
  let mut used_indices = Vec::from_iter(iter::repeat_n(true, k).chain(iter::repeat_n(false, size.saturating_sub(k))));
  let mut current_slot = (size + 1).saturating_sub(k);
  unfold(|| {
    if current_slot == 0 {
      return None;
    }
    current_slot = k;
    let tuple = Some(collect_by_index(&values, &variation[1..]));
    while current_slot > 0 && ((variation[current_slot] + 1)..(size as i64)).all(|x| used_indices[x as usize]) {
      used_indices[variation[current_slot] as usize] = false;
      current_slot -= 1;
    }
    if current_slot > 0 {
      let initial_index = ((variation[current_slot] + 1)..(size as i64)).find(|x| !used_indices[*x as usize]).unwrap();
      used_indices[variation[current_slot] as usize] = false;
      used_indices[initial_index as usize] = true;
      variation[current_slot] = initial_index;
      for index in &mut variation[(current_slot + 1)..=k] {
        let new_index = (0..=(size as i64)).find(|x| !used_indices[*x as usize]).unwrap();
        used_indices[new_index as usize] = true;
        *index = new_index;
      }
    }
    tuple
  })
  .collect()
}

pub(crate) fn windowed<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  iterator: impl Iterator<Item = &'a Item>, size: usize, step: usize,
) -> Vec<Collection> {
  assert_ne!(size, 0, "window size must be non-zero");
  assert_ne!(step, 0, "step must be non-zero");
  let mut window = VecDeque::<Item>::with_capacity(size);
  iterator
    .flat_map(|item| {
      window.push_back(item.clone());
      if window.len() >= size {
        let tuple = Some(Collection::from_iter(window.clone()));
        for _ in 0..step {
          let _unused = window.pop_front();
        }
        tuple
      } else {
        None
      }
    })
    .collect()
}

pub(crate) fn windowed_circular<'a, Item: Clone + 'a, Collection: FromIterator<Item>>(
  mut iterator: impl Iterator<Item = &'a Item>, size: usize, step: usize,
) -> Vec<Collection> {
  assert_ne!(size, 0, "window size must be non-zero");
  assert_ne!(step, 0, "step must be non-zero");
  let mut window = VecDeque::<Item>::with_capacity(size);
  let mut init = VecDeque::<Item>::with_capacity(size - 1);
  unfold(|| {
    while window.len() < size {
      if let Some(item) = iterator.next() {
        window.push_back(item.clone());
        if init.len() < size - 1 {
          init.push_back(item.clone());
        }
      } else if let Some(item) = init.pop_front() {
        window.push_back(item);
      } else {
        return None;
      }
    }
    let tuple = Some(Collection::from_iter(window.clone()));
    for _ in 0..step {
      let _unused = window.pop_front();
    }
    tuple
  })
  .collect()
}
