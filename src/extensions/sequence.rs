use std::cmp::Ordering;
use std::collections::HashSet;
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

  #[inline]
  fn chunked(self, chunk_size: usize) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    let mut result = Self::This::default();
    let mut chunk = Self::default();
    let mut index: usize = 0;
    for item in self.into_iter() {
      if index > 0 && index == chunk_size {
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
  fn chunked_by(self, mut chunk_start: impl FnMut(&Item) -> bool) -> Self::This<Self>
  where
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
    Self::This<Self>: Default + Extend<Self>,
  {
    let mut result = Self::This::default();
    let mut chunk = Self::default();
    let mut index: usize = 0;
    for item in self.into_iter() {
      if index > 0 && chunk_start(&item) {
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

  // #[inline]
  // fn cycle(self, n: usize) -> Self
  // where
  //   Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  // {
  //   let mut iterator = self.into_iter();
  //   let mut value = iter::once(element);
  //   unfold(0_usize, |current| {
  //     if *current == index {
  //       *current += 1;
  //       value.next()
  //     } else {
  //       *current += 1;
  //       iterator.next()
  //     }
  //   })
  //   .collect()
  // }

  #[inline]
  fn delete(self, index: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().enumerate().filter_map(|(i, x)| if i != index { Some(x) } else { None }).collect()
  }

  #[inline]
  fn distinct(self) -> Self
  where
    Item: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + Default + Extend<Item>,
  {
    let mut result = Self::default();
    let mut occurred: HashSet<&Item> = HashSet::new();
    for item in self.into_iter() {
      if !occurred.contains(&item) {
        result.extend(iter::once(item));
      }
    }
    result
  }

  #[inline]
  fn distinct_by<K>(self, mut to_key: impl FnMut(&Item) -> K) -> Self
  where
    K: Eq + Hash,
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut occurred: HashSet<K> = HashSet::new();
    self
      .into_iter()
      .filter(|item| {
        let key = to_key(&item);
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
  fn enumerate(self) -> Self::This<(usize, Item)>
  where
    Self: IntoIterator<Item = Item> + Sized,
    Self::This<(usize, Item)>: FromIterator<(usize, Item)>,
  {
    self.into_iter().enumerate().collect()
  }

  fn init(self) -> Self;

  fn interleave(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + FromIterator<Item>,
  {
    self.into_iter().zip(iterable).map(|(item1, item2)| iter::once(item1).chain(iter::once(item2))).flatten().collect()
  }

  fn map_while<B>(&self, predicate: impl FnMut(&Item) -> Option<B>) -> Self::This<B>;

  #[inline]
  fn merge(self, iterable: impl IntoIterator<Item = Item>) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().chain(iterable.into_iter()).collect()
  }

  fn replace(self, range: impl RangeBounds<usize>, replace_with: Self) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut values = replace_with.into_iter();
    unfold(0_usize, |current| {
      if range.contains(current) {
        *current += 1;
        values.next()
      } else {
        *current += 1;
        iterator.next()
      }
    })
    .collect()
  }

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

  #[inline]
  fn put(self, index: usize, element: Item) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    let mut iterator = self.into_iter();
    let mut value = iter::once(element);
    unfold(0_usize, |current| {
      if *current == index {
        *current += 1;
        value.next()
      } else {
        *current += 1;
        iterator.next()
      }
    })
    .collect()
  }

  fn rev(self) -> Self;

  fn scan<S, B>(&self, init: S, function: impl FnMut(&mut S, &Item) -> Option<B>) -> Self::This<B>;

  #[inline]
  fn skip(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
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
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
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
  fn step_by(self, step: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().step_by(step).collect()
  }

  #[inline]
  fn tail(self) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().skip(1).collect()
  }

  #[inline]
  fn take(self, n: usize) -> Self
  where
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
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
    Self: IntoIterator<Item = Item> + Sized + FromIterator<Item>,
  {
    self.into_iter().take_while(predicate).collect()
  }

  #[inline]
  fn unit(value: Item) -> Self
  where
    Self: FromIterator<Item>,
  {
    iter::once(value).collect()
  }

  #[inline]
  fn unzip<B, C>(self) -> (Self::This<B>, Self::This<C>)
  where
    Self: IntoIterator<Item = (B, C)> + Sized,
    Self::This<B>: Default + Extend<B>,
    Self::This<C>: Default + Extend<C>,
  {
    self.into_iter().unzip()
  }

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
  Result: Sized + FromIterator<Item>,
{
  let size = iterator.len() - 1;
  iterator.skip(size).collect()
}
