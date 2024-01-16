use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::Display;
use std::fmt::Write;
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
  // sample
  // same_elements
  // rtake
  // rskip
  // longest common prefix
  // longest common suffix
  // combinations_with_replacement
  // powerset
  // subset
  // subsequence / contains_slice / index_of_slice
  // group_map_fold
  // pad_left
  // partition_at
  // partition_map
  // add_all_at
  // delete_all_at
  // scan
  // rscan
  // segment_range
  // replace
  // replace_at
  // replace_all_at
  // move
  // coalesce
  // circular_windowed
  // interleave_shortest
  // slice

  fn add_at(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut value = iter::once(element);
    unfold(0_usize, |position| {
      let result = if *position == index { value.next() } else { iterator.next() };
      *position += 1;
      result
    })
    .collect()
  }

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
    self.into_iter().enumerate().filter_map(|(i, x)| if i != index { Some(x) } else { None }).collect()
  }

  fn duplicates(self) -> Self
  where
    Item: Eq + Hash + Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let (size, _) = iterator.size_hint();
    let frequencies: HashMap<Item, usize> = HashMap::with_capacity(size);
    unfold(frequencies, |frequencies| {
      iterator.next().and_then(|item| {
        let count = frequencies.entry(item.clone()).or_default();
        *count += 1;
        if *count == 1 {
          Some(item)
        } else {
          None
        }
      })
    })
    .collect()
  }

  fn duplicates_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let (size, _) = iterator.size_hint();
    let frequencies: HashMap<K, usize> = HashMap::with_capacity(size);
    unfold(frequencies, |frequencies| {
      iterator.next().and_then(|item| {
        let count = frequencies.entry(to_key(&item)).or_default();
        *count += 1;
        if *count == 1 {
          Some(item)
        } else {
          None
        }
      })
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
    let (size, _) = iterator.size_hint();
    let mut result = HashMap::with_capacity(size);
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
    let (size, _) = iterator.size_hint();
    let mut result = HashMap::with_capacity(size);
    for item in iterator {
      *result.entry(to_key(item)).or_default() += 1;
    }
    result.shrink_to_fit();
    result
  }

  fn init(self) -> Self;

  // FIXME - add remaining elements to the end
  fn interleave(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().zip(iterable).flat_map(|(item1, item2)| iter::once(item1).chain(iter::once(item2))).collect()
  }

  #[inline]
  fn intersperse(self, element: Item, interval: usize) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.intersperse_with(|| element.clone(), interval)
  }

  fn intersperse_with(self, mut element: impl FnMut() -> Item, interval: usize) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    assert_ne!(interval, 0, "interval must be non-zero");
    let mut iterator = self.into_iter();
    let mut value = iter::repeat(element());
    unfold((0_usize, false), |(position, inserted)| {
      let result = if !*inserted && *position % interval == 0 {
        *inserted = true;
        value.next()
      } else {
        *inserted = false;
        iterator.next()
      };
      *position += 1;
      result
    })
    .collect()
  }

  fn join_items(&self, separator: &str) -> String
  where
    Item: Display;

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  // FIXME - do not cut the collection
  #[inline]
  fn pad(self, element: Item, size: usize) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().chain(iter::repeat(element)).take(size).collect()
  }

  #[inline]
  fn pad_with(self, mut element: impl FnMut() -> Item, size: usize) -> Self
  where
    Item: Clone,
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().chain(unfold((), |_| Some(element()))).take(size).collect()
  }

  // FIXME - implement
  // fn permutations(self) -> Self::This<Self>;

  /// Searches for an element in an iterator, returning its index.
  ///
  /// `position()` takes a closure that returns `true` or `false`. It applies
  /// this closure to each element of the iterator, and if one of them
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
  /// This function might panic if the iterator has more than `usize::MAX`
  /// non-matching elements.
  ///
  /// [`Some(index)`]: Some
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// let a = [1, 2, 3];
  ///
  /// assert_eq!(a.iter().position(|&x| x == 2), Some(1));
  ///
  /// assert_eq!(a.iter().position(|&x| x == 5), None);
  /// ```
  ///
  /// Stopping at the first `true`:
  ///
  /// ```
  /// let a = [1, 2, 3, 4];
  ///
  /// let mut iter = a.iter();
  ///
  /// assert_eq!(iter.position(|&x| x >= 2), Some(1));
  ///
  /// // we can still use `iter`, as there are more elements.
  /// assert_eq!(iter.next(), Some(&3));
  ///
  /// // The returned index depends on iterator state
  /// assert_eq!(iter.position(|&x| x == 4), Some(0));
  ///
  /// ```
  fn position(&self, predicate: impl FnMut(&Item) -> bool) -> Option<usize>;

  fn positions(&self, predicate: impl FnMut(&Item) -> bool) -> Self::This<usize>;

  fn rev(self) -> Self;

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>;

  #[inline]
  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().skip(n).collect()
  }

  /// Creates an collection without initial elements based on a predicate.
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

  fn splice(self, range: impl RangeBounds<usize>, replace_with: Self) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut values = replace_with.into_iter();
    unfold(0_usize, |position| {
      let result = if range.contains(position) { values.next() } else { iterator.next() };
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

  /// Creates an collection without trailing elements based on a predicate.
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
    let mut iterator = self.into_iter();
    let (size, _) = iterator.size_hint();
    let occurred = HashSet::with_capacity(size);
    unfold(occurred, |occurred| {
      iterator.next().and_then(|item| {
        if !occurred.contains(&item) {
          occurred.insert(item.clone());
          Some(item)
        } else {
          None
        }
      })
    })
    .collect()
  }

  fn unique_by<K: Eq + Hash>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    let iterator = self.into_iter();
    let (size, _) = iterator.size_hint();
    let mut occurred = HashSet::with_capacity(size);
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
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item>,
  {
    iter::once(value).collect()
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
  fn zip<I: IntoIterator>(self, iterable: I) -> Self::This<(Item, I::Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(Item, I::Item)>: FromIterator<(Item, I::Item)>,
  {
    self.into_iter().zip(iterable).collect()
  }
}

#[inline]
pub(crate) fn init<Item, Iterable, Result>(iterator: Iterable) -> Result
where
  Iterable: Iterator<Item = Item> + ExactSizeIterator,
  Result: FromIterator<Item>,
{
  let size = iterator.len() - 1;
  iterator.skip(size).collect()
}

pub(crate) fn join_items<'a, Item: Display + 'a>(
  mut iterator: impl Iterator<Item = &'a Item>, separator: &str,
) -> String {
  match iterator.next() {
    Some(item) => {
      let (size, _) = iterator.size_hint();
      let mut result = String::with_capacity(separator.len() * size);
      write!(&mut result, "{}", item).unwrap();
      for item in iterator {
        result.push_str(separator);
        write!(&mut result, "{}", item).unwrap();
      }
      result.shrink_to_fit();
      result
    }
    None => String::new(),
  }
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

pub(crate) fn windowed<'a, Item, Collection, Result>(
  mut iterator: impl Iterator<Item = &'a Item>, size: usize,
) -> Result
where
  Item: Clone + 'a,
  Collection: FromIterator<Item>,
  Result: FromIterator<Collection>,
{
  assert_ne!(size, 0, "window size must be non-zero");
  let current: LinkedList<Item> = LinkedList::new();
  unfold((0_usize, current), |(index, current)| {
    iterator.next().and_then(|item| {
      current.push_back(item.clone());
      let result = if *index >= size - 1 {
        let window = Collection::from_iter(current.clone());
        current.pop_front();
        Some(window)
      } else {
        None
      };
      *index += 1;
      result
    })
  })
  .collect()
}
