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

  // FIXME - implement these methods
  // zip_all
  // unzip_all
  // coalesce
  // cartesian_product
  // same_elements
  // longest common prefix
  // longest common suffix
  // combinations_with_replacement
  // powerset
  // subset
  // subsequence / contains_slice / index_of_slice
  // group_map_fold
  // segment_range
  // move
  // coalesce
  // circular_windowed
  // slice

  #[inline]
  fn add_at(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.add_all_at(index, iter::once(element))
  }

  fn add_all_at(self, index: usize, elements: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut added = elements.into_iter();
    unfold(0_usize, |position| {
      let result = if *position >= index {
        added.next().or(iterator.next())
      } else {
        *position += 1;
        iterator.next()
      };
      result
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
  /// assert!(!a.all_unique());
  /// assert!(b.all_unique());
  /// ```
  fn all_unique(&self) -> bool
  where
    Item: Eq + Hash;

  fn chunked(self, size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    assert_ne!(size, 0, "chunk size must be non-zero");
    let mut result = Self::This::default();
    let mut chunk = Self::default();
    let mut index: usize = 0;
    for item in self.into_iter() {
      if index > 0 && index == size {
        result.extend(iter::once(chunk));
        chunk = Self::default();
        index = 0;
      }
      chunk.extend(iter::once(item));
      index += 1;
    }
    if index > 0 {
      result.extend(iter::once(chunk));
    }
    result
  }

  fn chunked_by(self, mut split_before: impl FnMut(&Item) -> bool) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    let mut result = Self::This::default();
    let mut chunk = Self::default();
    let mut index: usize = 0;
    for item in self.into_iter() {
      if index > 0 && split_before(&item) {
        result.extend(iter::once(chunk));
        chunk = Self::default();
        index = 0;
      }
      chunk.extend(iter::once(item));
      index += 1;
    }
    if index > 0 {
      result.extend(iter::once(chunk));
    }
    result
  }

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
    result.shrink_to_fit();
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
    result.shrink_to_fit();
    result
  }

  /// Creates a new collection from the original collection without
  /// the last element.
  ///
  /// # Examples
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
      let result = if !*inserted && *position % interval == 0 {
        *inserted = true;
        value.next()
      } else {
        *inserted = false;
        *position += 1;
        iterator.next()
      };
      result
    })
    .collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  #[inline]
  fn pad(self, size: usize, element: Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.pad_with(size, || element.clone())
  }

  #[inline]
  fn pad_with(self, size: usize, mut to_element: impl FnMut() -> Item) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    unfold(0_usize, |position| {
      iterator.next().or_else(|| {
        let result = if *position < size { Some(to_element()) } else { None };
        *position += 1;
        result
      })
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
  /// result or panics. If debug assertions are enabled, a panic is
  /// guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the collection has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Examples
  ///
  /// Basic usage:
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
  /// this closure to each element of the collection, and if one of them
  /// returns `true`, then `position()` add the element index to its result.
  ///
  /// # Overflow Behavior
  ///
  /// The method does no guarding against overflows, so if there are more
  /// than [`usize::MAX`] non-matching elements, it either produces the wrong
  /// result or panics. If debug assertions are enabled, a panic is
  /// guaranteed.
  ///
  /// # Panics
  ///
  /// This function might panic if the collection has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// # Examples
  ///
  /// Basic usage:
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
  /// # Examples
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
  fn rev(self) -> Self;

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
  /// # Examples
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
  /// let a = &[-1i32, 0, 1];
  ///
  /// assert_eq!(a.skip_while(|x| x.is_negative()), &[0, 1]);
  /// ```
  ///
  /// Because the closure passed to `skip_while()` takes a reference, and some
  /// collections contain references, this leads to a possibly confusing
  /// situation, where the type of the closure argument is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[&-1, &0, &1];
  ///
  /// assert_eq!(a.skip_while(|x| **x < 0), &[&0, &1]); // need two *s!
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
  /// # Examples
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
  /// let a = &[-1i32, 0, 1];
  ///
  /// assert_eq!(a.take_while(|x| x.is_negative()), &[-1]);
  /// ```
  ///
  /// Because the closure passed to `take_while()` takes a reference, and some
  /// collections contain references, this leads to a possibly confusing
  /// situation, where the type of the closure is a double reference:
  ///
  /// ```
  /// use cantrip::*;
  ///
  /// let a = &[&-1, &0, &1];
  ///
  /// assert_eq!(a.take_while(|x| **x < 0), &[&-1]); // need two *s!
  /// ```
  #[inline]
  fn take_while(self, predicate: impl FnMut(&Item) -> bool) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().take_while(predicate).collect()
  }

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
      let result = if index >= size - 1 {
        let window = Collection::from_iter(current.clone());
        current.pop_front();
        Some(window)
      } else {
        None
      };
      result
    })
    .collect()
}
