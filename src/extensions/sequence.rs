#![allow(missing_docs)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, LinkedList};
use std::hash::Hash;
use std::iter;
use std::ops::RangeBounds;

use crate::extensions::util::unfold::unfold;

/// Sequence operations.
///
/// Methods have the following properties:
///
/// - Requires the collection to represent an ordered sequence
/// - May consume the collection and its elements
/// - May create a new collection
///
pub trait Sequence<Item> {
  type This<I>;

  // FIXME - add documentation
  // FIXME - implement these methods
  // coalesce
  // chunked_by
  // index_of_sequence
  // longest common prefix
  // permutations
  // powersequence
  // slice
  // subsequence
  // variations
  // variations_repetitive
  // windowed
  // windowed_circular
  // zip_fill

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
  /// let mut a = vec![1, 2];
  ///
  /// assert_eq!(a.clone().add_at(1, 3), [1, 3, 2]);
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
  /// let mut a = vec![1, 2];
  ///
  /// assert_eq!(a.clone().add_all_at(1, vec![3, 4]), [1, 3, 4, 2]);
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
  /// let a = vec![1, 1, 1];
  /// let b = vec![1, 2, 3];
  ///
  /// assert!(!a.all_unique());
  /// assert!(b.all_unique());
  /// ```
  fn all_unique(&self) -> bool
  where
    Item: Eq + Hash;

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
  /// let a = vec![1, 2, -1, 1, 2];
  ///
  /// let chunked = a.chunked(2);
  /// assert_eq!(chunked, vec![vec![1, 2], vec![-1, 1], vec![2]])
  /// ```
  fn chunked(self, size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    chunked(self, size, false)
  }

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
  /// let a = vec![1, 2, -1, 1, 2];
  ///
  /// let chunked = a.chunked(2);
  /// assert_eq!(chunked, vec![vec![1, 2], vec![-1, 1], vec![2]])
  /// ```
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

  /// Searches for an element in a collection, returning its index.
  ///
  /// `position()` compares each element of the collection with the specified value,
  /// and if one of them matches, then `position()` returns [`Some(index)`].
  /// If none of the elements match, it returns [`None`].
  ///
  /// `position()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a matching element.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the collection has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.index_of(&2), Some(1));
  /// assert_eq!(a.index_of(&5), None);
  /// ```
  fn index_of(&self, value: &Item) -> Option<usize>
  where
    Item: PartialEq,
  {
    self.position(|x| x == value)
  }

  /// Searches for an element in a collection, returning all its indices.
  ///
  /// `position()` compares each element of the collection with the specified value,
  /// and each time one of them matches, then `position()` adds the element index
  /// to its result.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the collection has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 1];
  ///
  /// assert_eq!(a.indices_of(&1), vec![0, 2]);
  /// ```
  #[inline]
  fn indices_of(&self, element: &Item) -> Self::This<usize>
  where
    Item: PartialEq,
  {
    self.positions(|x| x == element)
  }

  /// Creates a new collection from the original collection without
  /// the last element.
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let mut a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.init(), vec![1, 2]);
  /// ```
  fn init(self) -> Self;

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
  /// let padded = a.pad(5, 4);
  /// assert_eq!(padded, vec![1, 2, 3, 4, 4]);
  /// ```
  #[inline]
  fn pad(self, size: usize, element: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.pad_with(size, |_| element.clone())
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
  /// let padded = a.pad_with(5, |i| 2 * i);
  /// assert_eq!(padded, vec![1, 2, 3, 6, 8]);
  /// ```
  fn pad_with(self, size: usize, mut to_element: impl FnMut(usize) -> Item) -> Self
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
  /// let mut a = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(a.clone().move_item(1, 3), vec![1, 3, 4, 2, 5]);
  /// assert_eq!(a.clone().move_item(1, 5), vec![1, 3, 4, 5]);
  /// assert_eq!(a.clone().move_item(3, 3), vec![1, 2, 3, 4, 5]);
  /// assert_eq!(a.clone().move_item(3, 1), vec![1, 4, 2, 3, 5]);
  /// assert_eq!(a.clone().move_item(5, 1), vec![1, 2, 3, 4, 5]);
  /// ```
  #[inline]
  fn move_item(self, source_index: usize, target_index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    assert_ne!(source_index, 0, "source index must be non-zero");
    assert_ne!(source_index, 0, "source index must be non-zero");
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

  /// Searches for an element in a collection, returning its index.
  ///
  /// `position()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, and if one of them
  /// returns `true`, then `position()` returns [`Some(index)`]. If all of
  /// them return `false`, it returns [`None`].
  ///
  /// `position()` is short-circuiting; in other words, it will stop
  /// processing as soon as it finds a `true`.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the collection has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.position(|&x| x == 2), Some(1));
  /// assert_eq!(a.position(|&x| x == 5), None);
  /// ```
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  /// Searches for an element in a collection, returning all its indices.
  ///
  /// `positions()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the collection, each time one of them
  /// returns `true`, then `position()` adds the element index to its result.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the collection has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// # Example
  ///
  /// ```
  /// use crate::cantrip::*;
  ///
  /// let a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.positions(|&x| x % 2 == 0), vec![1]);
  /// assert_eq!(a.positions(|&x| x % 2 == 1), vec![0, 2]);
  /// ```
  fn positions(&self, predicate: impl FnMut(&Item) -> bool) -> Self::This<usize>;

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
  fn rev<I>(self) -> Self where
    I: DoubleEndedIterator<Item = Item>,
    Self: IntoIterator<Item = Item, IntoIter = I> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    iterator.rev().collect()
  }

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
  ///
  /// assert_eq!(a.skip(2), vec![3]);
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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1i32, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|x| x.is_negative()), vec![0, 1]);
  /// ```
  ///
  /// Because the closure passed to `skip_while()` takes a reference, and some
  /// collections contain references, this leads to a possibly confusing
  /// situation, where the type of the closure argument is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|x| *x < 0), vec![0, 1]);
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
  /// let mut a = vec![1, 2, 3];
  ///
  /// assert_eq!(a.tail(), vec![2, 3]);
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
  ///
  /// assert_eq!(a.take(5), vec![1, 2]);
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
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1i32, 0, 1];
  ///
  /// assert_eq!(a.take_while(|x| x.is_negative()), vec![-1]);
  /// ```
  ///
  /// Because the closure passed to `take_while()` takes a reference, and some
  /// collections contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// # Example
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = vec![-1, 0, 1];
  ///
  /// assert_eq!(a.take_while(|x| *x < 0), vec![-1]);
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

  #[inline]
  fn zip<I: IntoIterator>(self, elements: I) -> Self::This<(Item, I::Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(Item, I::Item)>: FromIterator<(Item, I::Item)>,
  {
    self.into_iter().zip(elements).collect()
  }
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

#[inline]
pub(crate) fn init<Item, Iterable, Result>(iterator: Iterable) -> Result
where
  Iterable: Iterator<Item = Item> + ExactSizeIterator,
  Result: FromIterator<Item>,
{
  let size = iterator.len() - 1;
  iterator.take(size).collect()
}

#[inline]
pub(crate) fn positions<'a, Item, Result>(
  iterator: impl Iterator<Item = &'a Item>, mut predicate: impl FnMut(&Item) -> bool,
) -> Result
where
  Item: 'a,
  Result: FromIterator<usize>,
{
  iterator.enumerate().filter(|(_, item)| predicate(item)).map(|(index, _)| index).collect()
}

#[allow(unused_results)]
pub(crate) fn windowed<'a, Item, Collection, Result>(iterator: impl Iterator<Item = &'a Item>, size: usize) -> Result
where
  Item: Clone + 'a,
  Collection: FromIterator<Item>,
  Result: FromIterator<Collection>,
{
  assert_ne!(size, 0, "window size must be non-zero");
  let mut current: LinkedList<Item> = LinkedList::new();
  iterator
    .enumerate()
    .flat_map(|(index, item)| {
      current.push_back(item.clone());
      if index >= size - 1 {
        let window = Collection::from_iter(current.clone());
        current.pop_front();
        Some(window)
      } else {
        None
      }
    })
    .collect()
}
